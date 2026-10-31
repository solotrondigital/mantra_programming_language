# Changelog — Mantra Language Support

All notable changes to the **Mantra Language Support** extension will be documented in this file.

Owned & Maintained by [Solotron Technologies Private Limited](http://www.solotrontech.com).

## [0.5.0] - 2026-10-31

### Added
- Official branding with `mantra_color_logo.png`.
- Formal ownership and documentation credits to Solotron Technologies Private Limited.
- Complete Details, Features, and Dependencies documentation tabs.
- LSP 3.17 protocol capabilities:
  - Document formatting provider with 4-space indentation rule.
  - Signature help provider with parameter hints.
  - Rename provider with prepare rename support.
  - Workspace and document symbol navigation.
  - AST-driven semantic token highlighting.
  - Real-time syntax and type diagnostics.

### Fixed
- Fixed LSP initialization payload JSON syntax error (missing comma in `signatureHelpProvider` trigger characters) that caused connection crashes on startup.
- Fixed relative path bundling for grammar and theme assets inside `.vsix` packages.

