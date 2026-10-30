#!/usr/bin/env python3
"""
Mantra Software Bill of Materials (SBOM) Generator
scripts/release/generate_sbom.py

Generates production-grade SPDX 2.3 and CycloneDX 1.5 JSON SBOM specifications
for the Mantra v0.6.0 compiler, runtime, and all 17 standard workspace crates.
"""

import os
import sys
import json
import uuid
from datetime import datetime, timezone

VERSION = "0.6.0"
REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))
DIST_DIR = os.path.join(REPO_ROOT, f"dist/releases/v{VERSION}")

CRATES = [
    ("mantrac", "Mantra primary compiler executable and driver", "0.6.0"),
    ("mantrac_lexer", "High-speed lexical tokenizer with source span tracking", "0.6.0"),
    ("mantrac_parser", "Recursive-descent AST parser with Pratt operator precedence", "0.6.0"),
    ("mantrac_ast", "Strongly-typed Abstract Syntax Tree definitions", "0.6.0"),
    ("mantrac_checker", "Static semantic analyzer and type checker", "0.6.0"),
    ("mantrac_ir", "SSA-based intermediate representation and optimizer", "0.6.0"),
    ("mantrac_codegen", "Native C99 emitter and Clang/LLVM machine code generator", "0.6.0"),
    ("mantrac_vm", "Native bytecode interpreter and virtual machine", "0.6.0"),
    ("mantra_bootstrap", "Self-hosting stage-0 bootstrap toolchain", "0.6.0"),
    ("mantra_mdk", "Mantra Developer Kit CLI (create, run, build, test, audit)", "0.6.0"),
    ("mantra_lsp", "Language Server Protocol daemon for modern IDEs", "0.6.0"),
    ("mantra_debugger", "Interactive runtime debugger and state inspector", "0.6.0"),
    ("mantra_web", "Async HTTP web server, routing, and WebAssembly engine", "0.6.0"),
    ("mantra_rest", "RESTful API framework and high-performance JSON serializer", "0.6.0"),
    ("mantra_graphql", "Native GraphQL schema builder and query execution engine", "0.6.0"),
    ("mantra_mobile", "Hybrid mobile platform bridges for Android and iOS", "0.6.0"),
    ("mantra_sql_orm", "Enterprise SQL ORM (Oracle, MSSQL, MongoDB, SQLite, Postgres)", "0.6.0"),
]

def generate_spdx():
    packages = []
    doc_spdx_id = "SPDXRef-DOCUMENT"

    # Root Toolchain Package
    packages.append({
        "name": "mantra-toolchain",
        "SPDXID": "SPDXRef-Package-mantra-toolchain",
        "versionInfo": VERSION,
        "supplier": "Organization: Solotron Technologies",
        "downloadLocation": f"https://github.com/solotrondigital/mantra_programming_language/releases/tag/v{VERSION}",
        "filesAnalyzed": False,
        "licenseConcluded": "MIT OR GPL-3.0-only",
        "licenseDeclared": "MIT OR GPL-3.0-only",
        "copyrightText": "Copyright (c) 2026 Solotron Technologies",
        "description": "Mantra Programming Language — Stable Language Edition & Production Toolchain"
    })

    relationships = []

    for name, desc, ver in CRATES:
        spdx_id = f"SPDXRef-Package-{name.replace('_', '-')}"
        packages.append({
            "name": name,
            "SPDXID": spdx_id,
            "versionInfo": ver,
            "supplier": "Organization: Solotron Technologies",
            "downloadLocation": "NOASSERTION",
            "filesAnalyzed": False,
            "licenseConcluded": "MIT OR GPL-3.0-only",
            "licenseDeclared": "MIT OR GPL-3.0-only",
            "copyrightText": "Copyright (c) 2026 Solotron Technologies",
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
        "documentNamespace": f"https://mantra-lang.org/spdx/mantra-toolchain-v{VERSION}-{uuid.uuid4()}",
        "creationInfo": {
            "created": datetime.now(timezone.utc).isoformat(),
            "creators": ["Tool: mantra-sbom-generator-0.6.0", "Organization: Solotron Technologies"]
        },
        "packages": packages,
        "relationships": relationships
    }

    out_path = os.path.join(DIST_DIR, "sbom-spdx.json")
    with open(out_path, "w") as f:
        json.dump(spdx, f, indent=2)
    print(f"✓ Generated SPDX 2.3 SBOM: {out_path}")

def generate_cyclonedx():
    components = []
    for name, desc, ver in CRATES:
        components.append({
            "type": "library",
            "name": name,
            "version": ver,
            "description": desc,
            "scope": "required",
            "licenses": [{"license": {"id": "MIT"}}, {"license": {"id": "GPL-3.0-only"}}],
            "purl": f"pkg:cargo/{name}@{ver}",
            "supplier": {"name": "Solotron Technologies"}
        })

    cyclonedx = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.5",
        "serialNumber": f"urn:uuid:{uuid.uuid4()}",
        "version": 1,
        "metadata": {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "tools": [{"vendor": "Solotron Technologies", "name": "mantra-sbom", "version": VERSION}],
            "component": {
                "type": "application",
                "name": "mantra",
                "version": VERSION,
                "description": "Mantra Programming Language & Production Toolchain",
                "licenses": [{"license": {"id": "MIT"}}, {"license": {"id": "GPL-3.0-only"}}]
            }
        },
        "components": components
    }

    out_path = os.path.join(DIST_DIR, "sbom-cyclonedx.json")
    with open(out_path, "w") as f:
        json.dump(cyclonedx, f, indent=2)
    print(f"✓ Generated CycloneDX 1.5 SBOM: {out_path}")

def main():
    os.makedirs(DIST_DIR, exist_ok=True)
    generate_spdx()
    generate_cyclonedx()

if __name__ == "__main__":
    main()

