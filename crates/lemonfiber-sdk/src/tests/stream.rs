use std::time::Duration;

use serde_json::json;

use super::{client, enveloped, problem, stream_of, Opening, Outcome, Scripted};
use crate::transport::Failure;
use crate::{Arrival, Break, Error, Following};

fn event(id: &str, kind: &str, data: &serde_json::Value) -> String {
    format!(
        "id: {id}\nevent: {kind}\ndata: {}\n\n",
        String::from_utf8_lossy(&enveloped(kind, data))
    )
}

fn pulled(id: &str, line: &str) -> String {
    event(id, "pull", &json!(line))
}

fn quick() -> Following {
    Following {
        first_pause: Duration::from_millis(10),
        ..Following::default()
    }
}

#[tokio::test(start_paused = true)]
async fn a_broken_stream_says_so_cools_what_it_held_and_resumes_from_the_last_event() -> Outcome {
    let (client, client_script) = client(Scripted::opening(vec![
        Ok(stream_of(200, &[": beat\n\n", &pulled("7", "one")], false)),
        Ok(stream_of(200, &[&pulled("8", "two")], true)),
    ]))?;
    let mut stream = client.events(quick());
    assert!(matches!(stream.next().await?, Arrival::Live(envelope) if envelope.kind == "pull"));
    assert!(matches!(
        stream.next().await?,
        Arrival::Gap {
            why: Break::Ended,
            ..
        }
    ));
    assert!(
        matches!(stream.next().await?, Arrival::Stale { envelope, .. } if envelope.kind == "pull")
    );
    assert!(matches!(
        stream.held().get("pull"),
        Some(Arrival::Stale { .. })
    ));
    assert!(matches!(stream.next().await?, Arrival::Live(_)));
    assert!(matches!(stream.held().get("pull"), Some(Arrival::Live(_))));
    assert!(
        matches!(stream.next().await?, Arrival::Gap { why: Break::Silent, quiet_for } if quiet_for >= Duration::from_secs(30))
    );
    let sent = client_script.sent();
    let resumed = sent.get(1).ok_or("the stream was not reopened")?;
    assert!(resumed.headers.contains(&("Last-Event-ID", "7".to_owned())));
    assert!(resumed
        .headers
        .contains(&("Accept", "text/event-stream".to_owned())));
    assert!(!sent.first().is_some_and(|first| first
        .headers
        .iter()
        .any(|(name, _)| *name == "Last-Event-ID")));
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn a_dropped_connection_and_an_unknown_kind_are_told_apart() -> Outcome {
    let (client, _) = client(Scripted::opening(vec![Ok(Opening {
        status: 200,
        chunks: vec![
            Ok(event("1", "from-the-future", &json!({})).into_bytes()),
            Err(Failure::Unreachable("reset".to_owned())),
        ],
        hangs: false,
    })]))?;
    let mut stream = client.events(Following::default());
    assert_eq!(
        stream.next().await?,
        Arrival::Unrecognised("from-the-future".to_owned())
    );
    assert!(matches!(
        stream.next().await?,
        Arrival::Gap {
            why: Break::Dropped,
            ..
        }
    ));
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn following_gives_up_once_too_many_openings_in_a_row_have_failed() -> Outcome {
    let (client, client_script) = client(Scripted::opening(vec![
        Ok(stream_of(503, &[], false)),
        Err(Failure::Unreachable("down".to_owned())),
        Ok(stream_of(503, &[], false)),
    ]))?;
    let mut stream = client.events(Following {
        reconnects: 2,
        ..quick()
    });
    assert_eq!(stream.next().await, Err(Error::StreamLost(2)));
    assert_eq!(client_script.sent().len(), 3);
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn an_opening_refused_outright_or_held_to_another_certificate_is_not_tried_again() -> Outcome
{
    let long = "x".repeat(70 * 1024);
    let (client, _) = client(Scripted::opening(vec![
        Ok(Opening {
            status: 401,
            chunks: vec![Ok(problem("ADMIT-4", "No.")), Err(Failure::TooLarge)],
            hangs: false,
        }),
        Ok(stream_of(404, &[&long, &long], false)),
        Err(Failure::CertificateRefused),
    ]))?;
    assert!(matches!(
        client.events(quick()).next().await,
        Err(Error::Refused(_))
    ));
    assert!(
        matches!(client.events(quick()).next().await, Err(Error::Refused(refusal)) if refusal.sentence.len() <= 140 * 1024)
    );
    assert_eq!(
        client.events(quick()).next().await,
        Err(Error::CertificateRefused)
    );
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn an_event_in_another_version_ends_the_stream_plainly() -> Outcome {
    let newer = "id: 1\ndata: {\"api_version\": 2, \"kind\": \"pull\", \"data\": \"x\"}\n\n";
    let (client, _) = client(Scripted::opening(vec![Ok(stream_of(200, &[newer], true))]))?;
    assert_eq!(
        client.events(Following::default()).next().await,
        Err(Error::VersionMismatch {
            spoken: 1,
            served: 2
        })
    );
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn an_opening_that_never_answers_counts_as_one_that_failed() -> Outcome {
    let client = crate::Client::with_transport(
        super::loopback()?,
        crate::Credential::new("t")?,
        super::Silent,
    )?
    .waiting(Duration::from_secs(1));
    let mut stream = client.events(Following {
        reconnects: 1,
        ..quick()
    });
    assert_eq!(stream.next().await, Err(Error::StreamLost(1)));
    Ok(())
}
