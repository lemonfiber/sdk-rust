use crate::contract::refusals::NOT_ADMITTED;
use crate::contract::Problem;
use crate::envelope::raw;
use crate::error::{Refusal, RefusedBecause};
use crate::transport::Response;
use crate::{Error, RefusalCode};

const STRUCTURE: &[u8] = b"{[<";

const NOT_ADMITTED_SAID: &str = "lemonfiber does not admit the credential this client holds. Use the token it printed, sign in again, or use a key the operator minted.";

const TOO_MANY_SAID: &str = "Too many wrong answers lately; wait before trying again.";

pub(crate) fn problem_in(body: &[u8]) -> Option<Problem> {
    raw(body)
        .ok()
        .filter(|envelope| envelope.kind() == "error")
        .and_then(|envelope| envelope.expect::<Problem>(&["error"], false).ok())
        .map(|envelope| envelope.data)
}

fn sentence_in(body: &[u8], problem: Option<&Problem>) -> Option<String> {
    let text = String::from_utf8_lossy(body);
    let text = text.trim();
    if text.is_empty() {
        None
    } else if !text
        .starts_with(|first: char| first.is_ascii() && STRUCTURE.contains(&(first as u8)))
    {
        Some(text.to_owned())
    } else {
        problem
            .map(|problem| problem.summary.trim().to_owned())
            .filter(|summary| !summary.is_empty())
    }
}

pub(crate) fn refusal_of(answer: &Response, at_the_door: Option<(RefusedBecause, &str)>) -> Error {
    let problem = problem_in(&answer.body);
    let code = problem
        .as_ref()
        .and_then(|problem| RefusalCode::listed(&problem.code));
    let sentence = sentence_in(&answer.body, problem.as_ref());
    let refused = |because: RefusedBecause, sentence: String| {
        Error::from(Refusal {
            because,
            sentence,
            status: answer.status,
            code,
            problem: problem.clone(),
            retry_after: answer
                .header("retry-after")
                .and_then(|said| said.trim().parse().ok()),
        })
    };
    match (answer.status, at_the_door) {
        (401, Some((because, otherwise))) => refused(
            because,
            sentence.clone().unwrap_or_else(|| otherwise.to_owned()),
        ),
        (401 | 403, _) => match (&sentence, code) {
            (Some(said), Some(code)) if *code != NOT_ADMITTED => {
                refused(RefusedBecause::Declined, said.clone())
            }
            _ => refused(RefusedBecause::NotAdmitted, NOT_ADMITTED_SAID.to_owned()),
        },
        (429, _) => refused(
            RefusedBecause::TooManyAttempts,
            sentence.clone().unwrap_or_else(|| TOO_MANY_SAID.to_owned()),
        ),
        (status, _) => match sentence.clone() {
            None => Error::Unreachable(format!(
                "Something other than lemonfiber answered, with status {status}."
            )),
            Some(said) => refused(
                match status {
                    400 => RefusedBecause::Misasked,
                    404 => RefusedBecause::Missing,
                    409 => RefusedBecause::Busy,
                    _ => RefusedBecause::Failed,
                },
                said,
            ),
        },
    }
}
