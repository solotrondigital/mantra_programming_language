# Mantra Emacs Support

Major mode for editing Mantra (`.mantra`) files with built-in Eglot (LSP) integration.

## Setup

1. Place `mantra-mode.el` in your `load-path` (e.g. `~/.emacs.d/lisp/`).
2. Add the following to your `init.el`:

```elisp
(require 'mantra-mode)

;; Automatically start Eglot for Mantra buffers
(add-hook 'mantra-mode-hook #'eglot-ensure)
```

Ensure `mantra-lsp` is on your `$PATH` (e.g. via `exec-path`).

