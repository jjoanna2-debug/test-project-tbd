# Contributing

Status reviewed: **2026-08-16**.

## Project Status

This repository is a personal learning and testing project. Contributions are welcome for consideration, but they are not guaranteed to be reviewed, accepted, merged, maintained, supported, credited beyond GitHub's normal attribution, or answered.

## Before Contributing

By opening an issue, pull request, discussion, comment, or other contribution, you confirm that:

- the contribution is your own work or you have the right to submit it;
- it does not contain secrets, credentials, private keys, tokens, passwords, personal data, customer data, confidential information, regulated information, or third-party material you are not authorized to share;
- any evidence artifact has been reviewed and explicitly redacted before publication;
- the contribution may be modified, rejected, closed, deleted, or ignored at the repository owner's discretion;
- the contribution is submitted under the same Apache License 2.0 terms that apply to this repository unless clearly agreed otherwise in writing.

## Use a Branch and Pull Request

Do not submit routine work directly to protected `main`.

```bash
git switch main
git pull --ff-only origin main
git switch -c contribution/describe-the-change
```

Keep one coherent change per branch. Review the diff before staging and avoid unrelated formatting, generated files, backups, or drive-by rewrites.

## Required Local Validation

For code, workflow, configuration, or doctor-policy changes, run:

```bash
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo run --quiet --locked --bin check_repo
```

For documentation-only changes, the Rust commands still provide the authoritative repository gate because the doctor validates required documentation and policy invariants. At minimum, review:

```bash
git diff --check
git diff
bash scripts/doctor.sh
```

The protected `Repository smoke test` remains the final merge requirement.

## Pull Request Requirements

A pull request should state:

- what changed;
- why it changed;
- how it was verified;
- security, privacy, data, compatibility, and operational impact;
- documentation impact;
- deliberate exclusions, limitations, or follow-up work.

Update current-state documentation in the same pull request when behavior, commands, architecture, file layout, policies, automation, supported tooling, roadmap status, or evidence handling changes.

## No Sensitive Information

Do not submit:

- API keys, access tokens, passwords, private keys, or credentials;
- live `.env` files, root credential stores, key containers, or password databases;
- personal, customer, confidential, regulated, or production data;
- harmful third-party exploit payloads;
- unredacted screenshots, logs, recordings, exports, or evidence bundles;
- material that violates another person's rights or applicable law.

The doctor catches common patterns and paths, not every possible disclosure. Passing automation is not permission to publish material you have not reviewed.

## Evidence and GitHub Releases

In-tree evidence must live under an explicitly redacted path. GitHub Release assets are outside the checked-out repository tree and are not inspected by the local doctor.

Before publishing or changing a release-asset bundle:

1. Review every asset manually.
2. Remove account identifiers, credentials, private data, and unrelated information.
3. Use a tag and release name that identifies the external issue and states that the assets are evidence.
4. Do not present an evidence tag as a software version or supported release.

## Scope and Design

Prefer changes that are:

- focused and reversible;
- dependency-free unless a dependency has clear measured value;
- consistent with the Rust-first design;
- deterministic and resource-bounded;
- accompanied by regression coverage when behavior changes;
- honest about internal metrics and external limitations.

The repository does not need more scaffolding merely because scaffolding can be generated. Civilization has suffered enough YAML already.

## Review and Maintenance Boundaries

There is no guaranteed review, response, merge, fix, release, support, or maintenance timeline. A contribution does not create an obligation or commercial relationship.

For related terms, see [DISCLAIMER.md](DISCLAIMER.md), [SECURITY.md](SECURITY.md), [SUPPORT.md](SUPPORT.md), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), and [LICENSE](LICENSE).
