-- Mantra Language Server (mantra-lsp) configuration for Neovim (0.8+)
-- Supports nvim-lspconfig and native vim.lsp client.

local M = {}

-- 1. Register .mantra filetype detection
vim.filetype.add({
  extension = {
    mantra = "mantra",
  },
})

-- 2. Buffer settings for Mantra (4 spaces, tabs illegal)
vim.api.nvim_create_autocmd("FileType", {
  pattern = "mantra",
  callback = function()
    vim.bo.commentstring = "# %s"
    vim.bo.expandtab = true
    vim.bo.shiftwidth = 4
    vim.bo.tabstop = 4
    vim.bo.softtabstop = 4
  end,
})

-- 3. LSP Server setup function
M.setup = function(opts)
  opts = opts or {}
  local cmd = opts.cmd or "mantra-lsp"

  local lspconfig_ok, lspconfig = pcall(require, "lspconfig")
  if not lspconfig_ok then
    -- Fallback to native vim.lsp.start if lspconfig is absent
    vim.api.nvim_create_autocmd("FileType", {
      pattern = "mantra",
      callback = function(ev)
        vim.lsp.start({
          name = "mantra-lsp",
          cmd = { cmd, "--stdio" },
          root_dir = vim.fs.root(ev.buf, { "mantra.toml", ".git" }),
          capabilities = opts.capabilities or vim.lsp.protocol.make_client_capabilities(),
          on_attach = opts.on_attach,
        })
      end,
    })
    return
  end

  local configs = require("lspconfig.configs")
  if not configs.mantra_lsp then
    configs.mantra_lsp = {
      default_config = {
        cmd = { cmd, "--stdio" },
        filetypes = { "mantra" },
        root_dir = lspconfig.util.root_pattern("mantra.toml", ".git"),
        settings = {},
        init_options = {},
        capabilities = opts.capabilities or vim.lsp.protocol.make_client_capabilities(),
      },
      docs = {
        description = "Mantra Language Server Protocol (mantra-lsp) integration.",
      },
    }
  end

  lspconfig.mantra_lsp.setup(vim.tbl_deep_extend("force", {
    on_attach = opts.on_attach,
    capabilities = opts.capabilities,
  }, opts.lsp_opts or {}))
end

return M

