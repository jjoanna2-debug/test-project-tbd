# Security Policy

Status reviewed: **2026-08-16**.

## Project Status

This is a public personal learning and testing repository. It is not a production system, supported security product, audited codebase, managed service, secret-management system, or hardened deployment.

The package is marked `publish = false` and is not distributed as a supported Rust crate.

## Supported Versions and Tags

No branch, commit, package version, tag, release, fork, artifact, or file receives formal security support.

| Surface | Current status |
| --- | --- |
| Protected `main` | Active development branch; no formal support |
| Package version `0.1.0` | Learning version; unpublished and unsupported |
| `codex-issue-22773-assets` | Evidence-asset tag, not a software release |
| `codex-issue-23192-assets` | Evidence-asset tag, not a software release |
| Forks and historical commits | Unsupported |

The two GitHub Releases contain public redacted evidence for external Codex issue reports. They do not represent supported software versions, stable builds, or compatibility commitments.

## Current Guardrails

The repository doctor and protected `Repository smoke test` currently enforce or verify:

- required repository, source, policy, documentation, workflow, and automation invariants;
- Rust formatting, strict all-target/all-feature Clippy, and locked all-target/all-feature tests;
- forbidden unsafe Rust;
- one deterministic repository inventory;
- iterative traversal capped at 20,000 visited entries;
- rejection of repository symlinks;
- a 4 MiB limit on non-binary text reads, including required-file reference checks;
- explicit failures for unreadable directories, entries, metadata, and files;
- sensitive filenames, root credential stores, key containers, password-manager databases, and live environment files;
- private-key blocks, AWS access-key shapes, provider token shapes, and scored generic secret assignments;
- placeholder, environment-reference, redaction, and low-entropy suppression;
- explicitly redacted names for in-tree evidence artifacts;
- explicit top-level workflow permissions;
- rejection of `write-all`, `contents: write`, and `pull_request_target`;
- disabled checkout credential persistence;
- full commit-SHA pins for third-party GitHub Actions;
- SHA-256 digests for Docker actions;
- protected workflow and Dependabot configuration invariants.

These controls are guardrails only. They are not proof that the repository is secure, vulnerability-free, complete, correctly configured on every platform, or suitable for sensitive use.

## Coverage Boundaries

The repository doctor inspects the current checked-out filesystem tree. It does not automatically inspect:

- Git history or deleted commits;
- forks or external clones;
- GitHub Release assets;
- external services, accounts, or deployment environments;
- live repository settings that are not represented by checked files;
- secrets already copied, cached, indexed, downloaded, or exposed elsewhere;
- every possible credential format, obfuscation, encoding, or vulnerability class.

Release assets require separate review before publication because they are not present in the checked-out repository tree.

## Security Expectations

Do not use this repository with:

- production credentials, passwords, keys, tokens, or secrets;
- personal, customer, confidential, regulated, or production data;
- business-critical, safety-critical, privacy-sensitive, or security-sensitive systems;
- any environment where failure, incorrect detection, false reassurance, or data exposure could cause harm.

Review, test, validate, and harden anything independently before use.

## Reporting a Security Issue

Never include a live secret, private key, credential, personal data, confidential material, or harmful third-party exploit payload in a public issue, pull request, discussion, comment, or screenshot.

If GitHub presents a private vulnerability-reporting interface for this repository, prefer it for non-public vulnerability details. Otherwise, open a minimal public issue that identifies the affected file or control without publishing sensitive proof, active credentials, or harmful exploitation details.

There is no guaranteed response time, remediation deadline, disclosure process, maintenance commitment, or support obligation.

## Accidental Secret Exposure

If a credential or secret is exposed:

1. Revoke or rotate it immediately.
2. Review access logs and downstream use where available.
3. Remove the material from the current branch and related public surfaces.
4. Assess whether Git history, release assets, caches, forks, downloads, or indexes still contain it.
5. Do not assume deletion of one file or commit has invalidated the exposed value.

## No Security Warranty

This repository is provided "as is" and may contain incomplete, outdated, insecure, vulnerable, placeholder, experimental, or intentionally simplified material. See [DISCLAIMER.md](DISCLAIMER.md), [SUPPORT.md](SUPPORT.md), and [LICENSE](LICENSE).
