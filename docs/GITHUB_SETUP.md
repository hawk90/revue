# GitHub Repository Setup Guide

This document describes the GitHub settings for the revue repository.

## Branch Protection Rules

**Settings → Branches → Add rule**

### Main Branch Protection

**Branch name pattern:** `main`

#### Pull Request Settings

- [x] **Require a pull request before merging**
  - [x] Require approvals: `1`
  - [x] Dismiss stale pull request approvals when new commits are pushed
  - [x] Require approval of the most recent reviewable push

#### Status Checks

- [x] **Require status checks to pass before merging**
  - [x] Require branches to be up to date before merging

  **Required checks:**
  - `Check`
  - `Test (ubuntu-latest)`
  - `Test (macos-latest)`
  - `Test (windows-latest)`
  - `Format`
  - `Clippy`
  - `Validate Commits` (commitlint)
  - `Validate PR Title`

#### Additional Protection

- [x] **Require conversation resolution before merging**
- [x] **Do not allow bypassing the above settings**
- [ ] Require signed commits (optional)
- [ ] Require linear history (optional - achieved automatically with squash merge)

#### Push Restrictions

- [x] **Restrict who can push to matching branches**
  - Allow only Release Please bot (for release tag creation)

---

## Merge Settings

**Settings → General → Pull Requests**

- [ ] Allow merge commits
- [x] **Allow squash merging** (default)
  - Default commit message: `Pull request title`
- [ ] Allow rebase merging
- [x] **Always suggest updating pull request branches**
- [x] **Automatically delete head branches**

---

## GitHub Actions Settings

**Settings → Actions → General**

### Actions permissions

- [x] Allow all actions and reusable workflows

### Workflow permissions

- [x] Read and write permissions
- [x] **Allow GitHub Actions to create and approve pull requests**

---

## Environments

**Settings → Environments**

### github-pages

- **Deployment branches:** `main` only
- **Required reviewers:** None (automatic deployment)

---

## Secrets

**Settings → Secrets and variables → Actions**

### Repository secrets

| Name | Description | Source |
|------|-------------|--------|
| `CODECOV_TOKEN` | Code coverage upload | [codecov.io](https://codecov.io) |
| `CARGO_REGISTRY_TOKEN` | crates.io publish | [crates.io](https://crates.io/settings/tokens) |

> `GITHUB_TOKEN` is automatically provided, no setup required

---

## GitHub Pages

**Settings → Pages**

- **Source:** GitHub Actions
- **Custom domain:** (optional)

---

## Setup Checklist

```
[ ] Create branch protection rule (main)
[ ] Configure merge settings (squash only)
[ ] Configure Actions permissions
[ ] Create github-pages environment
[ ] Add CODECOV_TOKEN secret
[ ] Add CARGO_REGISTRY_TOKEN secret
[ ] Set Pages source to GitHub Actions
```

## Rulesets (Recommended)

GitHub's new Rulesets feature provides more granular control.

**Settings → Rules → Rulesets → New ruleset**

```yaml
name: main-protection
target: branch
enforcement: active
bypass_actors:
  - github-actions[bot]  # For Release Please

conditions:
  ref_name:
    include: ["refs/heads/main"]

rules:
  - type: pull_request
    parameters:
      required_approving_review_count: 1
      dismiss_stale_reviews_on_push: true
      require_last_push_approval: true

  - type: required_status_checks
    parameters:
      strict_required_status_checks_policy: true
      required_status_checks:
        - context: Check
        - context: Test (ubuntu-latest)
        - context: Format
        - context: Clippy
        - context: Validate PR Title

  - type: non_fast_forward
    # Prevent force push
```
