use std::net::IpAddr;
use std::time::Duration;

use serde_json::json;

use super::{enveloped, problem, Outcome};
use crate::contract::reads::{Held, HeldId, Services};
use crate::contract::refusals::{NOT_ADMITTED, NO_SUCH_JOB};
use crate::contract::{Appetite, CapabilityState, Chosen};
use crate::envelope::raw;
use crate::error::{Refusal, RefusedBecause};
use crate::open::kept;
use crate::query::{located, segment};
use crate::refusal::refusal_of;
use crate::sse::{Parser, LARGEST};
use crate::transport::{Failure, Method, Request, Response};
use crate::{Address, CertificatePin, Credential, Error, File, Picture, RefusalCode, Resolve};

struct Resolving(Vec<IpAddr>);

impl Resolve for Resolving {
    fn resolve(&self, _host: &str) -> Vec<IpAddr> {
        self.0.clone()
    }
}

const PIN: &str = "ABCDEF0123456789abcdef0123456789ABCDEF0123456789abcdef0123456789";

#[test]
fn an_envelope_in_another_version_is_refused_naming_both() {
    let read = raw(br#"{"api_version": 2, "kind": "status", "data": {}}"#);
    assert_eq!(
        read,
        Err(Error::VersionMismatch {
            spoken: 1,
            served: 2
        })
    );
    let said = read
        .err()
        .map(|error| error.to_string())
        .unwrap_or_default();
    assert!(said.contains("version 1") && said.contains("version 2"));
}

#[test]
fn what_is_not_an_envelope_is_unreadable_and_quotes_nothing_of_it() {
    for (body, why) in [
        (&b"secret-token"[..], "it is not JSON"),
        (b"[]", "it is not an envelope"),
        (br#"{"kind": "status"}"#, "it carries no api_version"),
        (br#"{"api_version": 1, "data": {}}"#, "it names no kind"),
        (
            br#"{"api_version": 1, "kind": "", "data": {}}"#,
            "it names no kind",
        ),
        (
            br#"{"api_version": 1, "kind": "status"}"#,
            "it carries no data",
        ),
    ] {
        assert_eq!(raw(body), Err(Error::Unreadable(why.to_owned())));
    }
}

#[test]
fn an_envelope_carries_its_host_and_job_and_is_typed_by_its_kind() -> Outcome {
    let body = json!({ "api_version": 1, "kind": "pull", "data": "Pulling sonarr", "host": "nas", "job": "9f2c" });
    let envelope = raw(body.to_string().as_bytes())?.payload()?;
    assert_eq!(envelope.kind, "pull");
    assert_eq!(envelope.host.as_deref(), Some("nas"));
    assert_eq!(envelope.job.as_deref(), Some("9f2c"));
    assert_eq!(
        envelope.data,
        crate::contract::Payload::Pull(Box::new("Pulling sonarr".to_owned()))
    );
    Ok(())
}

#[test]
fn a_kind_this_client_does_not_know_is_named_and_one_it_does_not_expect_is_refused() -> Outcome {
    let newer = enveloped("from-the-future", &json!({}));
    assert_eq!(
        raw(&newer)?.payload(),
        Err(Error::UnknownKind("from-the-future".to_owned()))
    );
    assert_eq!(
        raw(&newer)?.expect::<serde_json::Value>(&["status"], false),
        Err(Error::UnknownKind("from-the-future".to_owned()))
    );
    let other = enveloped("pull", &json!("Pulling"));
    let refused = raw(&other)?.expect::<serde_json::Value>(&["status", "doctor"], false);
    assert_eq!(
        refused,
        Err(Error::UnexpectedKind {
            expected: vec!["status".to_owned(), "doctor".to_owned()],
            received: "pull".to_owned()
        })
    );
    assert!(refused
        .err()
        .map(|error| error.to_string())
        .unwrap_or_default()
        .contains("status or doctor"));
    let wrong = enveloped("pull", &json!({ "not": "text" }));
    assert_eq!(
        raw(&wrong)?.payload(),
        Err(Error::Unreadable(
            "its pull data is not what the contract describes".to_owned()
        ))
    );
    Ok(())
}

#[test]
fn a_read_writes_its_segments_into_the_path_and_the_rest_as_its_query() -> Outcome {
    let title = HeldId {
        id: "a b/c".to_owned(),
        member: Some("ana".to_owned()),
        defaults: None,
    };
    assert_eq!(
        located("/api/held/{id}", &["id"], &title)?,
        "/api/held/a%20b%2Fc?member=ana"
    );
    let services = Services {
        form: vec!["tv".to_owned(), "music".to_owned()],
    };
    assert_eq!(
        located("/api/services", &[], &services)?,
        "/api/services?form=tv&form=music"
    );
    assert_eq!(located("/api/held", &[], &Held::default())?, "/api/held");
    assert_eq!(
        located("/api/x", &[], &json!({ "most": 3, "none": null }))?,
        "/api/x?most=3"
    );
    Ok(())
}

#[test]
fn a_segment_a_url_would_resolve_away_is_not_sent() {
    for unkept in ["", ".", ".."] {
        assert!(matches!(segment(unkept), Err(Error::Unsendable(_))));
    }
    assert!(matches!(
        located("/api/held/{id}", &["id"], &HeldId::default()),
        Err(Error::Unsendable(said)) if said.contains("`id`")
    ));
    assert!(matches!(
        located("/api/x", &[], &"text"),
        Err(Error::Unsendable(_))
    ));
    assert!(matches!(
        located("/api/x", &[], &f64::NAN),
        Err(Error::Unsendable(_))
    ));
}

fn refused(status: u16, body: &[u8], retry: Option<&str>) -> Error {
    refusal_of(
        &Response {
            status,
            headers: retry
                .map(|said| vec![("Retry-After".to_owned(), said.to_owned())])
                .unwrap_or_default(),
            body: body.to_vec(),
        },
        None,
    )
}

fn because(error: &Error) -> Option<RefusedBecause> {
    match error {
        Error::Refused(refusal) => Some(refusal.because),
        _ => None,
    }
}

#[test]
fn a_refusal_is_read_by_its_status_and_its_code() {
    assert_eq!(
        because(&refused(400, &problem("READ-1", "Asked wrongly."), None)),
        Some(RefusedBecause::Misasked)
    );
    assert_eq!(
        because(&refused(404, b"No such word.", None)),
        Some(RefusedBecause::Missing)
    );
    assert_eq!(
        because(&refused(409, &problem("LIFE-1", "Busy."), None)),
        Some(RefusedBecause::Busy)
    );
    assert_eq!(
        because(&refused(500, &problem("X-1", "Broke."), None)),
        Some(RefusedBecause::Failed)
    );
    assert_eq!(
        because(&refused(403, &problem(NOT_ADMITTED.code, "No."), None)),
        Some(RefusedBecause::NotAdmitted)
    );
    assert_eq!(
        because(&refused(401, &problem("UNLISTED-1", "No."), None)),
        Some(RefusedBecause::NotAdmitted)
    );
    assert_eq!(
        because(&refused(
            403,
            &problem(NO_SUCH_JOB.code, "Not yours."),
            None
        )),
        Some(RefusedBecause::Declined)
    );
    assert_eq!(
        because(&refused(403, b"", None)),
        Some(RefusedBecause::NotAdmitted)
    );
}

#[test]
fn a_refusal_carries_its_sentence_code_problem_and_wait() {
    let Error::Refused(refusal) =
        refused(429, &problem("ADMIT-5", "  Wait a while.  "), Some(" 30 "))
    else {
        unreachable!("a 429 is a refusal");
    };
    assert_eq!(refusal.because, RefusedBecause::TooManyAttempts);
    assert_eq!(refusal.sentence, "Wait a while.");
    assert_eq!(refusal.retry_after, Some(30));
    assert_eq!(refusal.code.map(|code| code.code), Some("ADMIT-5"));
    assert_eq!(
        refusal.problem.map(|problem| problem.meaning),
        Some("What it means.".to_owned())
    );
    let Error::Refused(unsaid) = refused(429, b"{}", Some("soon")) else {
        unreachable!("a 429 is a refusal");
    };
    assert_eq!(unsaid.retry_after, None);
    assert!(unsaid.sentence.contains("Too many"));
    assert_eq!(unsaid.to_string(), unsaid.sentence);
}

#[test]
fn an_answer_with_no_sentence_did_not_come_from_lemonfiber() {
    assert!(matches!(refused(502, b"", None), Error::Unreachable(said) if said.contains("502")));
    assert!(matches!(
        refused(500, b"<html></html>", None),
        Error::Unreachable(_)
    ));
    assert!(matches!(
        refused(500, &problem("X-1", "   "), None),
        Error::Unreachable(_)
    ));
    assert!(matches!(
        refused(500, &enveloped("status", &json!({})), None),
        Error::Unreachable(_)
    ));
}

#[test]
fn the_door_names_its_own_refusal_and_keeps_a_sentence_it_was_given() {
    let door = Some((RefusedBecause::PasswordRefused, "Not the password."));
    let answer = |body: &[u8]| Response {
        status: 401,
        headers: Vec::new(),
        body: body.to_vec(),
    };
    assert!(
        matches!(refusal_of(&answer(b""), door), Error::Refused(refusal) if refusal.sentence == "Not the password.")
    );
    assert!(
        matches!(refusal_of(&answer(b"Wrong."), door), Error::Refused(refusal) if refusal.sentence == "Wrong.")
    );
}

#[test]
fn a_listed_code_is_found_and_another_is_not() {
    assert_eq!(RefusalCode::listed(NO_SUCH_JOB.code), Some(&NO_SUCH_JOB));
    assert_eq!(RefusalCode::listed("NOPE-1"), None);
    assert!(crate::contract::key_callable::ALL
        .iter()
        .any(|callable| callable.moved.is_some()));
}

#[test]
fn the_stream_is_read_whatever_way_its_lines_end_and_however_it_is_cut() -> Outcome {
    let mut parser = Parser::new(64);
    assert!(parser
        .push(b": heartbeat\r\n\r\nid: 1\revent: status\r")?
        .is_empty());
    let events = parser.push(b"\ndata: one\ndata:two\n\nid\n\n")?;
    assert_eq!(events.len(), 1);
    assert_eq!(
        events
            .first()
            .map(|event| (event.id.as_deref(), event.data.as_str())),
        Some((Some("1"), "one\ntwo"))
    );
    let mut parser = Parser::new(8);
    assert!(matches!(
        parser.push(b"data: 123456789\n"),
        Err(Error::Unreadable(_))
    ));
    let mut parser = Parser::new(12);
    assert!(matches!(parser.push(b"data: 1234"), Ok(events) if events.is_empty()));
    assert!(matches!(parser.push(b"56789"), Err(Error::Unreadable(_))));
    assert_eq!(LARGEST, 16_777_216);
    let mut parser = Parser::new(8);
    assert!(matches!(parser.push(b"data: 12"), Ok(events) if events.is_empty()));
    let events = parser.push(b"345678\n\n")?;
    assert_eq!(
        events.first().map(|event| event.data.as_str()),
        Some("12345678")
    );
    Ok(())
}

#[test]
fn a_pin_is_a_sha256_in_hexadecimal() -> Outcome {
    let pin = CertificatePin::new(PIN)?;
    assert_eq!(pin.hex(), PIN.to_ascii_lowercase());
    assert!(format!("{pin:?}").contains(&PIN.to_ascii_lowercase()));
    assert!(matches!(
        CertificatePin::new("abc"),
        Err(Error::AddressRefused(_))
    ));
    assert!(matches!(
        CertificatePin::new(&"g".repeat(64)),
        Err(Error::AddressRefused(_))
    ));
    assert_eq!(
        CertificatePin::of(b"abc").hex(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    Ok(())
}

#[test]
fn an_address_on_this_machine_needs_no_pin_and_any_other_needs_one() -> Outcome {
    let v4 = Address::new("http://127.0.0.1:9000/", None)?;
    assert_eq!(
        (v4.base(), v4.host(), v4.port()),
        ("http://127.0.0.1:9000", "127.0.0.1", 9000)
    );
    assert!(v4.reached().is_empty() && v4.pin().is_none());
    let v6 = Address::new("http://[::1]:9000/lemonfiber/", None)?;
    assert_eq!(
        (v6.base(), v6.host()),
        ("http://[::1]:9000/lemonfiber", "::1")
    );
    assert_eq!(Address::new("http://[::ffff:127.0.0.1]/", None)?.port(), 80);
    let named = Address::resolving(
        "http://Localhost:9000",
        None,
        &Resolving(vec![
            "127.0.0.1".parse()?,
            "::1".parse()?,
            "127.0.0.1".parse()?,
        ]),
    )?;
    assert_eq!(named.reached().len(), 2);
    assert_eq!(named.url("/api/status"), "http://localhost:9000/api/status");
    let pinned = Address::new(
        "https://stack.example:8443",
        Some(CertificatePin::new(PIN)?),
    )?;
    assert!(pinned.pin().is_some() && pinned.reached().is_empty());
    assert!(format!("{pinned:?}").contains("stack.example"));
    assert_eq!(Address::new("https://127.0.0.1", None)?.port(), 443);
    Ok(())
}

#[test]
fn an_address_lemonfiber_does_not_print_is_refused() -> Outcome {
    let elsewhere = Resolving(vec!["127.0.0.1".parse()?, "192.0.2.1".parse()?]);
    let pin = || CertificatePin::new(PIN);
    for (url, pinned) in [
        ("not an address", None),
        ("ftp://127.0.0.1", None),
        ("http://user:pass@127.0.0.1", None),
        ("http://user@127.0.0.1", None),
        ("http://:pass@127.0.0.1", None),
        ("http://127.0.0.1/?token=x", None),
        ("http://127.0.0.1/#x", None),
        ("http://192.0.2.1", None),
        ("http://127.0.0.1", Some(pin()?)),
        ("unix:/run/lemonfiber.sock", None),
    ] {
        assert!(
            matches!(Address::new(url, pinned), Err(Error::AddressRefused(_))),
            "{url}"
        );
    }
    assert!(matches!(
        Address::resolving("http://nas.local", None, &elsewhere),
        Err(Error::AddressRefused(_))
    ));
    assert!(matches!(
        Address::resolving("http://nas.local", None, &Resolving(Vec::new())),
        Err(Error::AddressRefused(_))
    ));
    assert!(matches!(
        Address::new("data:text/plain,hi", None),
        Err(Error::AddressRefused(_))
    ));
    Ok(())
}

#[test]
fn the_system_resolver_finds_this_machine() {
    let found = crate::SystemResolver.resolve("localhost");
    assert!(!found.is_empty() && found.iter().all(IpAddr::is_loopback));
    assert!(crate::SystemResolver.resolve("name.invalid").is_empty());
}

#[test]
fn a_credential_is_one_a_header_carries_and_is_shown_nowhere() -> Outcome {
    let credential = Credential::new("lfk_secret")?;
    assert_eq!(format!("{credential:?}"), "Credential(hidden)");
    for unsendable in ["", "has space", "line\nfeed", "é"] {
        assert!(matches!(
            Credential::new(unsendable),
            Err(Error::Unsendable(_))
        ));
    }
    Ok(())
}

#[test]
fn nothing_secret_is_shown_when_a_request_or_an_answer_is_printed() {
    let request = Request {
        method: Method::Post,
        url: "http://127.0.0.1:9000/api/keys".to_owned(),
        headers: vec![("X-Lemonfiber-Token", "secret".to_owned())],
        body: Some(b"secret".to_vec()),
        most: None,
    };
    assert_eq!(
        format!("{request:?}"),
        "Request(POST http://127.0.0.1:9000/api/keys)"
    );
    let response = Response {
        status: 200,
        headers: vec![("Content-Type".to_owned(), "text/plain".to_owned())],
        body: b"secret".to_vec(),
    };
    assert_eq!(format!("{response:?}"), "Response(200)");
    assert_eq!(response.header("content-type"), Some("text/plain"));
    assert_eq!(response.header("x-other"), None);
    let file = File {
        content: b"secret".to_vec(),
        content_type: None,
    };
    assert_eq!(format!("{file:?}"), "File(6 bytes, None)");
    let picture = Picture {
        content: b"secret".to_vec(),
        media_type: "image/png".to_owned(),
    };
    assert_eq!(format!("{picture:?}"), "Picture(6 bytes, image/png)");
    assert_eq!(
        [Method::Get, Method::Post, Method::Delete].map(Method::as_str),
        ["GET", "POST", "DELETE"]
    );
}

#[test]
fn every_error_says_what_happened_in_a_sentence() {
    let refusal = Refusal {
        because: RefusedBecause::Missing,
        sentence: "No such word.".to_owned(),
        status: 404,
        code: None,
        problem: None,
        retry_after: None,
    };
    for (error, said) in [
        (
            Error::Unsendable("Not sendable.".to_owned()),
            "Not sendable.",
        ),
        (Error::AddressRefused("Refused.".to_owned()), "Refused."),
        (Error::Unreachable("Gone.".to_owned()), "Gone."),
        (Error::CertificateRefused, "certificate"),
        (Error::TimedOut(Duration::from_millis(1500)), "1.5 seconds"),
        (Error::Unreadable("it is short".to_owned()), "it is short."),
        (Error::UnknownKind("x".to_owned()), "\"x\""),
        (Error::from(refusal), "No such word."),
        (Error::NoSuchJob("j".to_owned()), "\"j\""),
        (
            Error::StillRunning {
                job: "j".to_owned(),
                waited: Duration::from_secs(2),
            },
            "2 seconds",
        ),
        (Error::StreamLost(5), "5 attempts"),
        (Error::ClaimLapsed, "lapsed"),
    ] {
        assert!(error.to_string().contains(said), "{error}");
    }
    assert!(matches!(
        crate::line::failed(&Failure::TooLarge),
        Error::Unreadable(_)
    ));
    let unencodable = serde_json::from_str::<serde_json::Value>("{").map(|_| Vec::new());
    assert!(matches!(
        crate::line::Asked::carrying(Method::Post, "/api/keys".to_owned(), unencodable),
        Err(Error::Unsendable(_))
    ));
}

#[test]
fn a_value_a_read_answers_with_that_this_client_was_not_generated_with_is_kept() -> Outcome {
    let state: CapabilityState = serde_json::from_value(json!("available"))?;
    assert_eq!(state, CapabilityState::Available);
    let newer = json!("something-newer");
    let state: CapabilityState = serde_json::from_value(newer.clone())?;
    assert_eq!(
        state,
        CapabilityState::Unrecognised("something-newer".to_owned())
    );
    assert_eq!(serde_json::to_value(&state)?, newer);
    Ok(())
}

#[test]
fn a_tag_this_client_does_not_know_keeps_all_it_carried() -> Outcome {
    let newer = json!({ "chosen": "something-newer", "held": 1 });
    let chosen: Chosen = serde_json::from_value(newer.clone())?;
    assert_eq!(chosen, Chosen::Unrecognised(newer.clone()));
    assert_eq!(serde_json::to_value(&chosen)?, newer);
    Ok(())
}

#[test]
fn a_tag_this_client_knows_reads_as_that_tag_or_not_at_all() -> Outcome {
    let named: Chosen = serde_json::from_value(json!({ "chosen": "named", "door": "caddy" }))?;
    assert_eq!(
        named,
        Chosen::Named {
            door: "caddy".to_owned()
        }
    );
    assert!(serde_json::from_value::<Chosen>(json!({ "chosen": "named" })).is_err());
    assert!(serde_json::from_value::<Chosen>(json!({ "chosen": "named", "door": 1 })).is_err());
    Ok(())
}

#[test]
fn a_value_naming_no_tag_is_not_kept() {
    assert!(serde_json::from_value::<Chosen>(json!({ "held": 1 })).is_err());
    assert!(serde_json::from_value::<Chosen>(json!({ "chosen": 1 })).is_err());
}

#[test]
fn only_a_key_this_client_does_not_know_is_kept_of_a_keyed_value() {
    let known = ["none", "custom"];
    for newer in [json!("newer"), json!({ "newer": 1 })] {
        assert_eq!(kept(newer.clone(), None, &known), Ok(newer));
    }
    for refused in [
        json!("none"),
        json!({ "custom": "held" }),
        json!({ "newer": 1, "newest": 2 }),
        json!({}),
        json!(1),
    ] {
        assert!(kept(refused, None, &known).is_err());
    }
}

#[test]
fn a_value_a_request_sends_is_only_ever_one_the_contract_lists() {
    assert!(serde_json::from_value::<Appetite>(json!("something-newer")).is_err());
}
