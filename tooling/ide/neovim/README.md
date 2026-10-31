# Mantra Neovim Integration

Official Neovim LSP setup for the Mantra programming language.

## Prerequisites

1. `mantra-lsp` installed and on your system `$PATH`:
   ```bash
   mantra-lsp --version
   ```
2. Neovim 0.8+ (0.10+ recommended for full semantic tokens).

## Installation

### Using `nvim-lspconfig`

Copy or require `mantra.lua` in your `init.lua`:

```lua
local mantra = require("mantra")

mantra.setup({
  cmd = "mantra-lsp", -- Or absolute path to binary
  on_attach = function(client, bufnr)
    local bufopts = { noremap = true, silent = true, buffer = bufnr }

    vim.keymap.set('n', 'gd', vim.lsp.buf.definition, bufopts)
    vim.keymap.set('n', 'gD', vim.lsp.buf.declaration, bufopts)
    vim.keymap.set('n', 'gi', vim.lsp.buf.implementation, bufopts)
    vim.keymap.set('n', 'gr', vim.lsp.buf.references, bufopts)
    vim.keymap.set('n', 'K', vim.lsp.buf.hover, bufopts)
    vim.keymap.set('n', '<C-k>', vim.lsp.buf.signature_help, bufopts)
    vim.keymap.set('n', '<leader>rn', vim.lsp.buf.rename, bufopts)
    vim.keymap.set('n', '<leader>ca', vim.lsp.buf.code_action, bufopts)
    vim.keymap.set('n', '<leader>f', function() vim.lsp.buf.format { async = true } end, bufopts)
  end,
})
```

## Features Supported

* `textDocument/definition` (Go to definition)
* `textDocument/references` (Find references)
* `textDocument/hover` (Hover cards with markdown syntax)
* `textDocument/completion` (IntelliSense)
* `textDocument/signatureHelp` (Parameter hints)
* `textDocument/formatting` (mantra fmt 4-space alignment)
* `textDocument/semanticTokens/full` (Semantic highlights)
* `textDocument/rename` (Project-wide symbol renaming)

