# pdfsigner (Python)

Digitally **sign** PDF documents with a PKCS#12 keystore and **verify** their
signatures — the **PAdES** baseline profiles (ETSI EN 319 142) from **B-B to
B-LTA**.

The engine is the pure-Rust
[`pdf_signer`](https://github.com/StrategicProjects/pdf_signer) crate, wrapped
with [PyO3](https://pyo3.rs/): **no Java, no OpenSSL, no external tools**. It is
the Python sibling of the
[`pdfsigner` R package](https://github.com/StrategicProjects/pdfsigner).

## Install

```bash
pip install pdfsignerpy
```

Pre-built wheels are published for Linux (x86_64 · aarch64), macOS (Intel ·
Apple Silicon) and Windows, so installation needs **no compiler and no Rust**.

!!! note
    The PyPI distribution is **`pdfsignerpy`**, but you `import pdfsigner`
    (`pdfsigner` is blocked on PyPI as too similar to `pdf-signer`).

## Why pdfsigner?

Most Python PDF-signing libraries lean on heavy native stacks — OpenSSL via
`cryptography`, a Java runtime, or external tools like Poppler. `pdfsigner`
bundles the **entire crypto + PDF pipeline as one self-contained Rust
extension**, so there is nothing else to install.

- 🦀 **Zero system dependencies** — no OpenSSL, no Java, no Poppler.
- 📦 **Pre-built wheels** for Linux/macOS/Windows — `pip install` and go.
- 🔏 **Real PAdES B-B → B-LTA** — RFC 3161 timestamps and LTV (`/DSS` with the
  chain, CRLs and OCSP).
- ✅ **NIST PKITS-validated** RFC 5280 path validation (name constraints +
  certificate-policy engine).
- 🔑 RSA, ECDSA (P-256/P-384) and Ed25519; CRL + OCSP revocation.
- 🖋 Visible signatures with an embedded font and a PNG/JPEG logo.
- 🔁 The same engine powers the
  [`pdfsigner` R package](https://github.com/StrategicProjects/pdfsigner).

## Quick start

```python
import pdfsigner

pdfsigner.sign_pdf("input.pdf", "signed.pdf", "keystore.p12", "password",
                   signtext="Digitally signed")

for s in pdfsigner.verify_pdf("signed.pdf"):
    print(s["valid"], s["signer"])
```

See the [API reference](api.md) for every option.

## Architecture

![pdfsigner (Python) architecture: the Python API calls a PyO3 extension module that links the pure-Rust pdf_signer crate.](architecture.svg)

`import pdfsigner` calls a thin [PyO3](https://pyo3.rs/) extension module that
links the pure-Rust [`pdf_signer`](https://github.com/StrategicProjects/pdf_signer)
crate (pinned to `v0.1.7`). The same engine powers the
[`pdfsigner` R package](https://github.com/StrategicProjects/pdfsigner).

## Features

- **PAdES B-B → B-LTA**: CAdES `signing-certificate-v2`, RFC 3161 signature and
  document timestamps, a `/DSS` with the chain, CRLs and OCSP.
- **Visible or invisible** signatures, embedded TrueType/OpenType font and a
  PNG/JPEG logo.
- **Incremental updates** — multiple signatures compose; earlier ones stay
  valid.
- **Verification** with optional RFC 5280 chain validation (RSA / ECDSA /
  Ed25519, CRL + OCSP, name constraints, a NIST PKITS-validated policy engine).

## Authors

- **André Leite** — Universidade Federal de Pernambuco (maintainer)
- **Hugo Vasconcelos** — Universidade Federal de Pernambuco
- **Diogo Bezerra** — Universidade Federal de Pernambuco
- **Marcos Wasiliew** — Universidade Federal de Pernambuco
- **Carlos Amorim** — Universidade Federal de Pernambuco

## Citation

If you use this software, please cite it using the metadata in the project's
[`CITATION.cff`](https://github.com/StrategicProjects/pdfsignerpy/blob/main/CITATION.cff).
