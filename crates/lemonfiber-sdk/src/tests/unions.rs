use serde_json::{json, Value};

use super::Outcome;
use crate::contract::{Based, RunningCopy, Vouch};

pub(super) fn kept<T: serde::de::DeserializeOwned + serde::Serialize>(
    said: &Value,
) -> Result<(T, Value), serde_json::Error> {
    let read: T = serde_json::from_value(said.clone())?;
    let written = serde_json::to_value(&read)?;
    Ok((read, written))
}

#[test]
fn the_running_copy_carries_its_fields_in_every_state() -> Outcome {
    let beside = json!({
        "what": "backup", "scope": { "scope": "whole_stack" }, "started": 1,
        "written": 2, "whole": 3, "part": "sonarr configuration"
    });
    for (state, own) in [
        ("working", json!({})),
        ("paced", json!({ "limit": 5 })),
        ("stalled", json!({ "wrote": 4 })),
    ] {
        let mut said = beside.clone();
        if let (Some(fields), Some(more)) = (said.as_object_mut(), own.as_object()) {
            fields.insert("state".to_owned(), json!(state));
            fields.extend(more.clone());
        }
        let (read, written) = kept::<RunningCopy>(&said)?;
        assert!(
            matches!(
                read,
                RunningCopy::Working { .. }
                    | RunningCopy::Paced { .. }
                    | RunningCopy::Stalled { .. }
            ),
            "{state}: {read:?}"
        );
        assert_eq!(written, said, "{state}");
    }
    Ok(())
}

#[test]
fn a_vouch_and_what_an_image_is_built_on_carry_their_fields_in_every_branch() -> Outcome {
    for said in [
        json!({ "digest": "sha256:a", "image": "ghcr.io/x", "service": "sonarr", "provenance": "signed", "by": "k" }),
        json!({ "digest": "sha256:a", "image": "ghcr.io/x", "service": "sonarr", "provenance": "unproven", "why": "w" }),
        json!({ "digest": "sha256:a", "image": "ghcr.io/x", "service": "sonarr", "provenance": "refused", "why": "w" }),
    ] {
        let (read, written) = kept::<Vouch>(&said)?;
        assert!(
            matches!(
                read,
                Vouch::Signed { .. } | Vouch::Unproven { .. } | Vouch::Refused { .. }
            ),
            "{read:?}"
        );
        assert_eq!(written, said);
    }
    for said in [
        json!({ "image": "alpine", "release": "3.20", "basis": "recorded", "digests": ["sha256:b"] }),
        json!({ "image": "alpine", "release": "3.20", "basis": "unproven", "why": "w" }),
        json!({ "image": "alpine", "release": "3.20", "basis": "refused", "why": "w" }),
    ] {
        let (read, written) = kept::<Based>(&said)?;
        assert!(
            matches!(
                read,
                Based::Recorded { .. } | Based::Unproven { .. } | Based::Refused { .. }
            ),
            "{read:?}"
        );
        assert_eq!(written, said);
    }
    Ok(())
}
