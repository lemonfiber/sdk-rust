use std::time::Duration;

use serde_json::json;

use super::{client, enveloped, listing, loopback, problem, reply, started, Outcome, Scripted};
use crate::contract::actions::{Diagnose, Restart};
use crate::contract::reads::{
    AskableIdPoster, BundleName, Explain, ExplainAnswer, HeldIdPoster, Logs, Playing,
};
use crate::contract::refusals::NO_SUCH_JOB;
use crate::contract::{CapabilityState, CredentialScope, Payload};
use crate::transport::{Failure, Method, Response};
use crate::{
    Acted, Action, Address, CapabilitySet, CertificatePin, Client, Credential, Error,
    IdempotencyKey, JobStanding, Logged, Read, PICTURE_MOST, PICTURE_TYPES,
};

fn playing() -> Vec<u8> {
    enveloped(
        "playing",
        &json!({ "available": true, "findings": [], "member": "ana", "sessions": [] }),
    )
}

fn header<'a>(request: &'a crate::Request, name: &str) -> Option<&'a str> {
    request
        .headers
        .iter()
        .find(|(held, _)| *held == name)
        .map(|(_, value)| value.as_str())
}

#[tokio::test]
async fn a_read_asks_for_its_path_carrying_the_token_in_its_header() -> Outcome {
    let (client, client_script) = client(Scripted::answering(vec![Ok(reply(200, playing()))]))?;
    let read = Playing {
        member: Some("ana".to_owned()),
    };
    let envelope = client.read(&read).await?;
    assert_eq!(envelope.data.member, "ana");
    let sent = client_script.sent();
    let request = sent.first().ok_or("nothing was sent")?;
    assert_eq!(request.url, "http://127.0.0.1:9000/api/playing?member=ana");
    assert_eq!(header(request, "X-Lemonfiber-Token"), Some("token"));
    assert_eq!(header(request, "Accept"), Some("application/json"));
    assert!(!request.url.contains("token"));
    Ok(())
}

#[tokio::test]
async fn a_read_answering_with_several_kinds_is_typed_by_the_kind_that_arrived() -> Outcome {
    let word = json!({ "also_called": [], "forms": [], "short": "A form.", "word": "form" });
    let (client, _) = client(Scripted::answering(vec![
        Ok(reply(200, enveloped("glossary", &json!({ "words": [] })))),
        Ok(reply(200, enveloped("word", &word))),
        Ok(reply(200, enveloped("status", &json!({})))),
    ]))?;
    assert!(matches!(
        client.read(&Explain::default()).await?.data,
        ExplainAnswer::Glossary(_)
    ));
    assert!(matches!(
        client.read(&Explain::default()).await?.data,
        ExplainAnswer::Word(_)
    ));
    assert!(matches!(
        client.read(&Explain::default()).await,
        Err(Error::UnexpectedKind { .. })
    ));
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn a_read_is_asked_again_after_a_passing_failure_and_an_action_never_is() -> Outcome {
    let (client, client_script) = client(Scripted::answering(vec![
        Err(Failure::Unreachable("asleep".to_owned())),
        Ok(reply(503, Vec::new())),
        Ok(reply(200, playing())),
        Ok(reply(503, Vec::new())),
    ]))?;
    client.read(&Playing::default()).await?;
    assert_eq!(client_script.sent().len(), 3);
    assert!(matches!(
        client
            .act(&Diagnose {
                only: None,
                disruptive: false
            })
            .await,
        Err(Error::Unreachable(_))
    ));
    let sent = client_script.sent();
    assert_eq!(sent.len(), 4);
    assert!(sent.last().is_some_and(|action| action
        .headers
        .contains(&("Content-Type", "application/json".to_owned()))));
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn a_second_attempt_waits_twice_as_long_as_the_first() -> Outcome {
    let (client, _) = client(Scripted::answering(vec![
        Ok(reply(503, Vec::new())),
        Ok(reply(503, Vec::new())),
        Ok(reply(200, playing())),
    ]))?;
    let started = tokio::time::Instant::now();
    client.read(&Playing::default()).await?;
    let waited = started.elapsed();
    assert!(waited >= Duration::from_millis(750) && waited < Duration::from_millis(760));
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn asking_where_work_stands_is_asked_again_and_letting_it_go_never_is() -> Outcome {
    let (client, client_script) = client(Scripted::answering(vec![
        Ok(reply(503, Vec::new())),
        Ok(reply(200, started("job-1"))),
        Ok(reply(503, Vec::new())),
    ]))?;
    client.job("job-1").await?;
    assert_eq!(client_script.sent().len(), 2);
    assert!(client.release("job-1").await.is_err());
    assert_eq!(client_script.sent().len(), 3);
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn a_read_stops_asking_after_three_attempts_or_when_the_wait_would_run_out() -> Outcome {
    let (client, client_script) = client(Scripted::answering(vec![Ok(reply(502, Vec::new())); 4]))?;
    assert!(
        matches!(client.read(&Playing::default()).await, Err(Error::Unreachable(said)) if said.contains("502"))
    );
    assert_eq!(client_script.sent().len(), 3);
    let (hurried, hurried_script) =
        super::client(Scripted::answering(vec![Ok(reply(502, Vec::new())); 2]))?;
    let hurried = hurried.waiting(Duration::from_millis(100));
    assert!(matches!(
        hurried.read(&Playing::default()).await,
        Err(Error::Unreachable(_))
    ));
    assert_eq!(hurried_script.sent().len(), 1);
    let (nobody, nobody_script) = super::client(Scripted::default())?;
    assert!(
        matches!(nobody.read(&Playing::default()).await, Err(Error::Unreachable(said)) if said.contains("not answering"))
    );
    assert_eq!(nobody_script.sent().len(), 3);
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn a_call_waits_no_longer_than_it_was_given() -> Outcome {
    let client = Client::with_transport(loopback()?, Credential::new("token")?, super::Silent)?
        .waiting(Duration::from_secs(2));
    assert_eq!(
        client.read(&Playing::default()).await,
        Err(Error::TimedOut(Duration::from_secs(2)))
    );
    Ok(())
}

#[tokio::test]
async fn a_certificate_refused_or_an_answer_too_large_is_said_so() -> Outcome {
    let (client, _) = client(Scripted::answering(vec![
        Err(Failure::CertificateRefused),
        Err(Failure::TooLarge),
    ]))?;
    assert_eq!(
        client.read(&Playing::default()).await,
        Err(Error::CertificateRefused)
    );
    assert!(matches!(
        client.read(&Playing::default()).await,
        Err(Error::Unreadable(_))
    ));
    Ok(())
}

#[test]
fn a_pinned_stack_is_reached_only_through_the_crate_s_own_transport() -> Outcome {
    let pin = CertificatePin::new(&"a".repeat(64))?;
    let pinned = Address::new("https://stack.example", Some(pin))?;
    assert!(matches!(
        Client::with_transport(
            pinned.clone(),
            Credential::new("token")?,
            Scripted::default()
        ),
        Err(Error::AddressRefused(_))
    ));
    assert_eq!(
        Client::new(pinned, Credential::new("token")?)?
            .address()
            .host(),
        "stack.example"
    );
    Ok(())
}

#[tokio::test]
async fn a_file_is_kept_as_the_bytes_that_arrived() -> Outcome {
    let (client, client_script) = client(Scripted::answering(vec![
        Ok(Response {
            status: 200,
            headers: vec![("content-type".to_owned(), "application/zip".to_owned())],
            body: b"PK".to_vec(),
        }),
        Ok(reply(404, problem("BUNDLE-1", "No such bundle."))),
    ]))?;
    let bundle = BundleName {
        name: "support-1.zip".to_owned(),
    };
    let file = client.file(&bundle).await?;
    assert_eq!(
        (file.content.as_slice(), file.content_type.as_deref()),
        (&b"PK"[..], Some("application/zip"))
    );
    assert!(matches!(client.file(&bundle).await, Err(Error::Refused(_))));
    let sent = client_script.sent();
    assert_eq!(
        sent.first().map(|request| request.url.as_str()),
        Some("http://127.0.0.1:9000/api/bundle/support-1.zip")
    );
    assert_eq!(
        sent.first().and_then(|request| header(request, "Accept")),
        Some("*/*")
    );
    Ok(())
}

#[tokio::test]
async fn a_picture_is_a_raster_image_of_at_most_its_size() -> Outcome {
    let labelled = |label: &str| {
        Ok(Response {
            status: 200,
            headers: vec![("Content-Type".to_owned(), label.to_owned())],
            body: b"\x89PNG".to_vec(),
        })
    };
    assert_eq!(PICTURE_MOST, 2_097_152);
    let (client, client_script) = client(Scripted::answering(vec![
        labelled("image/PNG; charset=binary"),
        labelled("image/svg+xml"),
        Ok(reply(200, Vec::new())),
        Ok(reply(404, problem("PLAY-9", "No poster."))),
    ]))?;
    let poster = HeldIdPoster {
        id: "t1".to_owned(),
        ..HeldIdPoster::default()
    };
    assert_eq!(client.picture(&poster).await?.media_type, "image/png");
    assert!(
        matches!(client.picture(&poster).await, Err(Error::Unreadable(said)) if said.contains("image/svg+xml"))
    );
    assert!(
        matches!(client.picture(&poster).await, Err(Error::Unreadable(said)) if said.contains("\"\""))
    );
    assert!(matches!(
        client.picture(&AskableIdPoster::default()).await,
        Err(Error::Unsendable(_))
    ));
    assert!(matches!(
        client.picture(&poster).await,
        Err(Error::Refused(_))
    ));
    let sent = client_script.sent();
    assert_eq!(
        sent.first().and_then(|request| request.most),
        Some(PICTURE_MOST)
    );
    assert!(client_script.sent().first().is_some_and(|asked| asked
        .headers
        .contains(&("Accept", PICTURE_TYPES.join(", ")))));
    Ok(())
}

#[tokio::test]
async fn the_logs_are_one_envelope_a_line_or_the_work_following_them() -> Outcome {
    let line = |text: &str| {
        enveloped(
            "log",
            &json!({ "line": text, "service": "sonarr", "stream": "stdout" }),
        )
    };
    let mut lines = line("one");
    lines.extend_from_slice(b"\n\n");
    lines.extend(line("two"));
    lines.push(b'\n');
    let (client, _) = client(Scripted::answering(vec![
        Ok(reply(200, lines)),
        Ok(reply(200, started("follow-1"))),
        Ok(reply(200, enveloped("pull", &json!("Pulling")))),
        Ok(reply(400, problem("READ-1", "Not a form."))),
    ]))?;
    let read = Logs {
        tail: Some("2".to_owned()),
        ..Logs::default()
    };
    assert!(matches!(client.logs(&read).await?, Logged::Lines(lines) if lines.len() == 2));
    assert!(
        matches!(client.logs(&read).await?, Logged::Following(job) if job.data.job == "follow-1")
    );
    assert!(matches!(
        client.logs(&read).await,
        Err(Error::UnexpectedKind { .. })
    ));
    assert!(matches!(client.logs(&read).await, Err(Error::Refused(_))));
    Ok(())
}

#[tokio::test]
async fn an_action_answers_now_or_with_a_name_for_the_work() -> Outcome {
    let (client, client_script) = client(Scripted::answering(vec![
        Ok(reply(200, enveloped("pull", &json!("Pulled")))),
        Ok(reply(202, started("job-1"))),
        Ok(reply(409, problem("LIFE-1", "Something else is running."))),
    ]))?;
    let restart = Restart {
        forms: Some(vec!["tv".to_owned()]),
        dry_run: Some(true),
        ..Restart::default()
    };
    assert!(
        matches!(client.act(&restart).await?, Acted::Answered(envelope) if envelope.data == Payload::Pull(Box::new("Pulled".to_owned())))
    );
    let key = IdempotencyKey::new("attempt-1")?;
    assert!(
        matches!(client.act_once(&restart, &key).await?, Acted::Started(job) if job.data.job == "job-1")
    );
    assert!(
        matches!(client.act(&restart).await, Err(Error::Refused(refusal)) if refusal.because == crate::RefusedBecause::Busy)
    );
    let sent = client_script.sent();
    let first = sent.first().ok_or("nothing was sent")?;
    assert_eq!(
        (first.method, first.url.as_str()),
        (Method::Post, "http://127.0.0.1:9000/api/actions/restart")
    );
    assert_eq!(
        first.body.as_deref(),
        Some(&br#"{"forms":["tv"],"dry_run":true}"#[..])
    );
    assert_eq!(header(first, "Idempotency-Key"), None);
    assert_eq!(
        sent.get(1)
            .and_then(|second| header(second, "Idempotency-Key")),
        Some("attempt-1")
    );
    assert!(Restart::REHEARSAL && Restart::CONSENT == ["offer"]);
    Ok(())
}

#[test]
fn an_idempotency_key_is_visible_ascii_of_a_bounded_length() {
    assert!(IdempotencyKey::new("x".repeat(255)).is_ok());
    for refused in [String::new(), "x".repeat(256), "has space".to_owned()] {
        assert!(matches!(
            IdempotencyKey::new(refused),
            Err(Error::Unsendable(_))
        ));
    }
}

#[tokio::test]
async fn what_a_stack_can_do_is_read_for_the_credential_that_asked() -> Outcome {
    let capabilities = json!({
        "capabilities": {
            "/api/playing": "available",
            "/api/actions/restart": "unpermitted",
            "/api/something-newer": "unconfigured",
        },
        "scope": "member",
        "stack": "stack-1",
    });
    let (client, _) = client(Scripted::answering(vec![Ok(reply(
        200,
        enveloped("capabilities", &capabilities),
    ))]))?;
    let set: CapabilitySet = client.capabilities().await?;
    assert_eq!(set.scope, CredentialScope::Member);
    assert_eq!(set.stack.as_deref(), Some("stack-1"));
    assert_eq!(set.of_read::<Playing>(), Some(&CapabilityState::Available));
    assert_eq!(
        set.of_action::<Restart>(),
        Some(&CapabilityState::Unpermitted)
    );
    assert_eq!(set.of("/api/held"), None);
    assert_eq!(
        set.in_state(&CapabilityState::Unconfigured)
            .into_iter()
            .collect::<Vec<_>>(),
        ["/api/something-newer"]
    );
    assert_eq!(Playing::KINDS, ["playing"]);
    Ok(())
}

#[tokio::test]
async fn work_is_read_as_running_finished_or_ended_by_its_status_and_kind() -> Outcome {
    let (client, client_script) = client(Scripted::answering(vec![
        Ok(reply(202, started("job-1"))),
        Ok(reply(200, enveloped("pull", &json!("Done")))),
        Ok(reply(200, started("job-1"))),
        Ok(reply(
            404,
            problem(NO_SUCH_JOB.code, "No work by that name."),
        )),
        Ok(reply(404, Vec::new())),
        Ok(reply(404, problem("STACK-1", "No such service."))),
        Ok(reply(500, problem("STACK-2", "It broke."))),
    ]))?;
    assert!(matches!(
        client.job("job-1").await?,
        JobStanding::Running(_)
    ));
    assert!(matches!(
        client.job("job-1").await?,
        JobStanding::Finished(_)
    ));
    assert!(matches!(
        client.release("job-1").await?,
        JobStanding::Ended(_)
    ));
    assert_eq!(
        client.job("old").await,
        Err(Error::NoSuchJob("old".to_owned()))
    );
    assert_eq!(
        client.job("old").await,
        Err(Error::NoSuchJob("old".to_owned()))
    );
    assert!(
        matches!(client.job("job-1").await, Err(Error::Refused(refusal)) if refusal.because == crate::RefusedBecause::Missing)
    );
    assert!(
        matches!(client.job("job-1").await, Err(Error::Refused(refusal)) if refusal.because == crate::RefusedBecause::Failed)
    );
    let sent = client_script.sent();
    assert_eq!(
        sent.get(2).map(|request| request.method),
        Some(Method::Delete)
    );
    assert_eq!(
        sent.first().map(|request| request.url.as_str()),
        Some("http://127.0.0.1:9000/api/jobs/job-1")
    );
    Ok(())
}

#[tokio::test(start_paused = true)]
async fn work_is_followed_until_it_is_no_longer_going_or_the_wait_is_up() -> Outcome {
    let (client, _) = client(Scripted::answering(vec![
        Ok(reply(202, started("job-1"))),
        Ok(reply(200, enveloped("pull", &json!("Done")))),
        Ok(reply(202, started("job-2"))),
        Ok(reply(202, started("job-2"))),
        Ok(reply(202, started("job-2"))),
        Ok(reply(202, started("job-3"))),
        Ok(reply(200, started("job-3"))),
    ]))?;
    let every = Duration::from_secs(1);
    assert!(matches!(
        client.follow("job-1", every, None).await?,
        JobStanding::Finished(_)
    ));
    assert!(matches!(
        client.follow("job-2", every, Some(Duration::from_millis(1500))).await,
        Err(Error::StillRunning { job, .. }) if job == "job-2"
    ));
    assert!(matches!(
        client
            .follow("job-3", every, Some(Duration::from_secs(10)))
            .await?,
        JobStanding::Ended(_)
    ));
    Ok(())
}

#[tokio::test]
async fn keys_and_device_keys_are_asked_for_at_their_own_doors() -> Outcome {
    let (client, client_script) = client(Scripted::answering(vec![
        Ok(reply(200, enveloped("keys", &listing()))),
        Ok(reply(200, enveloped("keys", &listing()))),
        Ok(reply(
            200,
            enveloped("device-keys", &json!({ "keys": [], "rehearsed": false })),
        )),
        Ok(reply(
            200,
            enveloped(
                "device-challenge",
                &json!({ "challenge": "c", "expires": "1" }),
            ),
        )),
        Ok(reply(
            200,
            enveloped("device-keys", &json!({ "keys": [], "rehearsed": false })),
        )),
    ]))?;
    client.keys().await?;
    client.revoke("home assistant").await?;
    client.device_keys().await?;
    assert_eq!(
        client.device_key_challenge("phone").await?.data.challenge,
        "c"
    );
    client.revoke_device_key("phone").await?;
    assert!(matches!(
        client.revoke("..").await,
        Err(Error::Unsendable(_))
    ));
    let sent: Vec<(Method, String)> = client_script
        .sent()
        .into_iter()
        .map(|request| (request.method, request.url))
        .collect();
    assert_eq!(
        sent,
        [
            (Method::Get, "http://127.0.0.1:9000/api/keys".to_owned()),
            (
                Method::Delete,
                "http://127.0.0.1:9000/api/keys/home%20assistant".to_owned()
            ),
            (
                Method::Get,
                "http://127.0.0.1:9000/api/device-keys".to_owned()
            ),
            (
                Method::Post,
                "http://127.0.0.1:9000/api/device-keys/phone/challenge".to_owned()
            ),
            (
                Method::Delete,
                "http://127.0.0.1:9000/api/device-keys/phone".to_owned()
            ),
        ]
    );
    Ok(())
}

#[tokio::test]
async fn setup_is_walked_one_request_a_step() -> Outcome {
    let wizard = || {
        enveloped(
            "wizard",
            &json!({
                "asks": true, "at": "welcome", "guard": [], "offered": false, "phase": "in-progress",
                "plan": [], "ready_for_review": false, "rehearsed": false, "unanswered": [], "written": [],
            }),
        )
    };
    let (client, client_script) = client(Scripted::answering(vec![Ok(reply(200, wizard())); 6]))?;
    let walked = [
        client.setup().await?,
        client
            .setup_answer(&crate::contract::SetupAnswerBody::Household(true))
            .await?,
        client.setup_next().await?,
        client.setup_back().await?,
        client.setup_apply().await?,
        client
            .setup_recover(&crate::contract::SetupRecoverBody {
                choice: crate::contract::Choice::Resume,
            })
            .await?,
    ];
    let sent: Vec<String> = client_script
        .sent()
        .into_iter()
        .map(|request| request.url)
        .collect();
    assert_eq!(
        sent.iter()
            .map(|url| url.trim_start_matches("http://127.0.0.1:9000"))
            .collect::<Vec<_>>(),
        [
            "/api/setup",
            "/api/setup/answer",
            "/api/setup/next",
            "/api/setup/back",
            "/api/setup/apply",
            "/api/setup/recover"
        ]
    );
    assert!(walked.iter().all(|envelope| envelope.data.asks));
    assert_eq!(
        client_script
            .sent()
            .get(1)
            .and_then(|request| request.body.clone()),
        Some(br#"{"household":true}"#.to_vec())
    );
    Ok(())
}
