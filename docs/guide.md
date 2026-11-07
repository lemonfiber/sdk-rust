# Using the Rust client

This guide covers everything the `lemonfiber-sdk` crate does. The
[README](../README.md) has the install steps and a first call.

## The client

A `Client` talks to one stack with one credential. Every call is `async`, needs a
Tokio runtime, and returns a `Result`. Nothing panics for an expected failure.

```rust
use lemonfiber_sdk::{Address, Client, Credential};

let address = Address::new("http://127.0.0.1:9000", None)?;
let client = Client::new(address, Credential::new(token)?)?;
```

Each call, its attempts included, waits 30 seconds (`DEFAULT_WAIT`) for an answer;
`client.waiting(duration)` gives another wait.

## The address

A loopback address, or a host name that resolves only to loopback, needs nothing
more. Any other address is refused unless it comes with the stack's certificate pin,
and it must be `https`:

```rust
use lemonfiber_sdk::{Address, CertificatePin};

let pin = CertificatePin::new("86b25c676b761e9a398081373fec783c2bec970baa255370838aebb5c687841e")?;
let address = Address::new("https://nas.local:8443", Some(pin))?;
```

The pin is the SHA-256 digest of the stack's certificate, 64 hexadecimal characters.
`lemonfiber ui --lan --tls` prints it when it starts. It is checked during the TLS
handshake, before any request is written, and no argument weakens it.

A host name given without a pin is resolved once, when the `Address` is made, and
every connection goes to the addresses it resolved to then. `Address::resolving` takes
your own `Resolve` for that step.

## The credential

A `Credential` is one of:

- the per-run token `lemonfiber ui` prints;
- a session, from exchanging a password once at the door;
- an integration key the operator minted.

It travels in the `X-Lemonfiber-Token` header and never in a URL, and its `Debug`
shows none of it.

```rust
use lemonfiber_sdk::contract::SessionBody;
use lemonfiber_sdk::{Client, Door};

let door = Door::new(address.clone())?;
let session = door
    .admit(&SessionBody {
        password,
        name: None,
        claim: None,
    })
    .await?;
let client = Client::new(address, session.credential)?;
```

`name` names a household member, and the operator gives none. `session.until` says
when the session stops being admitted, and `session.member` whom it is for.

## Reading

`contract::reads` holds a struct for each read the contract lists, holding what it
takes. `Client::read` answers it with an `Envelope` whose `data` is typed by the kind
the read answers with:

```rust
use lemonfiber_sdk::contract::reads::{HeldId, Status};

let status = client.read(&Status::default()).await?;
println!("{:?}", status.data.condition);

let title = client
    .read(&HeldId {
        id: "tt0133093".to_owned(),
        ..HeldId::default()
    })
    .await?;
```

A read that answers with more than one kind answers with an enum named after it, such
as `HeldIdAnswer`, holding a variant for each.

A field the path holds is sent escaped as one segment. One left empty, or given as `.`
or `..`, is `Error::Unsendable` and nothing is sent. The other fields are the query.

Every enum an answer can carry is `#[non_exhaustive]` and has an `Unrecognised` variant,
holding a value this crate was not generated with as it arrived, so a stack newer than
the crate still answers. Match it with a `_` arm. Enums a request sends list only what
the contract lists.

Some reads are not a single envelope:

- `Client::logs` takes `reads::Logs` and answers with `Logged::Lines`, an envelope a
  line, or with `Logged::Following`, a name for the work following the logs where
  `follow` was asked for.
- `Client::file` takes a read answered with a file, such as `reads::BundleName`, and
  answers with a `File`: its bytes and the type it was labelled with.
- `Client::picture` takes a read answered with a picture, such as
  `reads::HeldIdPoster`, and answers with a `Picture`. A picture is one of
  `PICTURE_TYPES` (JPEG, PNG, WebP, GIF and AVIF, never SVG) and at most `PICTURE_MOST`
  bytes, 2 MiB. Any other answer is `Error::Unreadable`.

`read`, `file`, `picture`, `logs`, `capabilities` and `job` are asked twice more where nothing answered or a
gateway in front of the stack answered 502, 503 or 504, a quarter of a second later
and then half a second, every attempt within the call's wait.

Every answer is read through its envelope. An answer in an `api_version` this crate
does not speak is `Error::VersionMismatch`, naming both.

## What a stack can do

`Client::capabilities` answers with a `CapabilitySet`: every request the stack serves,
by its path, and what it comes to for the credential that asked.

| State                           | Meaning                                                   |
| ------------------------------- | --------------------------------------------------------- |
| `CapabilityState::Available`    | You can use it                                            |
| `CapabilityState::Unconfigured` | A setting on the stack has to be turned on first          |
| `CapabilityState::Unpermitted`  | This credential may not use it, for example a key's scope |

`of`, `of_action::<A>()` and `of_read::<R>()` answer for one request, or `None` where
the stack does not have it, and `in_state` collects the paths in one state. A path this
crate does not know is kept rather than refused. `read_at` says when the set was read,
since it can change while you hold it.

`scope` says whose credential asked, and `stack` is the stack's own identifier, or
`None` where it has none. Read `scope` rather than guess it from which requests are
permitted.

## Acting

`contract::actions` holds a struct for each action, holding its arguments.
`Client::act` sends it once and never asks again, because a second sending would be a
second change:

```rust
use lemonfiber_sdk::contract::actions::Restart;
use lemonfiber_sdk::{Acted, JobStanding};

let restart = Restart {
    services: Some(vec!["sonarr".to_owned()]),
    ..Restart::default()
};
if let Acted::Started(started) = client.act(&restart).await? {
    let job = started.job.unwrap_or_default();
    let standing = client.follow(&job, Duration::from_secs(2), None).await?;
    if let JobStanding::Finished(answer) = standing {
        println!("{:?}", answer.data);
    }
}
```

`Acted::Answered` holds the outcome where the work was done by the time the answer was
written, and `Acted::Started` a name for the work that goes on after it. `Client::job`
asks where it stands and `Client::release` lets the name go, ending the work.
`Client::follow` asks every `every` until it is finished or ended; given `within`, it
is `Error::StillRunning` once that has passed. A failure is an `Error` rather than a
standing.

An action that takes `dry_run` can be rehearsed first: it says what it would do and
does nothing. `Action::REHEARSAL` says whether it takes one, and `Action::CONSENT` which
arguments carry the operator's consent.

`Client::act_once` sends an action with an `IdempotencyKey`, so that an action sent
again after no answer arrived is carried out once.

### What an integration key may call

`contract::key_callable::ALL` lists the actions a key may call. Each `KeyCallable` says
whether calling it `disturbs` the running system, whether it takes a `rehearsal`,
whether it is `idempotent`, and in `moved` the code a call is refused with when the
offer it carries back has moved since its rehearsal.

## Integration keys

A program that runs beside the stack for months holds a key rather than the per-run
token. `Client::keys` lists the keys the credential may see, without their secrets.
`Client::mint` mints one, its secret in that answer and no other, and `Client::revoke`
revokes one by its name:

```rust
use lemonfiber_sdk::contract::KeysBody;

let minting = KeysBody {
    name: "home".to_owned(),
    scope: "read".to_owned(),
    purpose: "home-assistant".to_owned(),
    password: Some(password),
    challenge: None,
    claim: None,
    device: None,
    signature: None,
};
let minted = client.mint(&minting).await?;
client.revoke("home").await?;
```

`Client::mint_claim` mints a key as a claim instead, answered with no secret but a key
link, `lemonfiber://key?...`, that collects the key once. A program holding nothing
else collects it:

```rust
use lemonfiber_sdk::{Door, KeyLink};

let link = KeyLink::new(&scanned)?;
let key = Door::collect_link(&link).await?;
```

The claim is sent alone, with no credential, over the TLS the link's fingerprint
verifies. A claim already lapsed is `Error::ClaimLapsed` and nothing is sent. A claim
the stack turns away is a refusal with `RefusedBecause::ClaimRefused`. `KeyLink`'s
`Debug` leaves the claim out, and nothing here keeps the collected secret.

### Device keys

A phone holding a key pair in its hardware registers it once with
`Client::register_device_key`, with the platform's attestation and the password. From
then on it proves a mint with a signature over a challenge the stack issued, in place of
the password:

```rust
use lemonfiber_sdk::{base64url, signed_text};

let challenge = client.device_key_challenge("pixel").await?.data.challenge;
let mut minting = KeysBody {
    name: "den".to_owned(),
    scope: "read".to_owned(),
    purpose: "mcp".to_owned(),
    password: None,
    challenge: Some(challenge.clone()),
    claim: None,
    device: Some("pixel".to_owned()),
    signature: None,
};
let text = signed_text(&minting, &challenge)?;
minting.signature = Some(base64url(&phone_signs(&text)));
client.mint(&minting).await?;
```

The signature is ECDSA P-256 over SHA-256, made by the phone. `base64url` writes bytes
as the stack reads them in a body. `Client::device_keys` lists the device keys and
`Client::revoke_device_key` revokes one.

## First-run setup

`Client::setup` reads where setup stands. `setup_answer` answers the question it is on,
`setup_next` goes on past a step that only informs, `setup_back` goes back, `setup_apply`
writes the reviewed answers, and `setup_recover` takes one way out of an apply that
stopped part-way. Each answers with where setup now stands, and each that changes
something is sent once.

## Live updates

```rust
use lemonfiber_sdk::{Arrival, Following};

let mut stream = client.events(Following::default());
loop {
    match stream.next().await? {
        Arrival::Live(envelope) => println!("{:?}", envelope.data),
        Arrival::Stale { envelope, quiet_for } => println!("{:?} {quiet_for:?} ago", envelope.kind),
        Arrival::Gap { why, quiet_for } => println!("{why:?} after {quiet_for:?}"),
        Arrival::Unrecognised(kind) => println!("{kind} is newer than this client"),
    }
}
```

- The server sends a heartbeat at least every 15 seconds (`HEARTBEAT`). Silence for 30
  seconds is a broken stream, not a quiet one.
- A broken stream is reopened from the last event it carried, sent as `Last-Event-ID`.
  It waits one second, then twice as long after each failure up to thirty seconds, and is
  `Error::StreamLost` after five failures in a row. `Following` sets all three.
- Every value held from before a gap arrives again as `Arrival::Stale`, and stays stale
  until the stream carries it again. `stream.held()` says where each kind stands.
- A refused credential, a refused certificate or an answer in another `api_version` is
  an `Error` at once, not tried again.
- An event longer than 16 MiB is `Error::Unreadable`.

## Errors

Every failure is an `Error`, and its `Display` is a sentence you can show a person.
When lemonfiber turns a request down, it is `Error::Refused`, holding a `Refusal`.
`Refusal::because` is read from the code where the contract lists one and from the
status where it does not:

| `RefusedBecause`  | Meaning                                                                            |
| ----------------- | ---------------------------------------------------------------------------------- |
| `NotAdmitted`     | The credential is absent, wrong, expired, revoked or from another run              |
| `Declined`        | lemonfiber objects to who is asking or where from; a new credential would not help |
| `Missing`         | lemonfiber has nothing by the name the request gave                                |
| `Misasked`        | lemonfiber could not answer the request as it was asked                            |
| `Busy`            | Other work held the stack. The same request may be sent again once it is done      |
| `TooManyAttempts` | Too many wrong attempts lately. `retry_after` says how many seconds, where known   |
| `Failed`          | Nothing about the request was wrong; the machine could not answer it               |
| `PasswordRefused` | The password offered at the door was wrong, or none is configured                  |
| `ClaimRefused`    | The claim collects no key: none waits on it, it was collected, or it lapsed        |

A `Refusal` also carries the `status`, the `code` and the `problem`. Decide from
`because` or `code`, never from `sentence`. `contract::refusals` holds a constant for
each code the contract lists, `contract::refusals::ALL` lists them, and
`RefusalCode::listed` finds one by its code.

The other errors you will meet most are `Error::Unreachable` (nothing answered, or what
answered was not lemonfiber), `Error::CertificateRefused` (the stack presented another
certificate), `Error::TimedOut`, `Error::AddressRefused` and `Error::Unsendable` (nothing
was sent) and `Error::UnknownKind` (the stack is newer than this client).

## Your own transport

`Client::new` reaches the stack through `Http`, on `reqwest` and `rustls` with the
`ring` provider. It follows no redirect and takes no proxy from the environment.
`Client::with_transport` and `Door::with_transport` take your own `Transport` for a stack
on this machine:

```rust
use lemonfiber_sdk::{Failure, Opened, Request, Response, Transport};

struct Recorded;

impl Transport for Recorded {
    async fn send(&self, request: Request) -> Result<Response, Failure> {
        Err(Failure::Unreachable(request.url))
    }

    async fn open(&self, request: Request) -> Result<Opened, Failure> {
        Err(Failure::Unreachable(request.url))
    }
}
```

An address with a pin refuses a transport with `Error::AddressRefused`: a stack off this
machine is reached only through `Http`, which checks its pin. The address, the credential
header and the reading of the answer are the client's, whatever the transport.

## Where the types come from

`crates/lemonfiber-sdk/src/contract/` is generated from the contract vendored under
`contract/web-api/`, which lemonfiber builds from the Rust types that produce its
answers. Nothing there is edited by hand, and `contract/VERSION` names the lemonfiber
commit the copy came from.

```sh
just sync <tag-or-commit>   # vendor the contract at that revision, and regenerate
just generate               # rewrite the module from the vendored copy
```

Shapes are written by the first letter of their name, reads and actions likewise, and
refusals by family. `contract`, `contract::reads`, `contract::actions` and
`contract::refusals` re-export them, whichever file holds them.
