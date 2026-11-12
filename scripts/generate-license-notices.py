#!/usr/bin/env python3
"""Regenerate gathered upstream notices; normalize whitespace without changing words."""
from pathlib import Path
import subprocess
import tempfile

root = Path(__file__).resolve().parents[1]
version = subprocess.check_output(["cargo-about", "--version"], text=True).strip()
if not version.endswith("0.9.3"):
    raise SystemExit("Install cargo-about 0.9.3 for this checked notice template")
with tempfile.TemporaryDirectory(prefix="agon-licenses-") as directory:
    output = Path(directory) / "notices.txt"
    subprocess.run(["cargo-about", "generate", "--locked", "--workspace", "--fail",
                    "--output-file", str(output), "licenses.hbs"], cwd=root, check=True)
    text = output.read_text(encoding="utf-8")
    normalized = "\n".join(line.rstrip() for line in text.splitlines()).rstrip() + "\n"
    (root / "THIRD_PARTY_LICENSES.txt").write_text(normalized, encoding="utf-8")
