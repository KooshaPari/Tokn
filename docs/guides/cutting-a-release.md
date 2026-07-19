# Cutting a release

Operator/agent checklist for shipping a Tokn GitHub Release with binary assets.
Do **not** publish to crates.io unless `CARGO_REGISTRY_TOKEN` is set and you
intentionally want crates.io updated.

## Preconditions

1. Workspace version in root `Cargo.toml` (`workspace.package.version`) matches
   the intended tag (currently **0.1.5** → tag `v0.1.5`).
2. `CHANGELOG.md` has a dated section for that version.
3. Local quality gate: `cargo test --workspace --offline --lib` (add
   integration tests if they are fast enough).
4. Branch merged to `main` (or you are tagging the exact commit you intend).

## Tag and push (creates the release workflow run)

```bash
# From a clean checkout of the release commit on main:
git tag -a v0.1.5 -m "Tokn v0.1.5"
git push origin v0.1.5
```

Pushing `v*` triggers:

- `.github/workflows/release.yml` → calls `binaries.yml` (matrix includes
  `aarch64-apple-darwin`, `x86_64-apple-darwin`, linux, windows)
- `.github/workflows/release-crates.yml` publish job (requires
  `CARGO_REGISTRY_TOKEN`; fails loudly if missing)

## Attach / verify GitHub Release assets

`binaries.yml` uploads each matrix binary via `softprops/action-gh-release`
with `fail_on_unmatched_files: true`. If Actions billing blocks the run, build
locally and attach:

```bash
# Optional: create/update the GitHub Release and attach local builds
gh release create v0.1.5 \
  --title "v0.1.5" \
  --notes-file CHANGELOG.md \
  dist/tokenledger-aarch64-apple-darwin \
  dist/tokenledger-x86_64-apple-darwin \
  dist/tokenledger-x86_64-unknown-linux-gnu \
  dist/tokenledger-x86_64-pc-windows-msvc.exe

# Or attach to an existing draft/published release:
gh release upload v0.1.5 dist/tokenledger-* --clobber
```

Verify:

```bash
gh release view v0.1.5
gh release download v0.1.5 --dir /tmp/tokn-v0.1.5-assets --pattern 'tokenledger-*'
```

## Explicit non-goals for a dry run

- Do not `cargo publish` without operator confirmation.
- Do not force-push tags; retag only after deleting the remote tag intentionally.
