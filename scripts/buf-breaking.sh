#!/usr/bin/env bash
# Run buf breaking against main, or report that no released schema exists yet.
set -euo pipefail

BASELINE_REF="${BUF_BASELINE_REF:-main}"
BUF="${BUF:-buf}"

if ! git cat-file -e "${BASELINE_REF}:quack-proto/buf.yaml" 2>/dev/null; then
  echo "no baseline yet (${BASELINE_REF} has no quack-proto/buf.yaml)"
  exit 0
fi

"$BUF" breaking quack-proto --against ".git#branch=${BASELINE_REF},subdir=quack-proto"
