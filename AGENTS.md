# AGENTS.md — sdk-rust

> **Start at the roadmap and board on [lemonfiber.app](https://lemonfiber.app),
> rendered from the report of where every unreleased version stands. Then the
> rules** every repository shares:
> [working in the repositories](https://github.com/lemonfiber/spec/blob/main/50-governance/working-in-the-repositories.md)
> and [the rules for agents](https://github.com/lemonfiber/spec/blob/main/50-governance/ai-contributors.md).
> This file holds only what is true of this repository.

## What this repo is

The Rust client for lemonfiber's web API, the crate `lemonfiber-sdk`, and the generator
that writes its contract module, `lemonfiber-sdk-generate`. A library with **no user
interface and no server**. Spec:
[`30-repos/sdk-rust.md`](https://github.com/lemonfiber/spec/blob/main/30-repos/sdk-rust.md)
and the
[web API contract](https://github.com/lemonfiber/spec/blob/main/20-architecture/contracts/web-api.md).

It is a **peer** of [`sdk-ts`](https://github.com/lemonfiber/sdk-ts),
[`sdk-php`](https://github.com/lemonfiber/sdk-php) and
[`sdk-python`](https://github.com/lemonfiber/sdk-python), not a translation of any of
them. A client that disagrees with the contract is wrong.

## The rules you cannot break

- **`crates/lemonfiber-sdk/src/contract/` is not edited by hand.** It is generated from
  the contract vendored under `contract/` (`ARCH-R56`, `ARCH-R58`). `just contract-check`
  fails on any difference, a file added or removed included.
- **The workspace lints are the core's**: `unsafe_code` forbidden, `missing_docs` denied,
  clippy's pedantic set, and `unwrap`, `expect`, `panic` and indexing denied. No
  `#[allow]` is a fix.
- **100% line coverage**, as `just coverage` measures it. A closure or a branch no test
  reaches is a line the gate counts as missed.
- **Dependencies are held to the versions the core's lockfile resolves**, so the core
  takes this crate without a second copy of anything; `cargo deny` refuses a duplicate.
- **The token is a header, never a URL** (`ARCH-R52`); **loopback, or an address a
  certificate pin vouches for** (`ARCH-R60`, `ARCH-R99`). A pinned stack is reached
  through `Http` alone.
- **No explanatory comments.** A `///` doc is written where a lint asks for one, and
  says what the item is.

## Checks

```
just ci              # fmt, clippy, the contract check, cargo-deny, coverage
just test            # the suite alone
just reads ../spec   # the reads held to the contract page in a spec checkout
```

## Before you open a PR

- `just ci` is clean.
