# Task runner for lemonfiber/sdk-rust. `just` with no argument lists the tasks.
default:
    @just --list

# Turn on the repository's own git hooks. Once per clone; `ci` does it too.
hooks:
    git config core.hooksPath .githooks
    @echo "hooks on: .githooks/commit-msg, .githooks/pre-push"

# What the `check`, `deny` and `contract` jobs and the coverage gate run. The read list
# needs a spec checkout (`just reads ../spec`). Mutation testing, backward compatibility
# and the shared workflows are CI's.
ci: hooks fmt clippy contract-check deny coverage

# Formatting, changing nothing.
fmt:
    cargo fmt --all --check

# Every lint, every warning an error.
clippy:
    cargo clippy --all-targets --workspace --locked -- -D warnings

# The suite alone.
test *args:
    cargo test --workspace --locked {{args}}

# The suite with every line outside the generator's entry point run, as the coverage gate
# holds it.
coverage:
    cargo llvm-cov --workspace --locked --ignore-filename-regex 'src/main\.rs' --fail-under-lines 100 --show-missing-lines

# The dependency policy: licences, sources, advisories and duplicate versions.
deny:
    cargo deny --log-level warn --all-features check

# Rewrite the generated contract module from the vendored artefact.
generate:
    cargo run --quiet --locked -p lemonfiber-sdk-generate -- generate

# Fail where the generated module differs from what the vendored artefact generates.
contract-check:
    cargo run --quiet --locked -p lemonfiber-sdk-generate -- check

# Hold the reads this client reaches to the `## Reading` list of the contract page in a
# spec checkout.
reads spec="../spec":
    cargo run --quiet --locked -p lemonfiber-sdk-generate -- reads --spec {{spec}}

# Vendor the contract at a release tag or a full commit hash of lemonfiber into
# `contract/`, then regenerate. The one task that reaches the network.
sync revision:
    #!/usr/bin/env bash
    set -euo pipefail
    taken="$(mktemp -d)"
    trap 'rm -rf "${taken}"' EXIT
    curl --fail --silent --show-error --location --max-redirs 0 --proto '=https' \
        "https://codeload.github.com/lemonfiber/lemonfiber/tar.gz/{{revision}}" \
        | tar -xz -C "${taken}"
    cargo run --quiet --locked -p lemonfiber-sdk-generate -- take "${taken}"/*/contract/web-api "{{revision}}"
    just generate

# The public surface against the commit the core pins. Needs `cargo-semver-checks` and `gh`.
bc base="origin/main" title="":
    scripts/backward_compat.sh {{quote(base)}} {{quote(title)}}

# Mutation testing, every mutant told apart by the suite or timed out. Slow: CI runs it on
# every pull request that changes code.
mutation:
    #!/usr/bin/env bash
    set -uo pipefail
    cargo mutants --package lemonfiber-sdk --no-shuffle --colors never --jobs "$(nproc)"
    status=$?
    if [ "${status}" -ne 0 ] && [ "${status}" -ne 3 ]; then
      exit "${status}"
    fi
    judged="$(jq '.caught + .timeout' mutants.out/outcomes.json)"
    echo "${judged} mutants told apart."
    [ "${judged}" -gt 0 ]

# Rewrite CHANGELOG.md from the commit history.
changelog:
    git-cliff -o CHANGELOG.md
