use crate::contract::{
    PushEndpointListing, PushEndpointsBody, PushEndpointsNameBody, RegisteredPushEndpoint,
};
use crate::line::Asked;
use crate::query::segment;
use crate::transport::Method;
use crate::{Client, Envelope, Error};

const PUSH_ENDPOINTS: &str = "/api/push-endpoints";

/// The phones that asked to be woken: every one for the operator, a member's own for a
/// member's session, and none for a key. An endpoint's address lets whoever holds it wake
/// the phone, so no answer carries it once it is registered: each carries its host and a
/// fingerprint instead.
impl Client {
    /// Every push endpoint this credential keeps, in the order they were registered.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn push_endpoints(&self) -> Result<Envelope<PushEndpointListing>, Error> {
        self.expect(Asked::read(PUSH_ENDPOINTS.to_owned()), "push-endpoints")
            .await
    }

    /// Registers a phone's push endpoint under a name, with its `https` address and its
    /// `WebPush` key and secret in base64url. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn register_push_endpoint(
        &self,
        registering: &PushEndpointsBody,
    ) -> Result<Envelope<RegisteredPushEndpoint>, Error> {
        self.expect(
            Asked::sending(Method::Post, PUSH_ENDPOINTS.to_owned(), registering)?,
            "push-endpoint",
        )
        .await
    }

    /// Removes one push endpoint by its name, answered with the endpoints as they now stand,
    /// naming the one removed. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet, and [`Error::Unsendable`] for a name no path can
    /// carry as one part.
    pub async fn remove_push_endpoint(
        &self,
        name: &str,
    ) -> Result<Envelope<PushEndpointListing>, Error> {
        self.remove_push_endpoint_with(name, &PushEndpointsNameBody { dry_run: None })
            .await
    }

    /// What removing one push endpoint by its name would come to, removing nothing: the
    /// endpoints as they would then stand, the listing saying it was a rehearsal.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet, and [`Error::Unsendable`] for a name no path can
    /// carry as one part.
    pub async fn rehearse_remove_push_endpoint(
        &self,
        name: &str,
    ) -> Result<Envelope<PushEndpointListing>, Error> {
        self.remove_push_endpoint_with(
            name,
            &PushEndpointsNameBody {
                dry_run: Some(true),
            },
        )
        .await
    }

    /// Removes one push endpoint by its name, or only rehearses removing it where `removing`
    /// sets `dry_run` true: answered with the endpoints as they then stand, the listing
    /// saying whether it was a rehearsal. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet, and [`Error::Unsendable`] for a name no path can
    /// carry as one part.
    pub async fn remove_push_endpoint_with(
        &self,
        name: &str,
        removing: &PushEndpointsNameBody,
    ) -> Result<Envelope<PushEndpointListing>, Error> {
        let path = format!("{PUSH_ENDPOINTS}/{}", segment(name)?);
        self.expect(
            Asked::removing(path, removing.dry_run, removing)?,
            "push-endpoints",
        )
        .await
    }
}
