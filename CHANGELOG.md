# Changelog

All notable changes to this repository will be documented in this file.

This project follows a simple chronological changelog. It is a learning repository, not a formal release-managed product.

## [Unreleased]

### Added

- Starter public repository structure.
- Beginner onboarding documentation.
- Rust starter project.
- Rust and shell checks for repository structure.
- Apache-2.0 `NOTICE` file for copyright attribution.
- Project structure documentation in `docs/PROJECT_STRUCTURE.md`.
- Local setup guide in `docs/LOCAL_SETUP.md`.
- GitHub workflow guide in `docs/GITHUB_WORKFLOW.md`.
- License, legal notices, disclaimer, security, support, contribution, conduct, and sponsorship policy files.
- Basic issue templates for bugs, feature requests, and documentation tasks.
- Pull request template.
- Basic GitHub Actions repository smoke test.
- Weekly Dependabot checks for GitHub Actions and Cargo.
- `CODEOWNERS`, `.editorconfig`, and `.gitattributes` for repository hygiene.
- Roadmap for future learning steps.
- Practice issues for branch workflow and future project direction.
- Repository doctor checks for common secret patterns, sensitive filenames, workflow permissions, pinned GitHub Actions, and explicitly redacted evidence artifacts.
- Rust-native repository doctor binary so the repo remains Rust-first.

### Changed

- Improved repository documentation and public-use boundaries.
- Updated README and starter guide to reflect the expanded repository structure.
- Expanded the basic GitHub Actions smoke test to verify the current file layout.
- Replaced the static web starter with a Rust-first staging project.
- Clarified that funding metadata is inactive until GitHub exposes active funding links.
- Hardened repository checks for locked Cargo validation, unsafe-code rejection, secret-pattern screening, pinned workflow actions, and explicitly redacted evidence artifacts.
- Kept local backup and delegated-work directories out of Git and the repository
  doctor so preserved external checkouts cannot create false secret findings.
- Expanded sensitive-filename checks to cover environment variants, credential
  manifests, private-key files, and key-container formats.
- Collapsed artifact, secret, and workflow checks into one deterministic file
  inventory so the doctor no longer walks the repository repeatedly or rereads
  workflow files.
- Rejected repository symlinks instead of silently omitting them from security
  checks, and normalized binary-extension handling across letter case.
- Split secret and workflow classifiers into focused Rust modules, then replaced
  broad substring matching with scored key-aware assignment parsing, provider
  token signatures, placeholder suppression, and an average-precision floor.
- Allowed documented environment-template variants while blocking common root
  credential stores and password-manager databases.
- Made workflow permission checks context-aware and required Docker actions to
  use immutable SHA-256 digests.
- Reconciled the roadmap with completed issue and pull request practice.
- Promoted the Rust-native repository doctor in the README with current checks and sample passing output.

### Notes

- This repository is experimental and provided for learning, testing, and GitHub workflow practice.
- Earlier closed issues may mention the previous static-page starter. The current repository has pivoted to a Rust-first staging lab with a Rust-native repository doctor.
