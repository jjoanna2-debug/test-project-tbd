# Roadmap

Status reviewed: **2026-08-17**.

This roadmap records the repository's completed baseline and reserves future work for concrete use cases. Completed phases describe implemented, tested behavior. Optional phases are not commitments or unfinished obligations.

## Phase 1 — Repository Foundations

Status: **complete**

- Maintain a current README and beginner map.
- Keep licensing, legal notices, disclaimer, security, support, contribution, conduct, sponsorship, and funding documents aligned.
- Use structured issue forms and a pull request template.
- Keep editing, Git, and Markdown configuration explicit.

## Phase 2 — Rust-First Project

Status: **complete**

- Provide a minimal Rust starter package.
- Use Edition 2024 with an exact Rust `1.97.1` toolchain.
- Keep the package dependency-free and non-publishable.
- Forbid unsafe Rust and deny debug macros, `todo!`, and `unimplemented!` paths.
- Preserve a locked dependency state for local and CI validation.

## Phase 3 — Protected GitHub Workflow

Status: **complete**

- Protect `main` with the required `Repository smoke test`.
- Validate pull requests and pushes to `main`.
- Validate merge-queue groups and allow manual dispatches.
- Cancel superseded runs for the same pull request or ref.
- Pin third-party Actions to immutable lowercase commit SHAs.
- Require immutable lowercase SHA-256 digests for Docker actions.
- Disable persisted checkout credentials and keep workflow permissions read-only.
- Reject unapproved trigger surfaces and YAML authorization indirection.
- Treat compiler, Clippy, and rustdoc warnings as failures.
- Validate formatting, all targets and features, tests, documentation, and repository policy.

## Phase 4 — Repository Doctor

Status: **complete**

- Build one deterministic repository inventory per run.
- Use iterative traversal with a 20,000-entry ceiling.
- Reject symlinks and surface unreadable filesystem state.
- Limit scannable text reads to 4 MiB, including required-file reference checks.
- Validate up to 64 KiB from declared binary files rather than trusting extensions alone.
- Recognize common binary magic prefixes before skipping content.
- Reject undeclared binary content, invalid UTF-8, and UTF-8 text hidden behind binary suffixes.
- Validate required files, source invariants, redacted evidence paths, sensitive filenames, credential stores, and key containers.
- Split content, secret, workflow, and output responsibilities into focused Rust modules.
- Parse GitHub Actions policy in context rather than through blind substring matching.
- Run the doctor locally and as part of the protected CI gate.

## Phase 5 — Classifier Calibration

Status: **complete**

- Maintain a dedicated labeled calibration corpus rather than embedding a handful of examples beside the classifier.
- Cover realistic secret assignments, provider-token shapes, hard negatives, placeholders, misleading key names, documentation examples, comments, escaped strings, multiline quoted values, and YAML literal and folded blocks.
- Construct provider-shaped probes at runtime so the repository does not store credential-like literals.
- Parse quoted, unquoted, multiline, escaped, literal-block, and folded-block assignments within explicit line and byte limits.
- Cover current GitHub, GitLab, OpenAI, Slack, Stripe, npm, Google, SendGrid, AWS access-key, and private-key signals.
- Reject repeated, sequential, low-diversity, placeholder, example-host, environment-reference, and prose fixtures before they can inflate confidence.
- Restrict colon-based assignment parsing to valid configuration keys so Rust type annotations, Markdown code, and quoted test literals do not masquerade as credentials.
- Measure average precision, precision, recall, and F1 at the active score threshold.
- Require at least `0.98` average precision and `0.95` precision, recall, and F1 on the maintained internal corpus.
- Preserve one highest-scoring generic assignment finding per file to control duplicate noise.

These metrics protect regression behavior on the repository's maintained corpus. They are not claims about an external production dataset.

## Phase 6 — Structured Diagnostics

Status: **complete**

- Preserve concise human-readable output for local use.
- Expose deterministic JSON findings with stable path, line, and message fields.
- Emit native GitHub Actions error annotations for file- and line-aware CI diagnostics.
- Select GitHub annotation output automatically inside GitHub Actions while keeping text as the local default.
- Support explicit `--format text`, `--format json`, and `--format github` selection.
- Provide `--help` and fail on unknown or ambiguous command-line arguments.
- Escape JSON and GitHub workflow-command data correctly.
- Keep output handling dependency-free and covered by unit tests.

## Phase 7 — Optional Future Direction

Status: **optional**

Proceed only when a real consumer exists:

- benchmark large-but-valid repositories within the existing resource ceilings;
- separate reusable library code from binary entry points if a second consumer appears;
- add configuration only when fixed policy is no longer sufficient;
- extend the calibration corpus when provider formats or realistic hard negatives change;
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

Update this roadmap when an implemented capability changes the baseline or an optional item acquires a concrete consumer. Do not leave completed work labeled “in progress,” which is how roadmaps become fiction with headings.

## Boundaries

This roadmap does not create support, maintenance, delivery, feature, sponsorship, or service obligations. It is a current planning document for this repository only.
