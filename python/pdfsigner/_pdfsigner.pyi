from os import PathLike
from typing import Any, Union

__version__: str

_Path = Union[str, PathLike[str]]

def sign_pdf(
    input: _Path,
    output: _Path,
    keystore: _Path,
    password: str,
    *,
    reason: str | None = ...,
    name: str | None = ...,
    location: str | None = ...,
    level: str = ...,
    tsa_url: str | None = ...,
    signtext: str | None = ...,
    page: int = ...,
    x: float = ...,
    y: float = ...,
    width: float = ...,
    height: float = ...,
    font_size: float = ...,
    border: bool = ...,
    font: _Path | None = ...,
    image: _Path | None = ...,
) -> None:
    """Sign ``input`` with a PKCS#12 keystore, writing ``output``."""

def verify_pdf(
    input: _Path,
    roots: _Path | None = ...,
) -> list[dict[str, Any]]:
    """Verify every signature in ``input``; returns one dict per signature."""
