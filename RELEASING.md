# Releasing `pdfsigner` to PyPI

Publishing is automated by `.github/workflows/release.yml`. It builds abi3
wheels (Linux x86_64/aarch64, macOS x86_64/arm64, Windows x64) plus an sdist and
uploads them to PyPI using **Trusted Publishing** (OIDC) — there is no API token
to store or rotate.

## One-time setup on PyPI

Because `pdfsigner` is not yet on PyPI, register a *pending* trusted publisher
(this also reserves the name on first publish):

1. Sign in at <https://pypi.org> → **Account → Publishing** →
   *Add a pending publisher*.
2. Fill in:
   - **PyPI Project Name**: `pdfsigner`
   - **Owner**: `StrategicProjects`
   - **Repository name**: `pdfsignerpy`
   - **Workflow name**: `release.yml`
   - **Environment name**: `pypi`
3. Save.

Optionally, in the GitHub repo, create an **Environment** named `pypi`
(Settings → Environments) and add protection rules (e.g. required reviewers) so a
publish must be approved.

## Cutting a release

1. Bump the version in **`Cargo.toml`** (`[package] version`). maturin reads the
   package version from there (`pyproject.toml` declares `dynamic = ["version"]`).
2. Commit and tag:
   ```bash
   git commit -am "Release vX.Y.Z"
   git tag vX.Y.Z
   git push origin main --tags
   ```
3. The `release` workflow builds all wheels + sdist and publishes them to PyPI.

## Dry run (no publish)

Trigger the workflow manually (Actions → *release* → *Run workflow*) or:

```bash
gh workflow run release.yml
```

On a manual run the `publish` job is skipped (it only runs for `v*` tags), so you
get the built wheels and sdist as artifacts to inspect without touching PyPI.
