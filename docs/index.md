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
pip install pdfsignerpy          # once wheels are published; then `import pdfsigner`
# or, from a checkout (needs a Rust toolchain from https://rustup.rs):
pip install maturin
maturin develop --release
```

!!! note
    The PyPI distribution is **`pdfsignerpy`**, but you `import pdfsigner`
    (`pdfsigner` is blocked on PyPI as too similar to `pdf-signer`).

## Quick start

```python
import pdfsigner

pdfsigner.sign_pdf("input.pdf", "signed.pdf", "keystore.p12", "password",
                   signtext="Digitally signed")

for s in pdfsigner.verify_pdf("signed.pdf"):
    print(s["valid"], s["signer"])
```

See the [API reference](api.md) for every option.

## Features

- **PAdES B-B → B-LTA**: CAdES `signing-certificate-v2`, RFC 3161 signature and
  document timestamps, a `/DSS` with the chain, CRLs and OCSP.
- **Visible or invisible** signatures, embedded TrueType/OpenType font and a
  PNG/JPEG logo.
- **Incremental updates** — multiple signatures compose; earlier ones stay
  valid.
- **Verification** with optional RFC 5280 chain validation (RSA / ECDSA /
  Ed25519, CRL + OCSP, name constraints, a NIST PKITS-validated policy engine).
