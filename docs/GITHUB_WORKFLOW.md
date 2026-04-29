# GitHub Workflow

This guide explains the basic workflow this repository is meant to teach.

## 1. Create a Branch

A branch is a safe place to make changes before merging them into `main`.

```bash
git checkout -b practice/my-first-change
```

## 2. Make a Small Change

Edit a small file, such as:

```text
index.html
src/styles.css
README.md
```

Keep the change small so it is easy to review.

## 3. Check What Changed

```bash
git status
git diff
```

## 4. Commit the Change

```bash
git add index.html
git commit -m "Practice first branch change"
```

Replace `index.html` with the file you actually changed.

## 5. Push the Branch

```bash
git push -u origin practice/my-first-change
```

## 6. Open a Pull Request

On GitHub, open a pull request from your branch into `main`.

Use the pull request template and confirm that no secrets, credentials, personal data, or confidential information are included.

## 7. Review Checks

GitHub Actions will run basic repository checks.

If a check fails, read the message, fix the issue, commit again, and push again.

## 8. Merge

When the change is reviewed and the checks are acceptable, merge the pull request.

## Safety Rules

Do not commit:

- passwords;
- API keys;
- tokens;
- private keys;
- `.env` files;
- personal data;
- customer data;
- confidential information;
- regulated data;
- production credentials.

This repository is a learning sandbox, not a production project.
