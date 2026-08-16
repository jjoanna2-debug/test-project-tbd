# Pull Request

## Summary

Describe the completed change in concrete terms.

## Why

Explain the problem, stale behavior, risk, or maintenance cost this pull request addresses.

## Scope

List the files or subsystems intentionally changed. State anything deliberately excluded.

## Verification

Check every command that applies:

- [ ] `cargo fmt --check`
- [ ] `cargo check --locked --all-targets --all-features`
- [ ] `cargo clippy --locked --all-targets --all-features -- -D warnings`
- [ ] `cargo test --locked --all-targets --all-features`
- [ ] `cargo doc --locked --no-deps --document-private-items`
- [ ] `cargo run --quiet --locked --bin check_repo`
- [ ] `git diff --check`

Describe any additional test cases, failure-path checks, or manual verification:

## Impact Review

- [ ] No new runtime dependency
- [ ] No workflow permission increase
- [ ] No credential, secret, personal-data, confidential-data, or production-data exposure
- [ ] No unredacted in-tree or GitHub Release evidence asset
- [ ] No unsupported software-release claim for an evidence tag
- [ ] Resource bounds remain explicit and enforced
- [ ] Backward compatibility or intentional breakage is explained

Explain any checked item that does not apply or any impact that needs review:

## Documentation

- [ ] Current-state documentation was updated in this pull request
- [ ] Review dates remain accurate
- [ ] README, roadmap, changelog, policy, workflow, and structure references remain consistent
- [ ] No documentation change was required, with the reason stated below

Documentation note:

## Risks and Rollback

Describe the strongest plausible failure mode and the clean rollback path.

## Follow-Up

List only genuinely deferred work. Do not use this section as a landfill for pieces required to make the current change correct.

## Final Checklist

- [ ] The branch contains one coherent change
- [ ] The final diff was reviewed, not merely the commit list
- [ ] Unrelated files and generated artifacts were excluded
- [ ] The pull request is ready for the protected `Repository smoke test`
- [ ] I understand that submission does not create a review, merge, support, or maintenance obligation
