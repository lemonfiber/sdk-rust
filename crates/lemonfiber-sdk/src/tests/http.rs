use std::net::SocketAddr;
use std::sync::Arc;

use futures_util::StreamExt;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::SupportedProtocolVersion;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use super::{enveloped, Outcome};
use crate::transport::{Failure, Method, Request, Transport};
use crate::{Address, CertificatePin, Door, Http, KeyLink, Resolve};

async fn answer_one<S: AsyncRead + AsyncWrite + Unpin>(
    mut stream: S,
    response: &[u8],
) -> std::io::Result<()> {
    let mut read = Vec::new();
    let mut buffer = [0u8; 4096];
    let mut count = 1;
    while count > 0 && !read.windows(4).any(|window| window == b"\r\n\r\n") {
        count = stream.read(&mut buffer).await?;
        read.extend_from_slice(buffer.get(..count).unwrap_or_default());
    }
    stream.write_all(response).await?;
    stream.shutdown().await
}

async fn listening() -> std::io::Result<(TcpListener, SocketAddr)> {
    let listener = TcpListener::bind(("127.0.0.1", 0)).await?;
    let at = listener.local_addr()?;
    Ok((listener, at))
}

async fn plain(responses: Vec<Vec<u8>>) -> std::io::Result<SocketAddr> {
    let (listener, at) = listening().await?;
    tokio::spawn(async move {
        for response in responses {
            if let Ok((tcp, _)) = listener.accept().await {
                let _ = answer_one(tcp, &response).await;
            }
        }
    });
    Ok(at)
}

async fn tls(
    versions: &'static [&'static SupportedProtocolVersion],
    response: Vec<u8>,
) -> Result<(SocketAddr, CertificatePin), Box<dyn std::error::Error>> {
    let made = rcgen::generate_simple_self_signed(vec!["127.0.0.1".to_owned()])?;
    let der: CertificateDer<'static> = made.cert.der().clone();
    let key = PrivateKeyDer::try_from(made.signing_key.serialize_der())?;
    let config = rustls::ServerConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_protocol_versions(versions)?
    .with_no_client_auth()
    .with_single_cert(vec![der.clone()], key)?;
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let (listener, at) = listening().await?;
    tokio::spawn(async move {
        while let Ok((tcp, _)) = listener.accept().await {
            if let Ok(stream) = acceptor.accept(tcp).await {
                let _ = answer_one(stream, &response).await;
            }
        }
    });
    Ok((at, CertificatePin::of(der.as_ref())))
}

const TLS12_ONLY: &[&SupportedProtocolVersion] = &[&rustls::version::TLS12];

fn ok(body: &[u8]) -> Vec<u8> {
    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    response.extend_from_slice(body);
    response
}

fn get(url: String, most: Option<usize>) -> Request {
    Request {
        method: Method::Get,
        url,
        headers: vec![("Accept", "application/json".to_owned())],
        body: None,
        most,
    }
}

#[tokio::test]
async fn a_loopback_stack_is_reached_and_an_answer_is_held_to_its_size() -> Outcome {
    let unsized_answer = b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n0123456789".to_vec();
    let at = plain(vec![
        ok(b"{}"),
        ok(b"0123456789"),
        unsized_answer,
        ok(b"{}"),
    ])
    .await?;
    let address = Address::new(&format!("http://{at}"), None)?;
    let http = Http::for_address(&address)?;
    let answered = http.send(get(address.url("/api/version"), None)).await?;
    assert_eq!(
        (answered.status, answered.body.as_slice()),
        (200, &b"{}"[..])
    );
    assert_eq!(answered.header("content-type"), Some("application/json"));
    assert_eq!(
        http.send(get(address.url("/"), Some(4))).await,
        Err(Failure::TooLarge)
    );
    assert_eq!(
        http.send(get(address.url("/"), Some(4))).await,
        Err(Failure::TooLarge)
    );
    let posted = Request {
        method: Method::Post,
        body: Some(b"{}".to_vec()),
        ..get(address.url("/api/actions/restart"), None)
    };
    assert_eq!(http.send(posted).await?.status, 200);
    Ok(())
}

#[tokio::test]
async fn a_stream_is_handed_back_as_it_arrives() -> Outcome {
    let at = plain(vec![
        b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\ndata: x\n\n".to_vec(),
    ])
    .await?;
    let address = Address::new(&format!("http://{at}"), None)?;
    let mut opened = Http::for_address(&address)?
        .open(Request {
            method: Method::Delete,
            ..get(address.url("/api/events"), None)
        })
        .await?;
    assert_eq!(opened.status, 200);
    let mut said = Vec::new();
    while let Some(chunk) = opened.body.next().await {
        said.extend(chunk?);
    }
    assert_eq!(said, b"data: x\n\n");
    assert!(format!("{opened:?}").starts_with("Opened(200"));
    Ok(())
}

#[tokio::test]
async fn nothing_listening_is_unreachable() -> Outcome {
    let (listener, at) = listening().await?;
    drop(listener);
    let address = Address::new(&format!("http://{at}"), None)?;
    let http = Http::for_address(&address)?;
    assert!(matches!(
        http.send(get(address.url("/"), None)).await,
        Err(Failure::Unreachable(_))
    ));
    assert!(matches!(
        http.open(get(address.url("/"), None)).await,
        Err(Failure::Unreachable(_))
    ));
    Ok(())
}

struct Loopback;

impl Resolve for Loopback {
    fn resolve(&self, _host: &str) -> Vec<std::net::IpAddr> {
        vec![std::net::IpAddr::from([127, 0, 0, 1])]
    }
}

#[tokio::test]
async fn a_name_is_reached_at_the_loopback_addresses_it_was_found_at() -> Outcome {
    let at = plain(vec![ok(b"{}")]).await?;
    let address = Address::resolving(
        &format!("http://stack.invalid:{}", at.port()),
        None,
        &Loopback,
    )?;
    let answered = Http::for_address(&address)?
        .send(get(address.url("/"), None))
        .await?;
    assert_eq!(answered.status, 200);
    Ok(())
}

#[tokio::test]
async fn a_pinned_stack_is_reached_only_where_it_presents_the_pinned_certificate() -> Outcome {
    let (at, pin) = tls(rustls::DEFAULT_VERSIONS, ok(b"{}")).await?;
    let (at12, pin12) = tls(TLS12_ONLY, ok(b"{}")).await?;
    let base = format!("https://{at}");
    let pinned = Address::new(&base, Some(pin.clone()))?;
    assert_eq!(
        Http::for_address(&pinned)?
            .send(get(pinned.url("/"), None))
            .await?
            .status,
        200
    );
    let pinned12 = Address::new(&format!("https://{at12}"), Some(pin12))?;
    assert_eq!(
        Http::for_address(&pinned12)?
            .send(get(pinned12.url("/"), None))
            .await?
            .status,
        200
    );
    let other = Address::new(&base, Some(CertificatePin::new(&"0".repeat(64))?))?;
    assert_eq!(
        Http::for_address(&other)?
            .send(get(other.url("/"), None))
            .await,
        Err(Failure::CertificateRefused)
    );
    let unpinned = Address::new(&base, None)?;
    assert_eq!(
        Http::for_address(&unpinned)?
            .send(get(unpinned.url("/"), None))
            .await,
        Err(Failure::CertificateRefused)
    );
    Ok(())
}

#[tokio::test]
async fn a_key_link_collects_its_key_over_the_tls_its_fingerprint_pins() -> Outcome {
    let minted =
        serde_json::json!({ "name": "home", "purpose": "mcp", "scope": "read", "secret": "lfk_x" });
    let (at, pin) = tls(
        rustls::DEFAULT_VERSIONS,
        ok(&enveloped("minted-key", &minted)),
    )
    .await?;
    let link = KeyLink::new(&format!(
        "lemonfiber://key?address=https://{at}&fingerprint={}&stack=s&expires=4102444800&claim=c",
        pin.hex()
    ))?;
    assert_eq!(Door::collect_link(&link).await?.data.secret, "lfk_x");
    Ok(())
}

#[tokio::test]
async fn an_answer_cut_short_is_unreachable_and_a_stream_cut_short_is_a_gap() -> Outcome {
    let cut = b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\nshort".to_vec();
    let at = plain(vec![cut.clone(), cut]).await?;
    let address = Address::new(&format!("http://{at}"), None)?;
    let http = Http::for_address(&address)?;
    assert!(matches!(
        http.send(get(address.url("/"), None)).await,
        Err(Failure::Unreachable(_))
    ));
    let client = crate::Client::new(address, crate::Credential::new("token")?)?;
    let mut stream = client.events(crate::Following::default());
    assert!(matches!(
        stream.next().await?,
        crate::Arrival::Gap {
            why: crate::Break::Dropped,
            ..
        }
    ));
    Ok(())
}
