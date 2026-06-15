# pdfsignerpy (Python package) — guidance for Claude

Python bindings (**PyO3 + maturin**) over the pure-Rust `pdf_signer` engine.
**Published on PyPI** as `pdfsignerpy` — but the **import name is `pdfsigner`**
(`pip install pdfsignerpy` → `import pdfsigner`; PyPI blocks `pdfsigner` itself as
too similar to the existing `pdf-signer`).

## Structure
- `src/lib.rs` — PyO3 bindings: `sign_pdf()` / `verify_pdf()`, compiled as the
  `_pdfsigner` extension module.
- `python/pdfsigner/` — the Python package (mixed layout): `__init__.py` re-exports
  from `._pdfsigner`; `_pdfsigner.pyi` type stubs; `py.typed`.
- `Cargo.toml` — `pdf_signer` is a **git dep pinned to tag `v0.1.7`** (not vendored).
- `pyproject.toml` — distribution name `pdfsignerpy`, `module-name =
  pdfsigner._pdfsigner`, `python-source = python`, abi3.
- `tests/` — pytest; fixtures in `tests/fixtures/`.

## Build / test
```sh
# In a virtualenv:
maturin develop --release && pytest -q
# Without a venv (what CI uses — `maturin develop` REQUIRES a venv):
pip install . && pytest -q
```

## Docs
MkDocs-Material in `docs/`; `.github/workflows/docs.yml` runs `mkdocs gh-deploy`
→ **gh-pages** → https://strategicprojects.github.io/pdfsignerpy/

## Release (PyPI, Trusted Publishing)
1. Bump `version` in **`Cargo.toml`** (maturin reads it; pyproject is `dynamic`).
2. `git tag vX.Y.Z && git push --tags`.
3. `.github/workflows/release.yml` builds abi3 wheels and publishes via OIDC.
   See `RELEASING.md` for the one-time PyPI trusted-publisher setup
   (project `pdfsignerpy`, repo `pdfsignerpy`, workflow `release.yml`,
   environment `pypi`).

## CI gotchas (hard-won)
- `maturin develop` needs an active virtualenv → CI uses `pip install .` instead.
- aarch64 **Linux** wheels need **`manylinux: "2_28"`** (the manylinux2014
  cross-gcc is too old to define `__ARM_ARCH` for `ring`'s ARM assembly).
- macOS ships one **universal2** wheel built on `macos-latest` (avoids the slow
  `macos-13` Intel runner). Do not enable `sccache` (breaks ring's asm).
