#!/usr/bin/env python3
"""Compare local VS Code extension versions with their published registry versions."""

from __future__ import annotations

import json
import os
import re
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any


MARKETPLACE_URL = "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery?api-version=7.2-preview.1"
OPEN_VSX_URL = "https://open-vsx.org/api/{namespace}/{name}"


def request_json(request: urllib.request.Request) -> Any:
    try:
        with urllib.request.urlopen(request, timeout=20) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return None
        raise RuntimeError(f"Registry request failed ({error.code}): {request.full_url}") from error
    except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as error:
        raise RuntimeError(f"Could not read registry response from {request.full_url}: {error}") from error


def marketplace_versions(extension_id: str) -> list[str]:
    body = {
        "filters": [{"criteria": [{"filterType": 7, "value": extension_id}]}],
        # IncludeVersions (0x1) is required for the published version list.
        "flags": 1,
    }
    request = urllib.request.Request(
        MARKETPLACE_URL,
        data=json.dumps(body).encode("utf-8"),
        headers={
            "Accept": "application/json;api-version=7.2-preview.1",
            "Content-Type": "application/json",
            "User-Agent": "mantra-release-version-check/1.0",
        },
        method="POST",
    )
    payload = request_json(request)
    if payload is None:
        return []
    try:
        extensions = payload["results"][0]["extensions"]
        if not extensions:
            return []
        extension = next(
            (item for item in extensions if f"{item['publisher']['publisherName']}.{item['extensionName']}".lower() == extension_id.lower()),
            None,
        )
        return [item["version"] for item in extension.get("versions", [])] if extension else []
    except (KeyError, IndexError, TypeError) as error:
        raise RuntimeError(f"Unexpected Visual Studio Marketplace response for {extension_id}") from error


def open_vsx_versions(namespace: str, name: str) -> list[str]:
    url = OPEN_VSX_URL.format(
        namespace=urllib.parse.quote(namespace, safe=""),
        name=urllib.parse.quote(name, safe=""),
    )
    request = urllib.request.Request(url, headers={"Accept": "application/json", "User-Agent": "mantra-release-version-check/1.0"})
    payload = request_json(request)
    if payload is None:
        return []
    if not isinstance(payload, dict):
        raise RuntimeError(f"Unexpected Open VSX response for {namespace}.{name}")
    version = payload.get("version") or payload.get("latestVersion")
    if version:
        return [str(version)]
    # The metadata API normally exposes the current version directly. Keep a
    # clear failure if its response shape changes instead of publishing blindly.
    raise RuntimeError(f"Open VSX response for {namespace}.{name} did not include a version")


def semver_key(version: str) -> tuple[tuple[int, ...], int, tuple[tuple[int, Any], ...]]:
    match = re.fullmatch(r"v?(\d+(?:\.\d+)*)(?:-([0-9A-Za-z.-]+))?(?:\+[0-9A-Za-z.-]+)?", version)
    if not match:
        raise RuntimeError(f"Cannot compare non-SemVer extension version: {version!r}")
    numbers = tuple(int(part) for part in match.group(1).split("."))
    prerelease = match.group(2)
    if prerelease is None:
        return numbers, 1, ()
    parts: list[tuple[int, Any]] = []
    for part in prerelease.split("."):
        parts.append((0, int(part)) if part.isdigit() else (1, part.lower()))
    return numbers, 0, tuple(parts)


def published_version(versions: list[str]) -> str:
    return max(versions, key=semver_key) if versions else ""


def should_publish(candidate: str, current: str, force: bool) -> bool:
    if force:
        return True
    if not current:
        return True
    candidate_key, current_key = semver_key(candidate), semver_key(current)
    if candidate_key < current_key:
        raise RuntimeError(f"Local version {candidate} is older than the registry version {current}")
    return candidate_key > current_key


def main() -> int:
    force = os.environ.get("FORCE_PUBLISH", "false").lower() == "true"
    output_path = os.environ.get("GITHUB_OUTPUT")
    results: dict[str, str] = {}
    extensions = [
        ("lang", Path("tooling/ide/vscode/package.json")),
        ("cli", Path("tooling/ide/vscode-cli/package.json")),
    ]

    for key, path in extensions:
        package = json.loads(path.read_text(encoding="utf-8"))
        candidate = str(package["version"])
        extension_id = f"{package['publisher']}.{package['name']}"
        namespace = str(package["publisher"])
        name = str(package["name"])
        marketplace_current = published_version(marketplace_versions(extension_id))
        open_vsx_current = published_version(open_vsx_versions(namespace, name))
        print(f"{extension_id}: candidate={candidate}")
        print(f"  Visual Studio Marketplace published version: {marketplace_current or '(not published)'}")
        print(f"  Open VSX published version: {open_vsx_current or '(not published)'}")
        results[f"{key}_marketplace_version"] = marketplace_current
        results[f"{key}_open_vsx_version"] = open_vsx_current
        results[f"publish_{key}_marketplace"] = str(should_publish(candidate, marketplace_current, force)).lower()
        results[f"publish_{key}_open_vsx"] = str(should_publish(candidate, open_vsx_current, force)).lower()
        results[f"{key}_version"] = candidate

    if output_path:
        with open(output_path, "a", encoding="utf-8") as output:
            for key, value in results.items():
                output.write(f"{key}={value}\n")
    else:
        print(json.dumps(results, indent=2))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, json.JSONDecodeError, KeyError, RuntimeError) as error:
        print(f"::error::{error}", file=sys.stderr)
        raise SystemExit(1)
