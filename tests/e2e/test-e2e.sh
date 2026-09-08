#!/usr/bin/env bash
# Compatibility entrypoint for the real derive -> TypeScript check.
set -euo pipefail
repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"
exec npm run test:codegen
