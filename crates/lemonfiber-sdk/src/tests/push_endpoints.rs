use std::path::Path;

use serde_json::{json, Value};

use super::{client, reply, Outcome, Scripted};
use crate::contract::PushEndpointsBody;
use crate::transport::Method;
use crate::{base64url, Error};

const ADDRESS: &str = "https://push.example/send/a-phone";

fn example(kind: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../contract/web-api/examples/kinds")
            .join(format!("{kind}.json")),
    )
}

#[tokio::test]
async fn push_endpoints_are_registered_listed_and_removed_as_the_contract_examples_say() -> Outcome
{
    let (client, script) = client(Scripted::answering(vec![
        Ok(reply(200, example("push-endpoint")?)),
        Ok(reply(200, example("push-endpoints")?)),
        Ok(reply(200, example("push-endpoints")?)),
    ]))?;
    let registering = PushEndpointsBody {
        name: "a-phone".to_owned(),
        endpoint: ADDRESS.to_owned(),
        p256dh: base64url(b"\x04\xfb\xff"),
        auth: base64url(b"\x01"),
    };
    assert_eq!(
        client.register_push_endpoint(&registering).await?.data.name,
        "a-phone"
    );
    let listed = client.push_endpoints().await?;
    assert_eq!(
        listed
            .data
            .endpoints
            .first()
            .map(|one| (one.host.as_str(), one.fingerprint.as_str())),
        Some(("push.example", "2fb15b007051cfa9"))
    );
    client.remove_push_endpoint("a phone/2").await?;
    let sent = script.sent();
    let asked: Vec<(Method, &str)> = sent
        .iter()
        .map(|request| (request.method, request.url.as_str()))
        .collect();
    assert_eq!(
        asked,
        vec![
            (Method::Post, "http://127.0.0.1:9000/api/push-endpoints"),
            (Method::Get, "http://127.0.0.1:9000/api/push-endpoints"),
            (
                Method::Delete,
                "http://127.0.0.1:9000/api/push-endpoints/a%20phone%2F2"
            ),
        ]
    );
    let body: Value = serde_json::from_slice(
        sent.first()
            .and_then(|request| request.body.as_deref())
            .unwrap_or_default(),
    )?;
    assert_eq!(
        body,
        json!({ "name": "a-phone", "endpoint": ADDRESS, "p256dh": "BPv_", "auth": "AQ" })
    );
    Ok(())
}

#[tokio::test]
async fn a_push_endpoint_name_no_path_can_carry_is_refused_before_sending() -> Outcome {
    let (client, script) = client(Scripted::answering(vec![]))?;
    assert!(matches!(
        client.remove_push_endpoint("..").await,
        Err(Error::Unsendable(_))
    ));
    assert!(script.sent().is_empty());
    Ok(())
}

#[tokio::test]
async fn a_push_endpoint_listing_answered_as_another_kind_is_refused() -> Outcome {
    let (client, _script) = client(Scripted::answering(vec![Ok(reply(
        200,
        example("push-endpoint")?,
    ))]))?;
    assert!(client.push_endpoints().await.is_err());
    Ok(())
}
