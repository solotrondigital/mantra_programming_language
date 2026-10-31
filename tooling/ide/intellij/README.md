# Mantra IntelliJ & Android Studio Plugin

Provides Mantra language support for:
- IntelliJ IDEA (Ultimate & Community)
- Android Studio
- CLion
- RustRover

## Building

```bash
./gradlew buildPlugin
```

The resulting plugin `.zip` will be placed in `build/distributions/`.

## Manual Installation

1. In IntelliJ IDEA / Android Studio, open **Settings / Preferences** (`Cmd+,` on macOS, `Ctrl+Alt+S` on Linux/Windows).
2. Go to **Plugins** > ⚙️ Gear Icon > **Install Plugin from Disk...**
3. Select the built `.zip` archive.
4. Restart your IDE.
5. Ensure `mantra-lsp` is on your system `$PATH`.

