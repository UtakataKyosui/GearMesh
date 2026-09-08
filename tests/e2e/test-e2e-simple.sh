#!/usr/bin/env bash
# Both legacy entrypoints now exercise the same real macro fixture.
set -euo pipefail
repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
exec bash "$repo_root/tests/e2e/test-e2e.sh"
