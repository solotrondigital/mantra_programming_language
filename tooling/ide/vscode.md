# Mantra IDE Integrations — Visual Studio Code & Editors

Official IDE ecosystem for the **Mantra Programming Language** (`.mantra`), developed and maintained by **[Solotron Technologies Private Limited](http://www.solotrontech.com)**.
Publisher ID: **`solotrondigital`**

---

## 📦 Packaged Extension Bundles (.vsix & .vstx)

Both extensions are built, packaged, and ready to install:

| Extension | Version | VSIX Package | VSTX Bundle |
| :--- | :--- | :--- | :--- |
| **Mantra Language Support** | `v0.5.0` | [`mantra-language-support-0.5.0.vsix`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/tooling/ide/vscode/mantra-language-support-0.5.0.vsix) | [`mantra-language-support-0.5.0.vstx`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/tooling/ide/vscode/mantra-language-support-0.5.0.vstx) |
| **Mantra CLI & Project Manager** | `v0.6.0` | [`mantra-cli-0.6.0.vsix`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/tooling/ide/vscode-cli/mantra-cli-0.6.0.vsix) | [`mantra-cli-0.6.0.vstx`](file:///Users/user/Documents/workspace/Mantra-Development/mantra_programming_language/tooling/ide/vscode-cli/mantra-cli-0.6.0.vstx) |

---

## 🚀 Installation Commands

### Method 1: Using Mantra CLI (Recommended — Installs All Extensions)
Run the following single command from your terminal:
```bash
mantra ide install vscode
```
*This automatically builds and copies both extensions (`Language Support` + `CLI Manager`) into `~/.vscode/extensions` and `~/.antigravity-ide/extensions`.*

---

### Method 2: Installing via VS Code `code` CLI

To install the **Language Support** extension:
```bash
code --install-extension mantra_programming_language/tooling/ide/vscode/mantra-language-support-0.5.0.vsix
```

To install the **Mantra CLI** extension:
```bash
code --install-extension mantra_programming_language/tooling/ide/vscode-cli/mantra-cli-0.6.0.vsix
```

*(You can also use the `.vstx` files directly if required by your tooling).*

---

### Method 3: Installing from VS Code UI
1. Open Visual Studio Code or Antigravity IDE.
2. Press <kbd>Cmd</kbd> + <kbd>Shift</kbd> + <kbd>X</kbd> (or <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>X</kbd>) to open the **Extensions** pane.
3. Click the **`...`** (Views and More Actions) menu in the top right corner.
4. Select **Install from VSIX...**.
5. Select `mantra_programming_language/tooling/ide/vscode/mantra-language-support-0.5.0.vsix`.
6. Repeat for `mantra_programming_language/tooling/ide/vscode-cli/mantra-cli-0.6.0.vsix`.

---

## 🌐 Publishing to Registries

### 1. Open VSX (Eclipse Foundation)
* **Namespace**: `solotrondigital` (matches your Open VSX account `@solotrondigital`)
* **Web Upload**: [https://open-vsx.org/publish](https://open-vsx.org/publish)
* **CLI Upload**:
  ```bash
  npx ovsx publish mantra_programming_language/tooling/ide/vscode/mantra-language-support-0.5.0.vsix -p <OPEN_VSX_TOKEN>
  npx ovsx publish mantra_programming_language/tooling/ide/vscode-cli/mantra-cli-0.6.0.vsix -p <OPEN_VSX_TOKEN>
  ```

### 2. Visual Studio Marketplace (Microsoft)
* **Publisher ID**: `solotrondigital` (managed under `solotron.digital@outlook.com`)
* **Portal**: [https://marketplace.visualstudio.com/manage](https://marketplace.visualstudio.com/manage)
* **CLI Upload**:
  ```bash
  npx vsce publish --packagePath mantra_programming_language/tooling/ide/vscode/mantra-language-support-0.5.0.vsix -p <VS_MARKETPLACE_TOKEN>
  npx vsce publish --packagePath mantra_programming_language/tooling/ide/vscode-cli/mantra-cli-0.6.0.vsix -p <VS_MARKETPLACE_TOKEN>
  ```
