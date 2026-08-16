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

The `codex-issue-22773-assets` release metadata explicitly identifies redacted screenshots. The `codex-issue-23192-assets` release is public screenshot evidence. Neither release represents a supported software version, stable build, or compatibility commitment, and the local doctor does not inspect or attest to either release's asset contents.

## Current Guardrails

The repository doctor and protected `Repository smoke test` currently enforce or verify:

- required repository, source, policy, documentation, workflow, toolchain, and automation invariants;
- Rust formatting, compilation, strict all-target/all-feature Clippy, locked all-target/all-feature tests, and documentation compilation;
- exact Rust `1.97.1`, Edition 2024, and forbidden unsafe Rust;
- one deterministic repository inventory;
- iterative traversal capped at 20,000 visited entries;
- rejection of repository symlinks;
- a 4 MiB limit on non-binary text reads, including required-file reference checks;
- explicit failures for unreadable directories, entries, metadata, and files;
- sensitive filenames, root credential stores, key containers, password-manager databases, and live environment files;
- private-key blocks, AWS access-key shapes, provider token shapes, and scored generic secret assignments;
- quoted, unquoted, escaped, multiline, YAML literal-block, and YAML folded-block assignments within explicit line and byte limits;
- exact secret-key semantics and valid configuration-key grammar;
- placeholder, environment-reference, example-host, redaction, prose, repeated-pattern, sequential-pattern, and low-diversity suppression;
- a maintained labeled corpus covering realistic positives, hard negatives, provider probes, documentation examples, comments, multiline values, YAML blocks, escaped strings, and misleading key names;
- internal calibration floors of `0.98` average precision and `0.95` precision, recall, and F1 at the active threshold;
- explicitly redacted names for in-tree evidence artifacts;
- explicit top-level workflow permissions;
- rejection of `write-all`, `contents: write`, and `pull_request_target`;
- disabled checkout credential persistence;
- full commit-SHA pins for third-party GitHub Actions;
- SHA-256 digests for Docker actions;
- protected workflow and Dependabot configuration invariants.

Provider-shaped calibration probes are constructed at runtime so the repository does not store credential-like literals solely for testing.

These controls are guardrails only. The calibration metrics describe the maintained internal corpus. They do not prove that the repository is secure, vulnerability-free, complete, correctly configured on every platform, or suitable for sensitive use.

## Coverage Boundaries

The repository doctor inspects the current checked-out filesystem tree. It does not automatically inspect:

- Git history or deleted commits;
- forks or external clones;
- GitHub Release assets;
- external services, accounts, or deployment environments;
- live repository settings that are not represented by checked files;
- secrets already copied, cached, indexed, downloaded, or exposed elsewhere;
- every possible credential format, obfuscation, encoding, or vulnerability class;
- external production datasets or provider-side validity.

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
