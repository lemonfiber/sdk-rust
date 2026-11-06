use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::contract::{Payload, API_VERSION, KINDS};
use crate::Error;

/// One answer in the envelope every answer arrives in, its data typed.
#[derive(Debug, Clone, PartialEq)]
pub struct Envelope<T> {
    /// The wire version it was written in, which is the one this client speaks.
    pub api_version: u32,
    /// Which kind of document it is.
    pub kind: String,
    /// The machine it is about, where that is not the one lemonfiber runs on.
    pub host: Option<String>,
    /// The work that said it, by the name its accepting reply gave it.
    pub job: Option<String>,
    /// What it carries.
    pub data: T,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Raw {
    kind: String,
    host: Option<String>,
    job: Option<String>,
    data: Value,
}

pub(crate) fn raw(body: &[u8]) -> Result<Raw, Error> {
    let document: Value =
        serde_json::from_slice(body).map_err(|_| Error::Unreadable("it is not JSON".to_owned()))?;
    let Value::Object(mut fields) = document else {
        return Err(Error::Unreadable("it is not an envelope".to_owned()));
    };
    let served = fields
        .get("api_version")
        .and_then(Value::as_u64)
        .ok_or_else(|| Error::Unreadable("it carries no api_version".to_owned()))?;
    if served != u64::from(API_VERSION) {
        return Err(Error::VersionMismatch {
            spoken: API_VERSION,
            served,
        });
    }
    let kind = match fields.remove("kind") {
        Some(Value::String(kind)) if !kind.is_empty() => kind,
        _ => return Err(Error::Unreadable("it names no kind".to_owned())),
    };
    let data = fields
        .remove("data")
        .ok_or_else(|| Error::Unreadable("it carries no data".to_owned()))?;
    let text = |field: Option<Value>| match field {
        Some(Value::String(said)) => Some(said),
        _ => None,
    };
    Ok(Raw {
        kind,
        host: text(fields.remove("host")),
        job: text(fields.remove("job")),
        data,
    })
}

impl Raw {
    pub(crate) fn kind(&self) -> &str {
        &self.kind
    }

    pub(crate) fn payload(self) -> Result<Envelope<Payload>, Error> {
        self.expect(KINDS, true)
    }

    pub(crate) fn expect<T: DeserializeOwned>(
        self,
        kinds: &[&str],
        tagged: bool,
    ) -> Result<Envelope<T>, Error> {
        let (shell, data) = self.opened(kinds, tagged)?;
        shell.holding(serde_json::from_value(data).ok())
    }

    fn opened(self, kinds: &[&str], tagged: bool) -> Result<(Envelope<()>, Value), Error> {
        if !KINDS.contains(&self.kind.as_str()) {
            return Err(Error::UnknownKind(self.kind));
        }
        if !kinds.contains(&self.kind.as_str()) {
            return Err(Error::UnexpectedKind {
                expected: kinds.iter().map(|kind| (*kind).to_owned()).collect(),
                received: self.kind,
            });
        }
        let Self {
            kind,
            host,
            job,
            data,
        } = self;
        let data = if tagged {
            serde_json::json!({ "kind": kind, "data": data })
        } else {
            data
        };
        let shell = Envelope {
            api_version: API_VERSION,
            kind,
            host,
            job,
            data: (),
        };
        Ok((shell, data))
    }
}

impl Envelope<()> {
    fn holding<T>(self, data: Option<T>) -> Result<Envelope<T>, Error> {
        let Some(data) = data else {
            return Err(Error::Unreadable(format!(
                "its {} data is not what the contract describes",
                self.kind
            )));
        };
        Ok(Envelope {
            api_version: self.api_version,
            kind: self.kind,
            host: self.host,
            job: self.job,
            data,
        })
    }
}
