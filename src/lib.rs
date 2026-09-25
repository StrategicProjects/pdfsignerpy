//! Python bindings for the pure-Rust `pdf_signer` PAdES engine.
//!
//! Exposes `sign_pdf`, `verify_pdf` and `verify_pdf_report`, imported in
//! Python as `pdfsigner._pdfsigner`. The user-facing API lives in
//! `python/pdfsigner`.

use std::path::PathBuf;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use pdf_signer::{
    sign_pdf_file, verify_pdf_file, verify_pdf_file_with_roots, Appearance, PadesLevel,
    SignOptions, TrustStore,
};

/// Turn any displayable error into a Python `ValueError`.
fn err<E: std::fmt::Display>(e: E) -> PyErr {
    PyValueError::new_err(e.to_string())
}

/// Map a level string (e.g. "bb", "b-lta") to a `PadesLevel`.
fn parse_level(s: &str) -> PyResult<PadesLevel> {
    match s.to_ascii_lowercase().replace('-', "").as_str() {
        "bb" => Ok(PadesLevel::Bb),
        "bt" => Ok(PadesLevel::Bt),
        "blt" => Ok(PadesLevel::Blt),
        "blta" => Ok(PadesLevel::Blta),
        other => Err(PyValueError::new_err(format!(
            "unknown PAdES level {other:?}; expected one of bb, bt, blt, blta"
        ))),
    }
}

/// Read an optional file into bytes; `None` path means "no file".
fn read_opt(path: Option<PathBuf>) -> PyResult<Option<Vec<u8>>> {
    match path {
        Some(p) => Ok(Some(std::fs::read(&p).map_err(err)?)),
        None => Ok(None),
    }
}

/// Sign ``input`` with a PKCS#12 keystore, writing ``output``.
///
/// A visible signature box is drawn when ``signtext`` is given; pass ``font``
/// (a .ttf/.otf path) to embed a TrueType/OpenType font and ``image`` (a
/// PNG/JPEG path) to draw a logo. ``level`` selects the PAdES profile
/// (``"bb"``, ``"bt"``, ``"blt"``, ``"blta"``); levels above ``bb`` require
/// ``tsa_url``. Raises ``ValueError`` on failure.
#[pyfunction]
#[pyo3(signature = (
    input, output, keystore, password, *,
    reason=None, name=None, location=None, contact_info=None, level="bb", tsa_url=None,
    signtext=None, page=1, x=36.0, y=36.0, width=320.0, height=64.0,
    font_size=8.0, border=true, font=None, image=None
))]
#[allow(clippy::too_many_arguments)]
fn sign_pdf(
    input: PathBuf,
    output: PathBuf,
    keystore: PathBuf,
    password: &str,
    reason: Option<String>,
    name: Option<String>,
    location: Option<String>,
    contact_info: Option<String>,
    level: &str,
    tsa_url: Option<String>,
    signtext: Option<String>,
    page: usize,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    font_size: f64,
    border: bool,
    font: Option<PathBuf>,
    image: Option<PathBuf>,
) -> PyResult<()> {
    let pades_level = parse_level(level)?;
    let appearance = match signtext {
        Some(text) if !text.is_empty() => Some(Appearance {
            page: page.max(1),
            x,
            y,
            width,
            height,
            font_size,
            text,
            border,
            font: read_opt(font)?,
            image: read_opt(image)?,
            image_rect: None,
        }),
        _ => None,
    };

    let opts = SignOptions {
        reason,
        name,
        location,
        contact_info,
        tsa_url,
        pades_level,
        appearance,
        ..Default::default()
    };

    sign_pdf_file(&input, &output, &keystore, password, &opts).map_err(err)
}

/// Verify every signature in ``input``.
///
/// Returns a list of dicts, one per signature or document timestamp (in file
/// order), with keys ``valid``, ``is_timestamp``, ``signer``,
/// ``chain_trusted``, ``covers_whole_document``, ``trusted_time`` (seconds
/// since the Unix epoch of the trusted RFC 3161 time the chain was judged at,
/// or ``None``), ``signed_len``, ``byte_range`` and ``detail``. Pass ``roots``
/// (a PEM bundle of trusted root certificates) to validate each signer's and
/// TSA's chain (``chain_trusted`` is then a bool, otherwise ``None``). An
/// empty list means no signatures were found.
///
/// This per-signature view says nothing about the document as a whole; use
/// :func:`verify_pdf_report` for the ``document_intact`` / ``all_valid`` /
/// ``all_trusted`` verdict.
#[pyfunction]
#[pyo3(signature = (input, roots=None))]
fn verify_pdf(
    py: Python<'_>,
    input: PathBuf,
    roots: Option<PathBuf>,
) -> PyResult<Vec<Py<PyDict>>> {
    let report = run_verify(&input, roots)?;
    signatures_to_py(py, &report)
}

/// Verify ``input`` and return the whole-document report.
///
/// Returns a dict with ``signatures`` (the list :func:`verify_pdf` returns),
/// ``document_intact`` (the last valid signature or document timestamp covers
/// the entire file, or everything after it is a PAdES ``/DSS`` — content
/// changed by an unsigned incremental update after signing makes this
/// ``False``), ``all_valid`` (at least one signature, every entry valid and the
/// document intact) and ``all_trusted`` (``all_valid`` and every entry chains
/// to one of ``roots``; always ``False`` without ``roots``).
#[pyfunction]
#[pyo3(signature = (input, roots=None))]
fn verify_pdf_report(
    py: Python<'_>,
    input: PathBuf,
    roots: Option<PathBuf>,
) -> PyResult<Py<PyDict>> {
    let report = run_verify(&input, roots)?;
    let d = PyDict::new(py);
    d.set_item("signatures", signatures_to_py(py, &report)?)?;
    d.set_item("document_intact", report.document_intact)?;
    d.set_item("all_valid", report.all_valid())?;
    d.set_item("all_trusted", report.all_trusted())?;
    Ok(d.unbind())
}

fn run_verify(input: &PathBuf, roots: Option<PathBuf>) -> PyResult<pdf_signer::SignatureReport> {
    match roots {
        Some(r) => {
            let pem = std::fs::read(&r).map_err(err)?;
            let store = TrustStore::from_pem(&pem).map_err(err)?;
            verify_pdf_file_with_roots(input, &store).map_err(err)
        }
        None => verify_pdf_file(input).map_err(err),
    }
}

fn signatures_to_py(py: Python<'_>, report: &pdf_signer::SignatureReport) -> PyResult<Vec<Py<PyDict>>> {
    let mut out = Vec::with_capacity(report.signatures.len());
    for s in &report.signatures {
        let trusted_time: Option<f64> = s.trusted_time.and_then(|t| {
            t.duration_since(std::time::UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs_f64())
        });
        let d = PyDict::new(py);
        d.set_item("valid", s.valid)?;
        d.set_item("is_timestamp", s.is_timestamp)?;
        d.set_item("signer", s.signer.clone())?;
        d.set_item("chain_trusted", s.chain_trusted)?;
        d.set_item("covers_whole_document", s.covers_whole_document)?;
        d.set_item("trusted_time", trusted_time)?;
        d.set_item("signed_len", s.signed_len)?;
        d.set_item("byte_range", s.byte_range.to_vec())?;
        d.set_item("detail", s.detail.clone())?;
        out.push(d.unbind());
    }
    Ok(out)
}

#[pymodule]
fn _pdfsigner(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(sign_pdf, m)?)?;
    m.add_function(wrap_pyfunction!(verify_pdf, m)?)?;
    m.add_function(wrap_pyfunction!(verify_pdf_report, m)?)?;
    Ok(())
}
