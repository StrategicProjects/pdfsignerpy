# API reference

The package exposes two functions.

## `sign_pdf`

```python
pdfsigner.sign_pdf(
    input, output, keystore, password, *,
    reason=None, name=None, location=None,
    level="bb", tsa_url=None,
    signtext=None, page=1, x=36.0, y=36.0, width=320.0, height=64.0,
    font_size=8.0, border=True, font=None, image=None,
) -> None
```

Sign `input` with a PKCS#12 (`.p12`/`.pfx`) keystore and write the signed PDF to
`output`. The signature is a detached `adbe.pkcs7.detached` CMS over the whole
document, added as an incremental update (existing signatures stay valid).
Raises `ValueError` on failure.

| Parameter | Description |
| --- | --- |
| `input`, `output`, `keystore` | paths (`str` or `os.PathLike`) |
| `password` | keystore password |
| `reason`, `name`, `location` | signature dictionary metadata |
| `level` | PAdES profile: `"bb"`, `"bt"`, `"blt"`, `"blta"`. `bt`+ need `tsa_url` |
| `tsa_url` | RFC 3161 timestamp authority (`http://` or `https://`) |
| `signtext` | text for a **visible** signature box (omit for invisible) |
| `page`, `x`, `y`, `width`, `height`, `font_size`, `border` | visible box geometry, in points |
| `font` | path to a TrueType/OpenType font to embed (default: Helvetica) |
| `image` | path to a PNG/JPEG logo to draw in the box |

```python
pdfsigner.sign_pdf(
    "in.pdf", "out.pdf", "keystore.p12", "password",
    signtext="Digitally signed", reason="Approval",
    font="Arial.ttf", image="logo.png",
    level="blta", tsa_url="http://timestamp.digicert.com",
)
```

## `verify_pdf`

```python
pdfsigner.verify_pdf(input, roots=None) -> list[dict]
```

Verify every signature in `input`. Returns one dict per signature; an empty list
means none were found. Pass `roots` (a PEM bundle of trusted roots) to validate
each signer's certificate chain.

| Key | Meaning |
| --- | --- |
| `valid` | `bool` — CMS signature valid over its byte range |
| `signer` | signer subject DN (`str` or `None`) |
| `chain_trusted` | `bool` when `roots` given, else `None` |
| `covers_whole_document` | `bool` |
| `signed_len` | bytes covered by the signature |
| `byte_range` | the four `/ByteRange` integers |
| `detail` | human-readable detail |

```python
for s in pdfsigner.verify_pdf("out.pdf", roots="icp-brasil-roots.pem"):
    print(s["valid"], s["chain_trusted"], s["signer"])
```
