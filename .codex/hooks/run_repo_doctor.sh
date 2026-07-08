#!/usr/bin/env bash
# Copyright 2026 Jean-Claude Joanna
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

hook_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$hook_dir/../.." && pwd)"

cd "$repo_root"

bash scripts/doctor.sh
