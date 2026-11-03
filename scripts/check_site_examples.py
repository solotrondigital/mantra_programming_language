#!/usr/bin/env python3
"""Verify that displayed website examples exactly match tested Mantra files."""

from __future__ import annotations

import sys
from html.parser import HTMLParser
from pathlib import Path


class ExampleParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.current: str | None = None
        self.depth = 0
        self.examples: dict[str, str] = {}

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attrs_map = dict(attrs)
        if tag == "pre" and attrs_map.get("data-test"):
            self.current = str(attrs_map["data-test"])
            self.depth = 1
            self.examples[self.current] = ""
        elif self.current is not None:
            self.depth += 1

    def handle_endtag(self, tag: str) -> None:
        if self.current is None:
            return
        if tag == "pre" and self.depth == 1:
            self.current = None
            self.depth = 0
        else:
            self.depth -= 1

    def handle_data(self, data: str) -> None:
        if self.current is not None:
            self.examples[self.current] += data


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: check_site_examples.py path/to/examples.html")
    page = Path(sys.argv[1])
    parser = ExampleParser()
    parser.feed(page.read_text(encoding="utf-8"))
    if not parser.examples:
        raise SystemExit(f"no data-test examples found in {page}")

    errors: list[str] = []
    for relative, displayed in parser.examples.items():
        tested = Path(relative)
        if not tested.is_file():
            errors.append(f"missing test source: {relative}")
        elif displayed != tested.read_text(encoding="utf-8"):
            errors.append(f"website sample differs from tested source: {relative}")
    if errors:
        raise SystemExit("\n".join(errors))
    print(f"Verified {len(parser.examples)} website examples against tested source files.")


if __name__ == "__main__":
    main()
