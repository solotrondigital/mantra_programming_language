;;; mantra-mode.el --- Major mode for Mantra programming language -*- lexical-binding: t; -*-

;; Author: Mantra Language Team <support@mantra-lang.dev>
;; Version: 0.5.0
;; Keywords: languages, mantra, lsp

;;; Commentary:
;; Major mode for editing Mantra (.mantra) source code, with Eglot LSP integration.

;;; Code:

(require 'eglot)

(defgroup mantra nil
  "Support for Mantra code."
  :group 'languages)

(defvar mantra-mode-syntax-table
  (let ((table (make-syntax-table)))
    ;; Comments start with '#' and end with newline
    (modify-syntax-entry ?# "<" table)
    (modify-syntax-entry ?\n ">" table)
    ;; Strings
    (modify-syntax-entry ?\" "\"" table)
    (modify-syntax-entry ?\' "\"" table)
    table)
  "Syntax table for `mantra-mode`.")

(defconst mantra-font-lock-keywords
  `((,(regexp-opt '("if" "elif" "else" "for" "while" "break" "continue" "return" "in") 'words)
     . font-lock-keyword-face)
    (,(regexp-opt '("def" "class" "interface" "module" "import" "from" "as" "let" "mut") 'words)
     . font-lock-builtin-face)
    (,(regexp-opt '("pub" "private" "protected" "static" "abstract" "override" "async" "await") 'words)
     . font-lock-type-face)
    (,(regexp-opt '("extends" "implements") 'words)
     . font-lock-keyword-face)
    (,(regexp-opt '("and" "or" "not") 'words)
     . font-lock-keyword-face)
    (,(regexp-opt '("True" "False" "None") 'words)
     . font-lock-constant-face)
    (,(regexp-opt '("self" "super") 'words)
     . font-lock-variable-name-face)
    (,(regexp-opt '("Int" "Float" "Bool" "String" "Byte" "Unit" "Any" "Option" "Result" "List" "Map" "Task") 'words)
     . font-lock-type-face)
    (,(regexp-opt '("print" "len" "range" "str" "int" "float" "assert" "read_file" "write_file" "system_exec" "get_env") 'words)
     . font-lock-function-name-face)
    ("\\bdef\\s-+\\([a-zA-Z_][a-zA-Z0-9_]*\\)" 1 font-lock-function-name-face)
    ("\\bclass\\s-+\\([A-Z][a-zA-Z0-9_]*\\)" 1 font-lock-type-face)
    ("\\binterface\\s-+\\([A-Z][a-zA-Z0-9_]*\\)" 1 font-lock-type-face))
  "Font lock keywords for `mantra-mode`.")

;;;###autoload
(define-derived-mode mantra-mode prog-mode "Mantra"
  "Major mode for editing Mantra code."
  :syntax-table mantra-mode-syntax-table
  (setq-local font-lock-defaults '(mantra-font-lock-keywords))
  (setq-local comment-start "# ")
  (setq-local comment-end "")
  (setq-local tab-width 4)
  (setq-local indent-tabs-mode nil))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.mantra\\'" . mantra-mode))

;; Register with Eglot
(with-eval-after-load 'eglot
  (add-to-list 'eglot-server-programs
               '(mantra-mode . ("mantra-lsp" "--stdio"))))

(provide 'mantra-mode)
;;; mantra-mode.el ends here

