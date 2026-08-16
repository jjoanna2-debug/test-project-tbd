# Roadmap

Status reviewed: **2026-08-16**.

This roadmap tracks the repository as it grows from a clean practice project into a small Rust-first staging lab. Completed phases describe the current baseline. Future phases are optional until a concrete use case justifies them.

## Phase 1 — Repository Foundations

Status: **complete**

- Maintain a clear current README and beginner map.
- Keep licensing, legal notices, disclaimer, security, support, contribution, conduct, sponsorship, and funding documents aligned.
- Use structured issue forms and a pull request template.
- Keep editing, Git, and Markdown configuration explicit.

## Phase 2 — Rust-First Project

Status: **complete**

- Provide a minimal Rust starter package.
- Pin the minimum supported Rust version in `Cargo.toml`.
- Keep the package dependency-free and non-publishable.
- Forbid unsafe Rust and enforce strict Clippy policy.
- Preserve a locked dependency state for local and CI validation.

## Phase 3 — Protected GitHub Workflow

Status: **complete**

- Protect `main` with the required `Repository smoke test`.
- Validate pull requests and pushes to `main`.
- Validate merge-queue groups and allow manual dispatches.
- Cancel superseded runs for the same pull request or ref.
- Pin third-party Actions to immutable commit SHAs.
- Disable persisted checkout credentials and keep workflow permissions read-only.

## Phase 4 — Repository Doctor

Status: **complete**

- Build one deterministic repository inventory per run.
- Use iterative traversal with a 20,000-entry ceiling.
- Reject symlinks and surface unreadable filesystem state.
- Limit non-binary text reads to 4 MiB, including required-file reference checks.
- Validate required files, source invariants, redacted evidence paths, sensitive filenames, credential stores, and key containers.
- Split secret and workflow checks into focused Rust modules.
- Parse GitHub Actions policy in context rather than through blind substring matching.
- Run the doctor locally and as part of the protected CI gate.

## Phase 5 — Classifier Calibration

Status: **in progress**

Current baseline:

- Provider-specific signatures cover GitHub, GitLab, OpenAI, Slack, Stripe, npm, Google, SendGrid, and AWS access-key shapes.
- Generic secret assignments are scored from key semantics and value characteristics.
- Placeholder, redaction, environment-reference, and low-entropy suppression reduce obvious false positives.
- A labeled internal corpus enforces average precision of at least `0.95`.

Next useful work:

- Expand the labeled corpus with more realistic positive and hard-negative cases.
- Measure precision and recall at the active score threshold in addition to average precision.
- Add cases for multiline configuration, escaped values, adjacent comments, and common documentation examples.
- Calibrate provider signatures against false-positive fixtures without weakening exact token-shape detection.
- Keep finding output concise by preserving per-file deduplication.

## Phase 6 — Useful CLI Direction

Status: **optional**

Proceed only when a real use case exists:

- expose structured output suitable for CI annotations or machine consumption;
- separate reusable library code from binary entry points if a second consumer appears;
- benchmark large-but-valid repositories within the existing resource ceilings;
- add configuration only when fixed policy is no longer sufficient;
- evaluate dependencies only when measured value exceeds maintenance and supply-chain cost.

## Deliberate Non-Goals

The repository is not currently pursuing:

- production deployment;
- hosted services or network APIs;
- credential storage or secret management;
- automatic remediation of detected findings;
- claims of external production-grade classifier performance;
- framework adoption for its own decorative pleasure.

## Review Rule

Update this roadmap when a phase changes status, an implemented capability changes the current baseline, or a future item becomes obsolete. Do not leave completed work labeled “in progress,” which is how roadmaps become fiction with headings.

## Boundaries

This roadmap does not create support, maintenance, delivery, feature, sponsorship, or service obligations. It is a current planning document for this repository only.
