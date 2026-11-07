use std::time::Duration;

use serde_json::json;

use super::{enveloped, loopback, problem, reply, Outcome, Scripted};
use crate::contract::{KeysClaimBody, SessionBody};
use crate::{Address, CertificatePin, Door, Error, KeyLink, RefusedBecause};

fn sign_in() -> SessionBody {
    SessionBody {
        claim: None,
        name: Some("ana".to_owned()),
        password: "hunter2".to_owned(),
    }
}

#[tokio::test]
async fn a_password_is_exchanged_once_for_a_session() -> Outcome {
    let door_script = Scripted::answering(vec![
        Ok(reply(
            200,
            enveloped(
                "admission",
                &json!({ "token": "s3ss10n", "until": "1700000000", "member": "ana" }),
            ),
        )),
        Ok(reply(401, Vec::new())),
        Ok(reply(401, problem("ADMIT-8", "That is not it."))),
        Ok(reply(429, problem("ADMIT-5", "Wait."))),
        Ok(reply(
            200,
            enveloped("admission", &json!({ "token": "has space", "until": "1" })),
        )),
    ]);
    let door =
        Door::with_transport(loopback()?, door_script.clone())?.waiting(Duration::from_secs(5));
    let session = door.admit(&sign_in()).await?;
    assert_eq!(session.member.as_deref(), Some("ana"));
    assert_eq!(session.until, "1700000000");
    assert_eq!(format!("{:?}", session.credential), "Credential(hidden)");
    let refused = |error: Result<crate::Session, Error>| match error {
        Err(Error::Refused(refusal)) => Some((refusal.because, refusal.sentence)),
        _ => None,
    };
    assert_eq!(
        refused(door.admit(&sign_in()).await),
        Some((
            RefusedBecause::PasswordRefused,
            "That is not the password, or none is configured.".to_owned()
        ))
    );
    assert_eq!(
        refused(door.admit(&sign_in()).await),
        Some((
            RefusedBecause::PasswordRefused,
            "That is not it.".to_owned()
        ))
    );
    assert_eq!(
        refused(door.admit(&sign_in()).await).map(|(because, _)| because),
        Some(RefusedBecause::TooManyAttempts)
    );
    assert!(matches!(
        door.admit(&sign_in()).await,
        Err(Error::Unsendable(_))
    ));
    let sent = door_script.sent();
    let first = sent.first().ok_or("nothing was sent")?;
    assert!(first
        .headers
        .iter()
        .all(|(name, _)| *name != "X-Lemonfiber-Token"));
    assert_eq!(first.url, "http://127.0.0.1:9000/api/session");
    Ok(())
}

#[tokio::test]
async fn a_claim_collects_its_key_once_with_the_claim_alone() -> Outcome {
    let minted = json!({ "name": "home", "purpose": "mcp", "scope": "read", "secret": "lfk_x" });
    let door = Door::with_transport(
        loopback()?,
        Scripted::answering(vec![
            Ok(reply(200, enveloped("minted-key", &minted))),
            Ok(reply(401, Vec::new())),
        ]),
    )?;
    let claim = KeysClaimBody {
        claim: "c1".to_owned(),
    };
    assert_eq!(door.collect(&claim).await?.data.secret, "lfk_x");
    assert!(matches!(
        door.collect(&claim).await,
        Err(Error::Refused(refusal)) if refusal.because == RefusedBecause::ClaimRefused && refusal.sentence.contains("collects no key")
    ));
    Ok(())
}

#[tokio::test]
async fn a_lapsed_claim_is_not_sent_and_a_pinned_door_takes_no_other_transport() -> Outcome {
    let lapsed = KeyLink::new(&format!(
        "lemonfiber://key?address=https://stack.example&fingerprint={}&stack=s&expires=1&claim=c",
        "a".repeat(64)
    ))?;
    assert_eq!(Door::collect_link(&lapsed).await, Err(Error::ClaimLapsed));
    let pinned = Address::new(
        "https://stack.example",
        Some(CertificatePin::new(&"a".repeat(64))?),
    )?;
    assert!(matches!(
        Door::with_transport(pinned.clone(), Scripted::default()),
        Err(Error::AddressRefused(_))
    ));
    assert!(Door::new(pinned).is_ok());
    Ok(())
}
