use crate::contract::refusals::NO_SUCH_JOB;
use crate::contract::{Payload, Started};
use crate::envelope::raw;
use crate::line::succeeded;
use crate::refusal::{problem_in, refusal_of};
use crate::transport::Response;
use crate::{Envelope, Error};

/// Where the work one name stands for got to. A failure is an [`Error`] instead.
#[derive(Debug, Clone, PartialEq)]
pub enum JobStanding {
    /// Still going: the envelope the accepting reply carried.
    Running(Envelope<Started>),
    /// Finished: what the equivalent command answers with.
    Finished(Envelope<Payload>),
    /// Ended before it finished: its name was released, or it was let go because nothing
    /// asked about it.
    Ended(Envelope<Started>),
}

pub(crate) fn standing_of(job: &str, answer: &Response) -> Result<JobStanding, Error> {
    if answer.status == 404 {
        let code = problem_in(&answer.body).map(|problem| problem.code);
        if code.is_none() || code.as_deref() == Some(NO_SUCH_JOB.code) {
            return Err(Error::NoSuchJob(job.to_owned()));
        }
    }
    if !succeeded(answer) {
        return Err(refusal_of(answer, None));
    }
    let envelope = raw(&answer.body)?;
    if envelope.kind() != "job" {
        return Ok(JobStanding::Finished(envelope.payload()?));
    }
    let started = envelope.expect(&["job"], false)?;
    Ok(if answer.status == 202 {
        JobStanding::Running(started)
    } else {
        JobStanding::Ended(started)
    })
}
