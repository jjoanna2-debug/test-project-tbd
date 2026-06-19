#!/usr/bin/env bash
# Copyright 2026 Jean-Claude Joanna
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$repo_root"

cargo run --locked --bin check_repo

echo "Doctor check passed."
