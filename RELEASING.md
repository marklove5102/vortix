# Releasing Vortix

This document explains how to create a new release and how to revert one if something goes wrong.

## Overview

Vortix uses two tools for automated releases:

| Tool | Purpose |
|------|---------|
| **release-plz** | Version bumping, changelog generation, crates.io publishing |
| **cargo-dist** | Building macOS binaries, creating GitHub releases |

## Release Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  1. You trigger "Release-plz" workflow manually                 │
│                           ↓                                     │
│  2. release-plz analyzes commits and creates a Release PR       │
│     - Bumps version in Cargo.toml (based on commit types)       │
│     - Updates CHANGELOG.md                                      │
│                           ↓                                     │
│  3. You review and merge the PR                                 │
│                           ↓                                     │
│  4. release-plz automatically:                                  │
│     - Creates git tag (e.g., v0.2.0)                            │
│     - Publishes to crates.io                                    │
│                           ↓                                     │
│  5. Tag triggers cargo-dist workflow which:                     │
│     - Builds macOS binaries (Intel + Apple Silicon)             │
│     - Creates GitHub Release with binaries attached             │
│     - Generates shell installer script                          │
└─────────────────────────────────────────────────────────────────┘
```

## Step-by-Step: Creating a Release

### Step 1: Trigger the Release Workflow

1. Go to your GitHub repository
2. Click **Actions** tab
3. Select **Release-plz** from the left sidebar
4. Click **Run workflow** button (top right)
5. Select `main` branch
6. Click **Run workflow**

### Step 2: Review the Release PR

After a few minutes, release-plz will create a PR titled something like:

> **chore: release v0.2.0**

The PR will contain:
- Version bump in `Cargo.toml`
- Updated `CHANGELOG.md` with all changes since last release

**Review checklist:**
- [ ] Version number looks correct
- [ ] Changelog entries make sense
- [ ] No unwanted changes included

### Step 3: Merge the PR

1. Click **Merge pull request**
2. Use **Squash and merge** (recommended) or regular merge
3. Delete the branch when prompted

### Step 4: Watch the Magic ✨

After merging:

1. **release-plz** creates a git tag (e.g., `v0.2.0`)
2. **release-plz** publishes to crates.io (if `CARGO_REGISTRY_TOKEN` is set)
3. The tag triggers **cargo-dist** workflow
4. **cargo-dist** builds binaries and creates GitHub Release

You can monitor progress in the **Actions** tab.

### Step 5: Verify the Release

1. Go to **Releases** page on GitHub
2. Check that the new release exists
3. Verify binaries are attached:
   - `vortix-aarch64-apple-darwin.tar.gz` (Apple Silicon)
   - `vortix-x86_64-apple-darwin.tar.gz` (Intel Mac)
4. Test the installer: `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Harry-kp/vortix/releases/latest/download/vortix-installer.sh | sh`

---

## Reverting a Release

Sometimes things go wrong. Here's how to undo a release.

### Scenario 1: PR Not Yet Merged

**Easy fix:** Just close the PR without merging.

```bash
# No action needed locally
```

### Scenario 2: PR Merged, Tag Created, But Release Failed

If cargo-dist failed but the tag exists:

```bash
# Delete the tag locally and remotely
git tag -d v0.2.0
git push origin --delete v0.2.0

# Revert the version bump commit
git revert HEAD
git push origin main
```

Then fix the issue and start the release process again.

### Scenario 3: Full Release Published (GitHub + crates.io)

**⚠️ Important:** You cannot delete a version from crates.io, but you can "yank" it.

#### Step 1: Yank from crates.io (prevents new installs)

```bash
cargo yank --version 0.2.0
```

#### Step 2: Delete GitHub Release

1. Go to **Releases** page
2. Click on the release (e.g., v0.2.0)
3. Click **Delete** (trash icon)
4. Confirm deletion

#### Step 3: Delete the Git Tag

```bash
# Delete locally
git tag -d v0.2.0

# Delete from remote
git push origin --delete v0.2.0
```

#### Step 4: Revert the Changes

```bash
# Find the merge commit
git log --oneline -5

# Revert it (replace COMMIT_HASH with actual hash)
git revert COMMIT_HASH
git push origin main
```

#### Step 5: Release a Patch Version

After fixing the issue, create a new release. The next version will be `0.2.1` (you can't reuse `0.2.0` on crates.io).

---

## Version Bump Rules

release-plz uses [Conventional Commits](https://www.conventionalcommits.org/) to determine version bumps:

| Commit Type | Example | Version Bump |
|-------------|---------|--------------|
| `fix:` | `fix: handle empty server list` | Patch (0.1.0 → 0.1.1) |
| `feat:` | `feat: add WireGuard support` | Minor (0.1.0 → 0.2.0) |
| `feat!:` or `BREAKING CHANGE:` | `feat!: change config format` | Major (0.1.0 → 1.0.0) |
| `chore:`, `docs:`, `style:` | `docs: update README` | No release |

---

## Troubleshooting

### "Release-plz" workflow doesn't create a PR

**Cause:** No releasable commits since last release.

**Fix:** Make sure you have at least one `feat:` or `fix:` commit.

### cargo-dist fails to build

**Cause:** Usually a compilation error or missing dependency.

**Fix:** 
1. Check the workflow logs
2. Fix the issue locally
3. Delete the tag: `git push origin --delete v0.2.0`
4. Start the release process again

### crates.io publish fails

**Cause:** Missing or invalid `CARGO_REGISTRY_TOKEN`.

**Fix:**
1. Go to https://crates.io/settings/tokens
2. Create a new token with `publish-update` scope
3. Add it as `CARGO_REGISTRY_TOKEN` secret in GitHub repo settings

### "version already exists" error

**Cause:** Trying to publish a version that was previously yanked.

**Fix:** Bump the version manually in `Cargo.toml` and commit:

```bash
# Edit Cargo.toml to bump version
git add Cargo.toml
git commit -m "chore: bump version to 0.2.1"
git push origin main
```

---

## Required Secrets

| Secret | Where to Get | Purpose |
|--------|--------------|---------|
| `GITHUB_TOKEN` | Auto-provided | Creating PRs, releases, tags |
| `CARGO_REGISTRY_TOKEN` | https://crates.io/settings/tokens | Publishing to crates.io |

---

## Quick Reference

```bash
# Check current version
grep '^version' Cargo.toml

# List all tags
git tag -l

# Delete a tag (local + remote)
git tag -d v0.2.0 && git push origin --delete v0.2.0

# Yank a crates.io version
cargo yank --version 0.2.0

# Un-yank (if you change your mind)
cargo yank --version 0.2.0 --undo
```
