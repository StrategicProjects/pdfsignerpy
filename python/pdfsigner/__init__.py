"""pdfsigner — digitally sign and verify PDF documents.

A thin Python wrapper over the pure-Rust ``pdf_signer`` PAdES engine (no Java,
no OpenSSL, no external command-line tools). Two functions are exposed:

* :func:`sign_pdf` — sign a PDF with a PKCS#12 keystore (PAdES B-B … B-LTA,
  optional visible signature box with an embedded font and logo).
* :func:`verify_pdf` — verify every signature in a PDF and, optionally,
  validate each signer's certificate chain against trusted roots.

Example
-------
>>> import pdfsigner
>>> pdfsigner.sign_pdf("in.pdf", "out.pdf", "keystore.p12", "password",
...                    signtext="Digitally signed")
>>> [s["valid"] for s in pdfsigner.verify_pdf("out.pdf")]
[True]
"""

from ._pdfsigner import __version__, sign_pdf, verify_pdf

__all__ = ["sign_pdf", "verify_pdf", "__version__"]
