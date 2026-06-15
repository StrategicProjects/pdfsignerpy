# pdfsigner (Python)

[![CI](https://github.com/StrategicProjects/pdfsignerpy/actions/workflows/ci.yml/badge.svg)](https://github.com/StrategicProjects/pdfsignerpy/actions/workflows/ci.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![pure Rust](https://img.shields.io/badge/backend-pure%20Rust-success.svg)

Digitally **sign** PDF documents with a PKCS#12 keystore and **verify** their
signatures — implementing the **PAdES** baseline profiles (ETSI EN 319 142)
from **B-B to B-LTA**.

The heavy lifting is done by the pure-Rust
[`pdf_signer`](https://github.com/StrategicProjects/pdf_signer) crate, wrapped
with [PyO3](https://pyo3.rs/): **no Java runtime, no OpenSSL, no external
command-line tools** — a single self-contained extension module. It is the
Python sibling of the
[`pdfsigner` R package](https://github.com/StrategicProjects/pdfsigner).

## Installation

A Rust toolchain is required to build from source (until wheels are published).
Install Rust from <https://rustup.rs>, then:

```bash
pip install pdfsigner            # once wheels are on PyPI
# or, from a checkout:
pip install maturin
maturin develop --release
```

## Usage

```python
import pdfsigner

# Sign (invisible). Levels above "bb" need a tsa_url.
pdfsigner.sign_pdf(
    "input.pdf", "signed.pdf", "keystore.p12", "password",
    reason="Approval",
    level="bb",                     # bb | bt | blt | blta
)

# Sign with a visible box, an embedded font and a logo.
pdfsigner.sign_pdf(
    "input.pdf", "signed.pdf", "keystore.p12", "password",
    signtext="Digitally signed",
    font="Arial.ttf",
    image="logo.png",
    level="blta",
    tsa_url="http://timestamp.digicert.com",
)

# Verify every signature.
for s in pdfsigner.verify_pdf("signed.pdf"):
    print(s["valid"], s["signer"], s["detail"])

# Verify and validate the signer chain against trusted roots (e.g. ICP-Brasil).
pdfsigner.verify_pdf("signed.pdf", roots="icp-brasil-roots.pem")
```

`verify_pdf` returns one dict per signature with keys: `valid`, `signer`,
`chain_trusted` (bool or `None` when no `roots` given), `covers_whole_document`,
`signed_len`, `byte_range` and `detail`.

## What it does

- **PAdES B-B → B-LTA**: CAdES `signing-certificate-v2`, RFC 3161 signature and
  document timestamps, a `/DSS` with the certificate chain, CRLs and OCSP.
- **Visible or invisible** signatures, with a bordered text box, an optional
  **embedded TrueType/OpenType font** and a **PNG/JPEG logo**.
- **Incremental updates** so multiple signatures compose and earlier ones stay
  valid.
- **Verification** of the message digest and the signer's signature, plus
  optional **RFC 5280 chain validation** (RSA / ECDSA / Ed25519, CRL + OCSP,
  name constraints and a NIST PKITS-validated policy engine).

## License

GPL-3.0-or-later. The bundled `pdf_signer` crate and its Rust dependencies
retain their own (permissive) licenses.
