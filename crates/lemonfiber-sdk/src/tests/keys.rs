use std::time::{Duration, UNIX_EPOCH};

use serde_json::{json, Value};

use super::{client, enveloped, reply, Outcome, Scripted};
use crate::contract::{DeviceKeyAttestation, DeviceKeysBody, KeysBody};
use crate::{base64url, signed_text, Error, KeyLink};

const FINGERPRINT: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

fn minting() -> KeysBody {
    KeysBody {
        challenge: None,
        claim: Some(true),
        device: None,
        name: "home".to_owned(),
        password: Some("hunter2".to_owned()),
        purpose: "home-assistant".to_owned(),
        scope: "read".to_owned(),
        signature: None,
    }
}

fn link(parameters: &str) -> String {
    format!("lemonfiber://key?{parameters}")
}

fn whole() -> String {
    format!("address=https%3A%2F%2Fstack.example%3A8443&fingerprint={FINGERPRINT}&stack=stack-1&expires=4102444800&claim=c1")
}

#[test]
fn bytes_are_sent_in_base64url_unpadded() {
    for (raw, written) in [
        (&b""[..], ""),
        (b"f", "Zg"),
        (&[0x66, 0x6f], "Zm8"),
        (b"foo", "Zm9v"),
        (b"foob", "Zm9vYg"),
        (b"fooba", "Zm9vYmE"),
        (b"foobar", "Zm9vYmFy"),
        (&[0xfb, 0xff], "-_8"),
    ] {
        assert_eq!(base64url(raw), written);
    }
}

#[test]
fn a_device_key_signs_the_lines_of_one_mint_and_no_other() -> Outcome {
    let key = KeysBody {
        claim: None,
        ..minting()
    };
    assert_eq!(
        signed_text(&key, "ch")?,
        b"lemonfiber-mint/1\nch\nhome\nread\nhome-assistant\nkey\n".to_vec()
    );
    assert!(signed_text(&minting(), "ch")?.ends_with(b"\nclaim\n"));
    assert!(matches!(
        signed_text(&key, "c\nh"),
        Err(Error::Unsendable(_))
    ));
    Ok(())
}

#[tokio::test]
async fn a_key_is_minted_with_its_secret_or_as_a_claim_and_a_device_key_is_registered() -> Outcome {
    let minted = json!({
        "name": "home", "purpose": "home-assistant", "scope": "read", "secret": "lfk_x",
        "address": null, "pin": null,
    });
    let claimed = json!({ "lapses": "1", "link": link(&whole()), "name": "home", "purpose": "home-assistant", "scope": "read" });
    let registered = json!({ "app": "app", "backing": "strongbox", "fingerprint": "f", "name": "phone", "registered": "1" });
    let (client, client_script) = client(Scripted::answering(vec![
        Ok(reply(200, enveloped("minted-key", &minted))),
        Ok(reply(200, enveloped("key-claim", &claimed))),
        Ok(reply(200, enveloped("device-key", &registered))),
    ]))?;
    assert_eq!(client.mint(&minting()).await?.data.name, "home");
    assert_eq!(client.mint_claim(&minting()).await?.data.name, "home");
    let registering = DeviceKeysBody {
        attestation: Some(DeviceKeyAttestation::Android {
            chain: vec![base64url(b"der")],
        }),
        name: "phone".to_owned(),
        password: "hunter2".to_owned(),
        public_key: base64url(b"spki"),
    };
    assert_eq!(
        client.register_device_key(&registering).await?.data.name,
        "phone"
    );
    let bodies: Vec<Value> = client_script
        .sent()
        .into_iter()
        .filter_map(|request| request.body)
        .map(|body| serde_json::from_slice(&body))
        .collect::<Result<_, _>>()?;
    assert_eq!(bodies.first().and_then(|body| body.get("claim")), None);
    assert_eq!(
        bodies.get(1).and_then(|body| body.get("claim")),
        Some(&json!(true))
    );
    assert_eq!(
        bodies.get(2).and_then(|body| body.get("attestation")),
        Some(&json!({ "platform": "android", "chain": ["ZGVy"] }))
    );
    Ok(())
}

#[test]
fn a_key_link_is_read_as_a_claim_s_mint_writes_it() -> Outcome {
    let read = KeyLink::new(&format!("  {}  ", link(&whole())))?;
    assert_eq!(read.address().base(), "https://stack.example:8443");
    assert_eq!(
        read.address().pin().map(crate::CertificatePin::hex),
        Some(FINGERPRINT)
    );
    assert_eq!(read.stack(), "stack-1");
    assert_eq!(read.claim(), "c1");
    assert_eq!(
        read.lapses(),
        UNIX_EPOCH + Duration::from_secs(4_102_444_800)
    );
    assert!(!read.lapsed(UNIX_EPOCH) && read.lapsed(read.lapses()));
    assert_eq!(
        format!("{read:?}"),
        "KeyLink(https://stack.example:8443, stack=stack-1)"
    );
    Ok(())
}

#[test]
fn a_key_link_that_is_not_one_is_refused() {
    let unsendable = [
        "https://stack.example".to_owned(),
        "lemonfiber://key".to_owned(),
        link(&whole().replace("&claim=c1", "")),
        link(&format!("{}&claim=c2", whole())),
        link(&whole().replace("claim=c1", "claim=")),
        link(&whole().replace("stack=stack-1", "stack=")),
        link(&whole().replace("expires=4102444800", "expires=soon")),
        link(&whole().replace("claim=c1", "other=c1")),
    ];
    for refused in unsendable {
        assert!(
            matches!(KeyLink::new(&refused), Err(Error::Unsendable(_))),
            "{refused}"
        );
    }
    assert!(matches!(
        KeyLink::new(&link(&whole().replace(FINGERPRINT, "abc"))),
        Err(Error::AddressRefused(_))
    ));
    assert!(matches!(
        KeyLink::new(&link(&whole().replace("https%3A", "http%3A"))),
        Err(Error::AddressRefused(_))
    ));
}
