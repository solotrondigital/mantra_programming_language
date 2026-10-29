# Example 06: Mantra WebAssembly in the Browser

This example demonstrates compiling Mantra business logic into WebAssembly (`.wasm` / `.wat`) and running it inside a web browser with the Mantra Browser Runtime.

## Project Structure

```
06_wasm_browser_app/
├── index.html            # Web user interface for browser testing
├── mantra_browser.js     # Mantra WebAssembly browser runtime and bridge
├── math_ops.mantra       # Mantra source code with exported functions
├── math_ops.wat          # Compiled WebAssembly text format
└── README.md             # Guide and instructions
```

## How It Works

1. **Mantra Source (`math_ops.mantra`)**:
   Defines pure business and calculation functions (`add`, `multiply`, `factorial`).
2. **WebAssembly Compilation**:
   Compiled using the Mantra compiler:
   ```bash
   mantrac math_ops.mantra --emit wat > math_ops.wat
   ```
3. **Browser Interoperability (`mantra_browser.js`)**:
   Provides standard environment imports (`env.print`, `env.print_str`, `env.now`), handles 64-bit integer (`BigInt`) parameter conversions, and manages linear memory string reading/writing.

## Running in the Browser

Start a local HTTP server in this directory:
```bash
# Using Python
python3 -m http.server 8000

# Or using Mantra
mantra run
```

Open your browser at `http://localhost:8000` to interact with the calculation core.

