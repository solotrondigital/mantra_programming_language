# Mantra Sublime Text Package

Provides syntax definitions and LSP integration for Sublime Text 3 and 4.

## Installation

1. Copy this folder into your Sublime Text `Packages/` directory:
   - macOS: `~/Library/Application Support/Sublime Text/Packages/Mantra`
   - Linux: `~/.config/sublime-text/Packages/Mantra`
   - Windows: `%APPDATA%\Sublime Text\Packages\Mantra`

2. For LSP support, install the **LSP** package via Package Control, and add the following to `Preferences > Package Settings > LSP > Settings`:

```json
{
  "clients": {
    "mantra-lsp": {
      "enabled": true,
      "command": ["mantra-lsp", "--stdio"],
      "selector": "source.mantra"
    }
  }
}
```

