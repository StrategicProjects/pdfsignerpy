import os
from pathlib import Path

import pytest

import pdfsigner

FIXTURES = Path(__file__).parent / "fixtures"
SAMPLE = FIXTURES / "sample.pdf"
KEYSTORE = FIXTURES / "keystore.p12"
PASSWORD = "password"


def test_has_version():
    assert isinstance(pdfsigner.__version__, str)
    assert pdfsigner.__version__


def test_sign_and_verify_invisible(tmp_path):
    out = tmp_path / "signed.pdf"
    pdfsigner.sign_pdf(str(SAMPLE), str(out), str(KEYSTORE), PASSWORD)
    assert out.exists()

    sigs = pdfsigner.verify_pdf(str(out))
    assert len(sigs) == 1
    s = sigs[0]
    assert s["valid"] is True
    assert s["covers_whole_document"] is True
    assert s["signer"]
    assert len(s["byte_range"]) == 4


def test_pathlike_inputs(tmp_path):
    out = tmp_path / "signed.pdf"
    # os.PathLike objects must work, not just str.
    pdfsigner.sign_pdf(SAMPLE, out, KEYSTORE, PASSWORD)
    assert pdfsigner.verify_pdf(out)[0]["valid"] is True


def test_visible_signature(tmp_path):
    out = tmp_path / "signed.pdf"
    pdfsigner.sign_pdf(
        SAMPLE, out, KEYSTORE, PASSWORD,
        signtext="Digitally signed (test)",
        reason="Approval",
    )
    assert pdfsigner.verify_pdf(out)[0]["valid"] is True


def test_second_signature_keeps_first_valid(tmp_path):
    first, second = tmp_path / "a.pdf", tmp_path / "b.pdf"
    pdfsigner.sign_pdf(SAMPLE, first, KEYSTORE, PASSWORD)
    pdfsigner.sign_pdf(first, second, KEYSTORE, PASSWORD)
    sigs = pdfsigner.verify_pdf(second)
    assert len(sigs) == 2
    assert all(s["valid"] for s in sigs)


def test_unsigned_pdf_has_no_signatures():
    assert pdfsigner.verify_pdf(SAMPLE) == []


def test_tampered_document_fails(tmp_path):
    out = tmp_path / "signed.pdf"
    pdfsigner.sign_pdf(SAMPLE, out, KEYSTORE, PASSWORD)
    assert pdfsigner.verify_pdf(out)[0]["valid"] is True

    data = bytearray(out.read_bytes())
    data[200] ^= 0xFF
    out.write_bytes(data)
    assert pdfsigner.verify_pdf(out)[0]["valid"] is False


def test_bad_level_raises(tmp_path):
    with pytest.raises(ValueError):
        pdfsigner.sign_pdf(SAMPLE, tmp_path / "x.pdf", KEYSTORE, PASSWORD, level="zzz")


def test_missing_input_raises(tmp_path):
    with pytest.raises(ValueError):
        pdfsigner.sign_pdf(tmp_path / "nope.pdf", tmp_path / "x.pdf", KEYSTORE, PASSWORD)


def test_wrong_password_raises(tmp_path):
    with pytest.raises(ValueError):
        pdfsigner.sign_pdf(SAMPLE, tmp_path / "x.pdf", KEYSTORE, "wrong-" + os.urandom(2).hex())
