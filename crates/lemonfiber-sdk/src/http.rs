use std::error::Error as _;
use std::net::SocketAddr;
use std::sync::Arc;

use futures_util::StreamExt;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{CertificateError, DigitallySignedStruct, SignatureScheme};

use crate::transport::{Failure, Method, Opened, Request, Response, Transport};
use crate::{Address, CertificatePin, Error};

const SET_UP: &str = "The connection to the stack could not be set up.";

/// The transport this crate reaches a stack through: `reqwest` over `rustls` with the
/// `ring` provider. A pinned address is reached over https alone and accepts only the
/// certificate it is pinned to, checked during the handshake; any other address is
/// reached at the loopback addresses it was found at. It follows no redirect and takes no
/// proxy from the environment.
#[derive(Debug, Clone)]
pub struct Http {
    client: reqwest::Client,
}

#[derive(Debug)]
struct Pinned {
    pin: CertificatePin,
    provider: Arc<CryptoProvider>,
}

impl ServerCertVerifier for Pinned {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        if CertificatePin::of(end_entity.as_ref()) == self.pin {
            Ok(ServerCertVerified::assertion())
        } else {
            Err(rustls::Error::InvalidCertificate(
                CertificateError::ApplicationVerificationFailure,
            ))
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

impl Http {
    /// The transport reaching `address`, held to its pin where it has one.
    ///
    /// # Errors
    ///
    /// [`Error::Unsendable`] where TLS could not be set up.
    pub fn for_address(address: &Address) -> Result<Self, Error> {
        configured(address)
            .map(|client| Self { client })
            .ok_or(Error::Unsendable(SET_UP.to_owned()))
    }

    async fn sent(&self, request: Request) -> Result<reqwest::Response, Failure> {
        let method = match request.method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Delete => reqwest::Method::DELETE,
        };
        let mut building = self.client.request(method, &request.url);
        for (name, value) in &request.headers {
            building = building.header(*name, value);
        }
        if let Some(body) = request.body {
            building = building.body(body);
        }
        building.send().await.map_err(|error| failed(&error))
    }
}

fn configured(address: &Address) -> Option<reqwest::Client> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let versions = rustls::ClientConfig::builder_with_provider(Arc::clone(&provider))
        .with_safe_default_protocol_versions()
        .ok()?;
    let config = match address.pin() {
        Some(pin) => versions
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(Pinned {
                pin: pin.clone(),
                provider,
            }))
            .with_no_client_auth(),
        None => versions
            .with_root_certificates(
                webpki_roots::TLS_SERVER_ROOTS
                    .iter()
                    .cloned()
                    .collect::<rustls::RootCertStore>(),
            )
            .with_no_client_auth(),
    };
    let reached: Vec<SocketAddr> = address
        .reached()
        .iter()
        .map(|ip| SocketAddr::new(*ip, address.port()))
        .collect();
    let mut builder = reqwest::Client::builder()
        .tls_backend_preconfigured(config)
        .https_only(address.pin().is_some())
        .redirect(reqwest::redirect::Policy::none())
        .no_proxy();
    if !reached.is_empty() {
        builder = builder.resolve_to_addrs(address.host(), &reached);
    }
    builder.build().ok()
}

fn failed(error: &reqwest::Error) -> Failure {
    let mut waiting: Vec<&(dyn std::error::Error + 'static)> = error.source().into_iter().collect();
    while let Some(at) = waiting.pop() {
        if matches!(
            at.downcast_ref::<rustls::Error>(),
            Some(rustls::Error::InvalidCertificate(_))
        ) {
            return Failure::CertificateRefused;
        }
        waiting.extend(at.source());
        waiting.extend(
            at.downcast_ref::<std::io::Error>()
                .and_then(std::io::Error::get_ref)
                .map(|inner| inner as &(dyn std::error::Error + 'static)),
        );
    }
    Failure::Unreachable(error.to_string())
}

fn headers_of(response: &reqwest::Response) -> Vec<(String, String)> {
    response
        .headers()
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_owned(),
                String::from_utf8_lossy(value.as_bytes()).into_owned(),
            )
        })
        .collect()
}

impl Transport for Http {
    async fn send(&self, request: Request) -> Result<Response, Failure> {
        let most = request.most;
        let mut response = self.sent(request).await?;
        let status = response.status().as_u16();
        let headers = headers_of(&response);
        if most.is_some_and(|most| {
            response
                .content_length()
                .is_some_and(|length| length > most as u64)
        }) {
            return Err(Failure::TooLarge);
        }
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|error| failed(&error))? {
            body.extend_from_slice(&chunk);
            if most.is_some_and(|most| body.len() > most) {
                return Err(Failure::TooLarge);
            }
        }
        Ok(Response {
            status,
            headers,
            body,
        })
    }

    async fn open(&self, request: Request) -> Result<Opened, Failure> {
        let response = self.sent(request).await?;
        let status = response.status().as_u16();
        let headers = headers_of(&response);
        let body = response.bytes_stream().map(|chunk| {
            chunk
                .map(|bytes| bytes.to_vec())
                .map_err(|error| failed(&error))
        });
        Ok(Opened {
            status,
            headers,
            body: Box::pin(body),
        })
    }
}
