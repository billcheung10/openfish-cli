#!/usr/bin/env python3
"""Public-release safety scanner for the Openfish CLI package."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

BLOCKED_PATH_PARTS = {
    ".git",
    "target",
    "node_modules",
    "__pycache__",
}

TEXT_EXTENSIONS = {
    "",
    ".rs",
    ".toml",
    ".lock",
    ".md",
    ".sh",
    ".rb",
    ".json",
    ".yml",
    ".yaml",
    ".txt",
}

PATTERNS = [
    ("pem file", re.compile(r"\b[A-Za-z0-9_.-]+\.pem\b", re.IGNORECASE)),
    ("private ssh key", re.compile(r"BEGIN [A-Z ]*PRIVATE KEY")),
    ("deployment ip", re.compile(r"\b122\.248\.217\.246\b")),
    ("deployment path", re.compile(r"/home/ubuntu/Openfish")),
    ("downloaded deploy key path", re.compile(r"/Users/[^\\s]+/Downloads/[^\\s]+\.pem")),
    (
        "private key literal",
        re.compile(r"(?i)(private[_ -]?key|secret[_ -]?key|seed)[^\n]{0,80}0x[a-f0-9]{64}"),
    ),
]

ALLOWLIST = {
    "openfish-client-sdk/src/auth.rs",
    "openfish-client-sdk/tests/common/mod.rs",
    "openfish-cli/tests/cli_integration.rs",
}


def is_text_file(path: Path) -> bool:
    return path.suffix in TEXT_EXTENSIONS


def should_skip(path: Path) -> bool:
    return any(part in BLOCKED_PATH_PARTS for part in path.parts)


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def main() -> int:
    failures: list[str] = []
    for path in ROOT.rglob("*"):
        if not path.is_file() or should_skip(path) or not is_text_file(path):
            continue
        name = rel(path)
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for label, pattern in PATTERNS:
            for match in pattern.finditer(text):
                if label == "private key literal" and name in ALLOWLIST:
                    continue
                line = text.count("\n", 0, match.start()) + 1
                failures.append(f"{name}:{line}: {label}: {match.group(0)}")

    if failures:
        for failure in failures:
            print(f"FAIL {failure}", file=sys.stderr)
        return 1
    print("openfish-cli public safety scan passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
