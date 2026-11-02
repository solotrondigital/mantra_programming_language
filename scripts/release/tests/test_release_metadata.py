import hashlib
import json
import tempfile
import unittest
from pathlib import Path

from scripts.release import generate_manifest, generate_sbom


class ReleaseMetadataTests(unittest.TestCase):
    def test_manifest_uses_actual_artifact_bytes_and_workspace_crates(self):
        with tempfile.TemporaryDirectory() as tmp:
            old_dir = generate_manifest.DIST_DIR
            generate_manifest.DIST_DIR = tmp
            try:
                artifact = Path(tmp) / "toolchain.tar.gz"
                artifact.write_bytes(b"verified artifact bytes")
                generate_manifest.main()

                manifest = json.loads((Path(tmp) / "release.json").read_text())
                row = next(item for item in manifest["artifacts"] if item["name"] == artifact.name)
                self.assertEqual(row["size_bytes"], artifact.stat().st_size)
                self.assertEqual(row["sha256"], hashlib.sha256(artifact.read_bytes()).hexdigest())
                self.assertEqual(row["sha512"], hashlib.sha512(artifact.read_bytes()).hexdigest())
                self.assertIn("mantra_agents", manifest["crates_included"])
                self.assertNotEqual(row["sha256"], "0" * 64)
            finally:
                generate_manifest.DIST_DIR = old_dir

    def test_manifest_refuses_to_invent_missing_release_artifacts(self):
        with tempfile.TemporaryDirectory() as tmp:
            old_dir = generate_manifest.DIST_DIR
            generate_manifest.DIST_DIR = tmp
            try:
                with self.assertRaises(SystemExit):
                    generate_manifest.main()
            finally:
                generate_manifest.DIST_DIR = old_dir

    def test_sbom_contains_current_workspace_members(self):
        with tempfile.TemporaryDirectory() as tmp:
            old_dir = generate_sbom.DIST_DIR
            generate_sbom.DIST_DIR = tmp
            try:
                generate_sbom.main()
                spdx = json.loads((Path(tmp) / "sbom-spdx.json").read_text())
                cyclonedx = json.loads((Path(tmp) / "sbom-cyclonedx.json").read_text())
                spdx_names = {package["name"] for package in spdx["packages"]}
                cyclone_names = {component["name"] for component in cyclonedx["components"]}
                self.assertIn("mantra_ml", spdx_names)
                self.assertIn("mantra_ml", cyclone_names)
                self.assertEqual(len(cyclone_names), len(generate_sbom.get_workspace_crates()))
            finally:
                generate_sbom.DIST_DIR = old_dir


if __name__ == "__main__":
    unittest.main()
