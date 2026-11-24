"""Project rule: absolute imports only (``from rustradiomics.x import y``, never ``from .x``)."""

from __future__ import annotations

import ast
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1] / "python" / "rustradiomics"
TESTS = Path(__file__).resolve().parent


def relative_imports(path: Path) -> list[str]:
    tree = ast.parse(path.read_text(), filename=str(path))
    return [
        f"{path.name}:{node.lineno}"
        for node in ast.walk(tree)
        if isinstance(node, ast.ImportFrom) and node.level > 0
    ]


def test_package_and_tests_use_absolute_imports_only() -> None:
    files = sorted([*PACKAGE.rglob("*.py"), *PACKAGE.rglob("*.pyi"), *TESTS.rglob("*.py")])
    assert len(files) > 10
    violations = [hit for path in files for hit in relative_imports(path)]
    assert violations == [], f"relative imports: {violations}"
