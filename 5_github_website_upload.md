# Mantra Project Upload & Deployment Guide

This guide details exactly which folders and files to upload to **GitHub** (source code repository) and which folders to upload to **Websites / Web Hosting** (static hosting platforms like GitHub Pages, Vercel, Netlify, or Apache/Nginx).

---

## 1. Uploading to GitHub (Source Code Repository)

GitHub is for your **source code, documentation, build configurations, and examples**. 

### ✅ Folders & Files to UPLOAD to GitHub:

| Folder / File | Description | Purpose |
| :--- | :--- | :--- |
| **`crates/`** | Core Rust compiler, runtime, and libraries | Contains all 11+ workspace crates (`mantrac`, `mantra`, `mantra_ast`, `mantra_parser`, `mantra_codegen`, `mantra_mdk`, `mantra_stdlib`, `mantra_sql_orm`, `mantra_web`, `mantra_rest`). **Required.** |
| **`mantra_web/`** | Official Website Source | Landing page, download page, examples showcase, and runnable examples. |
| **`mantra_docs/`** | Official Documentation Portal | Full HTML and markdown documentation for v0.1.0 and v0.2.0 releases. |
| **`examples/`** | Language Examples | Standalone example scripts (`01_hello_world.mantra` to `05_web_middleware.mantra`). |
| **`installer/`** | Installer Scripts | Shell, batch, and PowerShell scripts for installing Mantra across macOS, Linux, and Windows. *(Exclude binaries)* |
| **`tests/`** | Test Suite | Integration and end-to-end tests for compiler and runtime. |
| **`assets/`** | Static Media | Project logos, architecture diagrams, and images. |
| **`my_app/`, `my_rest_app/`** | Starter Project Templates | Sample starter projects created with `mantra init`. |
| **`Cargo.toml` & `Cargo.lock`** | Rust Workspace Config | Workspace crate declarations and locked dependency versions. |
| **`LICENSE`, `LICENSE-MIT`, `LICENSE-GPL`** | Open Source Licenses | MIT & GNU GPL v3.0 dual licenses by Solotron Technologies Private Limited. |
| **`*.md`** | Project Documentation | `README.md`, `1-Implentation.md`, `2_run.md`, `3_project.md`, `4_first_project.md`, `5_github_website_upload.md`. |
| **`.gitignore`** | Git Ignore File | Prevents pushing build artifacts and caches. |

---

### ❌ Folders & Files to NEVER UPLOAD to Git / GitHub:

> [!CAUTION]
> Adding these files to Git can cause repo bloat, hit GitHub's 100MB file limit, or fail git pushes.

1. **`target/`** ⚠️ **DO NOT COMMIT**
   - The compiled Rust build cache (can exceed 500MB - 10GB+).
2. **`dist/`** 
   - Pre-packaged installer binaries (e.g., `Mantra-0.1.0-macOS.dmg`).
   - *Best Practice*: Upload these to **GitHub Releases** as attached release assets instead of committing them to git.
3. **Compiled binaries inside `installer/`**:
   - `installer/bin/mantra`, `installer/bin/mantrac`
   - `installer/macos/*.dmg`
   - `installer/linux/*.tar.gz`
4. **`.DS_Store` / `Thumbs.db`**
   - Operating system folder preview caches.

---

### Quick Git Initialization Commands:

```bash
cd /Users/user/Documents/workspace/Mantra-Development

# 1. Initialize git
git init

# 2. Add all source files (target/ is safely ignored by .gitignore)
git add .

# 3. Create your first commit
git commit -m "Initial commit of Mantra v0.2.0 language, compiler, stdlib, docs, and web"

# 4. Link to your GitHub repository
git remote add origin https://github.com/<your-username>/mantra.git
git branch -M main
git push -u origin main
```

---

## 2. Uploading to Websites (Hosting Platforms)

If you are deploying the Mantra public website and documentation online (e.g., GitHub Pages, Cloudflare Pages, Netlify, Vercel, or custom server like Nginx/Apache):

### 🌐 The Website Folders:

The web experience is composed of two directories:
1. **`mantra_web/`**: The primary landing website:
   - `index.html` — Homepage
   - `why-mantra.html` — Language philosophy & Rust vs. C++ architecture analysis
   - `download.html` — Download page
   - `examples.html` — Interactive example catalog
   - `assets/` — Stylesheets, scripts, fonts, icons
   - `examples/` — Runnable `.mantra` source examples
2. **`mantra_docs/`**: The documentation hub:
   - `index.html` — Multi-version documentation index
   - `v0.1.0/` — Core language standard library docs
   - `v0.2.0/` — Web, REST, and ORM docs
   - `assets/` & `includes/` — Docs CSS/JS components

---

### Deployment Layout Options

#### Option A: Unified Hosting Root (Recommended for Production)
Deploy both folders together so all cross-links (`../mantra_docs/` and `../mantra_web/`) work seamlessly:

```
public_html/ (web root)
├── mantra_web/       --> Main landing page (or configure domain to point here)
│   ├── index.html
│   ├── download.html
│   ├── examples.html
│   ├── assets/
│   └── examples/
└── mantra_docs/      --> Documentation hub
    ├── index.html
    ├── v0.1.0/
    ├── v0.2.0/
    ├── assets/
    └── includes/
```

#### Option B: Clean Domain Root (`https://mantralang.org/`)
If you want `mantra_web` files directly at `https://your-domain.com/`:
1. Put contents of `mantra_web/` directly in the root:
   - `/index.html`
   - `/download.html`
   - `/examples.html`
   - `/assets/`
   - `/examples/`
2. Place `mantra_docs/` inside the root as `/docs/` (or `/mantra_docs/`).
3. For downloadable installers on the download page, place installer binaries in `/dist/` and `/installer/linux/`.

---

## Summary Checklist

- [ ] **GitHub**: Upload `crates/`, `mantra_web/`, `mantra_docs/`, `examples/`, `installer/` (scripts), `tests/`, `assets/`, `Cargo.toml`.
- [ ] **GitHub Ignore**: Exclude `target/`, `dist/`, `.DS_Store`.
- [ ] **Releases**: Upload `.dmg` and `.tar.gz` to GitHub Releases.
- [ ] **Website**: Deploy `mantra_web/` and `mantra_docs/`.
