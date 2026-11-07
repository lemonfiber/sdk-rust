#!/usr/bin/env bash
# Holds this tree's public surface to the commit the core pins, or to the base where it
# pins none. A break the base already makes is the base's, and a title with `!` after its
# type declares one. Exits 0 where nothing new breaks or the title declares it, 1 where
# something new breaks undeclared, and 2 where the comparison could not finish.
#
#   scripts/backward_compat.sh <base commit> [<pull request title>]
set -euo pipefail

CONSUMER=lemonfiber/lemonfiber
SOURCE=git+https://github.com/lemonfiber/sdk-rust
UNFINISHED=2

base="${1:?the base commit, then the title}"
title="${2-}"
work="$(mktemp -d)"
trap 'git worktree remove --force "${work}/base" 2>/dev/null || true; rm -rf "${work}"' EXIT

compared() {
  local directory="$1" against="$2" named="$3" status=0
  (cd "${directory}" && cargo semver-checks --package lemonfiber-sdk --baseline-rev "${against}" --release-type minor --color never) \
    >"${work}/${named}.out" 2>&1 || status=$?
  if ! grep -q 'Summary' "${work}/${named}.out"; then
    cat "${work}/${named}.out"
    echo "::error::The comparison of the ${named} against ${against} did not finish."
    exit "${UNFINISHED}"
  fi
  awk '
    /^--- failure / { lint = $3; sub(/:$/, "", lint) }
    /^Failed in:/ { listing = 1; next }
    /^$/ { listing = 0 }
    listing {
      line = $0
      sub(/^ +/, "", line)
      sub(/, previously in file .*$/, "", line)
      sub(/ in (file )?[^ ]+$/, "", line)
      print lint ": " line
    }
  ' "${work}/${named}.out" | sort -u >"${work}/${named}.txt"
  if [ "${status}" -ne 0 ] && [ ! -s "${work}/${named}.txt" ]; then
    cat "${work}/${named}.out"
    echo "::error::The comparison of the ${named} against ${against} failed, and named no break."
    exit "${UNFINISHED}"
  fi
}

if ! lock="$(gh api "repos/${CONSUMER}/contents/Cargo.lock" -H 'Accept: application/vnd.github.raw')"; then
  echo "::error::The lockfile of ${CONSUMER} could not be read, so what it pins is not known."
  exit "${UNFINISHED}"
fi
pin="$(printf '%s\n' "${lock}" | grep -oE "${SOURCE}[^\"]*#[0-9a-f]{40}" | sed 's/.*#//' | sort -u | tail -n 1 || true)"
if [ -z "${pin}" ]; then
  echo "${CONSUMER} pins no commit of this repository yet, so the surface is held to the base."
  pin="$(git rev-parse "${base}")"
fi
if ! git cat-file -e "${pin}^{commit}" 2>/dev/null; then
  echo "::error::${CONSUMER} pins ${pin}, which is not a commit of this repository."
  exit "${UNFINISHED}"
fi

echo "Comparing against ${pin}."
compared . "${pin}" head
: >"${work}/base.txt"
if [ -s "${work}/head.txt" ] && [ "$(git rev-parse "${base}")" != "${pin}" ]; then
  git worktree add --quiet --detach "${work}/base" "${base}"
  compared "${work}/base" "${pin}" base
fi

added="$(comm -13 "${work}/base.txt" "${work}/head.txt")"
if [ -z "${added}" ]; then
  echo "Nothing breaks against ${pin} that the base did not already break."
  exit 0
fi
printf '%s\n' "${added}"
declared="$(printf '%s\n' "${title}" | grep -E '^[a-z]+(\([a-z0-9.-]+\))?!: ' || true)"
if [ -n "${declared}" ]; then
  echo "::notice::This breaks what ${pin} exposes, and the title says so."
  exit 0
fi
cat "${work}/head.out"
echo "::error::This breaks what ${pin} exposes. Declare it with '!' in the title, for example 'feat!: ...'."
exit 1
