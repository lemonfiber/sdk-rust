use crate::Error;

pub(crate) const LARGEST: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Event {
    pub(crate) id: Option<String>,
    pub(crate) data: String,
}

#[derive(Debug, Default)]
struct Pending {
    id: Option<String>,
    data: Vec<String>,
    size: usize,
}

#[derive(Debug)]
pub(crate) struct Parser {
    largest: usize,
    bytes: Vec<u8>,
    pending: Pending,
}

impl Parser {
    pub(crate) fn new(largest: usize) -> Self {
        Self {
            largest,
            bytes: Vec::new(),
            pending: Pending::default(),
        }
    }

    pub(crate) fn push(&mut self, chunk: &[u8]) -> Result<Vec<Event>, Error> {
        self.bytes.extend_from_slice(chunk);
        let mut done = Vec::new();
        while let Some(end) = self
            .bytes
            .iter()
            .position(|byte| *byte == b'\n' || *byte == b'\r')
        {
            let carriage = self.bytes.get(end) == Some(&b'\r');
            let ending = match self.bytes.get(end + 1) {
                Some(b'\n') if carriage => 2,
                None if carriage => break,
                _ => 1,
            };
            let line: Vec<u8> = self.bytes.drain(..end + ending).take(end).collect();
            if let Some(event) = self.line(&String::from_utf8_lossy(&line))? {
                done.push(event);
            }
        }
        if self.bytes.len() > self.largest {
            return Err(too_large(self.largest));
        }
        Ok(done)
    }

    fn line(&mut self, line: &str) -> Result<Option<Event>, Error> {
        if line.is_empty() {
            let pending = std::mem::take(&mut self.pending);
            return Ok((!pending.data.is_empty()).then(|| Event {
                id: pending.id,
                data: pending.data.join("\n"),
            }));
        }
        let (name, value) = line.split_once(':').unwrap_or((line, ""));
        let value = value.strip_prefix(' ').unwrap_or(value);
        match name {
            "id" => self.pending.id = Some(value.to_owned()),
            "data" => {
                self.pending.size += value.len() + 1;
                if self.pending.size > self.largest + 1 {
                    return Err(too_large(self.largest));
                }
                self.pending.data.push(value.to_owned());
            }
            _ => {}
        }
        Ok(None)
    }
}

fn too_large(largest: usize) -> Error {
    Error::Unreadable(format!(
        "the stream carried an event larger than the {largest} bytes an event may be"
    ))
}
