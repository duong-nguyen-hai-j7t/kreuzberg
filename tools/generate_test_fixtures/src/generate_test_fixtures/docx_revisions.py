"""DOCX track-changes fixture generator (stub).

Populated in a follow-up commit. The CLI dispatch references this module
eagerly when the ``docx`` command is selected, so the stub returns an empty
list rather than raising.
"""

from __future__ import annotations

from pathlib import Path


def generate(output_root: Path, repo_root: Path) -> list[Path]:
    """Placeholder until the DOCX generator is implemented.

    Returns an empty list so the CLI still reports a clean run when only
    the scaffold has been merged.
    """
    del output_root, repo_root
    return []
