#!/usr/bin/env python3
"""
Mantra Software Bill of Materials (SBOM) Generator
scripts/release/generate_sbom.py

Generates production-grade SPDX 2.3 and CycloneDX 1.5 JSON SBOM specifications
for the Mantra compiler, runtime, and all standard workspace crates.
Extracts the version dynamically from Cargo.toml.
"""

import os
import sys
import json
import uuid
from datetime import datetime, timezone

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))

def get_version():
    if len(sys.argv) > 1 and sys.argv[1] and not sys.argv[1].startswith("-"):
        v = sys.argv[1]
        return v.lstrip("vV")
    cargo_path = os.path.join(REPO_ROOT, "Cargo.toml")
    if os.path.exists(cargo_path):
        with open(cargo_path, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if line.startswith("version") and "=" in line:
                    return line.split("=", 1)[1].strip().strip('"').strip("'")
    return "0.5.0"

VERSION = get_version()
DIST_DIR = os.path.join(REPO_ROOT, f"dist/releases/v{VERSION}")

CRATE_NAMES = [
    ("mantrac", "Mantra primary compiler executable and driver"),
    ("mantrac_lexer", "High-speed lexical tokenizer with source span tracking"),
    ("mantrac_parser", "Recursive-descent AST parser with Pratt operator precedence"),
    ("mantrac_ast", "Strongly-typed Abstract Syntax Tree definitions"),
    ("mantrac_checker", "Static semantic analyzer and type checker"),
    ("mantrac_ir", "SSA-based intermediate representation and optimizer"),
    ("mantrac_codegen", "Native C99 emitter and Clang/LLVM machine code generator"),
    ("mantrac_vm", "Native bytecode interpreter and virtual machine"),
    ("mantra_bootstrap", "Self-hosting stage-0 bootstrap toolchain"),
    ("mantra_mdk", "Mantra Developer Kit CLI (create, run, build, test, audit)"),
    ("mantra_lsp", "Language Server Protocol daemon for modern IDEs"),
    ("mantra_debugger", "Interactive runtime debugger and state inspector"),
    ("mantra_web", "Async HTTP web server, routing, and WebAssembly engine"),
    ("mantra_rest", "RESTful API framework and high-performance JSON serializer"),
    ("mantra_graphql", "Native GraphQL schema builder and query execution engine"),
    ("mantra_mobile", "Hybrid mobile platform bridges for Android and iOS"),
    ("mantra_sql_orm", "Enterprise SQL ORM (Oracle, MSSQL, MongoDB, SQLite, Postgres)"),
]

def generate_spdx():
    packages = []
    doc_spdx_id = "SPDXRef-DOCUMENT"

    packages.append({
        "name": "mantra_programming_languague",
        "SPDXID": "SPDXRef-Package-mantra-toolchain",
        "versionInfo": VERSION,
        "supplier": "Organization: Solotron Technologies Private Limited",
        "downloadLocation": f"https://github.com/solotrondigital/mantra_programming_language/releases/tag/v{VERSION}",
        "filesAnalyzed": False,
        "licenseConcluded": "MIT OR GPL-3.0",
        "licenseDeclared": "MIT OR GPL-3.0",
        "copyrightText": "Copyright (c) 2026 Solotron Technologies Private Limited",
        "description": f"Mantra v{VERSION} — Mantra a Programming Language"
    })

    relationships = []

    for name, desc in CRATE_NAMES:
        spdx_id = f"SPDXRef-Package-{name.replace('_', '-')}"
        packages.append({
            "name": name,
            "SPDXID": spdx_id,
            "versionInfo": VERSION,
            "supplier": "Organization: Solotron Technologies Private Limited",
            "downloadLocation": "NOASSERTION",
            "filesAnalyzed": False,
            "licenseConcluded": "MIT OR GPL-3.0",
            "licenseDeclared": "MIT OR GPL-3.0",
            "copyrightText": "Copyright (c) 2026 Solotron Technologies Private Limited",
            "description": desc
        })
        relationships.append({
            "spdxElementId": "SPDXRef-Package-mantra-toolchain",
            "relatedSpdxElement": spdx_id,
            "relationshipType": "CONTAINS"
        })

    spdx = {
        "spdxVersion": "SPDX-2.3",
        "dataLicense": "CC0-1.0",
        "SPDXID": doc_spdx_id,
        "name": f"Mantra-Toolchain-v{VERSION}-SBOM",
        "documentNamespace": f"https://solotron.in/spdx/mantra-toolchain-v{VERSION}-{uuid.uuid4()}",
        "creationInfo": {
            "created": datetime.now(timezone.utc).isoformat(),
            "creators": [f"Tool: mantra-sbom-generator-{VERSION}", "Organization: Solotron Technologies Private Limited"]
        },
        "packages": packages,
        "relationships": relationships
    }

    out_path = os.path.join(DIST_DIR, "sbom-spdx.json")
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(spdx, f, indent=2)
    print(f"✓ Generated SPDX 2.3 SBOM: {out_path} (Version: {VERSION})")

def generate_cyclonedx():
    components = []
    for name, desc in CRATE_NAMES:
        components.append({
            "type": "library",
            "name": name,
            "version": VERSION,
            "description": desc,
            "scope": "required",
            "licenses": [{"license": {"id": "MIT"}}, {"license": {"id": "GPL-3.0"}}],
            "purl": f"pkg:cargo/{name}@{VERSION}",
            "supplier": {"name": "Solotron Technologies Private Limited"}
        })

    cyclonedx = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.5",
        "serialNumber": f"urn:uuid:{uuid.uuid4()}",
        "version": 1,
        "metadata": {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "tools": [{"vendor": "Solotron Technologies Private Limited", "name": "mantra-sbom", "version": VERSION}],
            "component": {
                "type": "application",
                "name": "mantra_programming_languague",
                "version": VERSION,
                "description": f"Mantra v{VERSION} — Mantra a Programming Language",
                "licenses": [{"license": {"id": "MIT"}}, {"license": {"id": "GPL-3.0"}}]
            }
        },
        "components": components
    }

    out_path = os.path.join(DIST_DIR, "sbom-cyclonedx.json")
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(cyclonedx, f, indent=2)
    print(f"✓ Generated CycloneDX 1.5 SBOM: {out_path} (Version: {VERSION})")

def main():
    os.makedirs(DIST_DIR, exist_ok=True)
    generate_spdx()
    generate_cyclonedx()

if __name__ == "__main__":
    main()
