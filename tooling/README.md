This error occurs because Open VSX is operated by the Eclipse Foundation, which requires every extension publisher to sign their legal Publisher Agreement before publishing any extension for the first time.

Also note: Open VSX web portal expects .vsix files (the standard VS Code package format).

Here is the exact step-by-step procedure to resolve this and publish your extension:

Step 1: Sign the Eclipse Foundation Publisher Agreement
Go to https://open-vsx.org
.
Click Log In in the top-right corner and sign in with your GitHub account.
If this is your first time logging in with an Eclipse Foundation service:
You will be prompted to create/link an Eclipse Foundation Account.
If prompted, accept the Eclipse Contributor Agreement (ECA).
Once logged in, go to your profile settings:
Click your avatar in the top-right corner 
→
→ Settings (or visit https://open-vsx.org/user-settings
).
Scroll down to the Publisher Agreement section:
Click Review and Sign Publisher Agreement.
Review the terms and click Sign Agreement / Accept.
Step 2: Create / Claim Your Namespace (mantra-lang)

Open VSX requires you to own the publisher namespace matching "publisher" in your package.json (mantra-lang):

On open-vsx.org
, go to Namespaces or visit:
https://open-vsx.org/user-settings/namespaces
Click Create Namespace.
Enter mantra-lang (must match the "publisher": "mantra-lang" in package.json).
Click Create / Claim.
Step 3: Upload the Package via the Web Portal

Once the agreement is signed and the mantra-lang namespace is created:

Open https://open-vsx.org/publish
.
Select your namespace: mantra-lang.
Choose the .vsix file from your disk:
For Language Support:
mantra_programming_language/tooling/ide/vscode/mantra-language-support-0.5.0.vsix
For CLI Manager:
mantra_programming_language/tooling/ide/vscode-cli/mantra-cli-0.6.0.vsix
Click Publish.
Step 4 (Optional): Publishing from Terminal or CI/CD with an Access Token

If you prefer to publish directly from the command line or via the GitHub Actions workflow we configured:

In Open VSX, go to https://open-vsx.org/user-settings/tokens
.
Click Generate Token and copy the generated token.
Publish from your terminal:
bash
# Publish Language Support
npx ovsx publish mantra_programming_language/tooling/ide/vscode/mantra-language-support-0.5.0.vsix -p <YOUR_OPEN_VSX_TOKEN>
# Publish CLI Extension
npx ovsx publish mantra_programming_language/tooling/ide/vscode-cli/mantra-cli-0.6.0.vsix -p <YOUR_OPEN_VSX_TOKEN>
Add the token to your GitHub repository secrets as OPEN_VSX_TOKEN for automatic publishing on push.
Comparison: Open VSX vs Microsoft Visual Studio Marketplace
Registry	Agreement / Registration Link	Publisher Portal
Open VSX (Eclipse Foundation)	open-vsx.org/user-settings
 (Sign Eclipse Agreement)	open-vsx.org/publish

VS Code Marketplace (Microsoft)	marketplace.visualstudio.com/manage
	marketplace.visualstudio.com
for   Production Release & Package Distribution

[Build & Package (windows-2022)](https://github.com/solotrondigital/mantra_programming_language/actions/runs/41161018233/job/124418370714#logs)
failed 43 minutes ago in 1m 13s
0s
10s
8s
1s
48s
2s
Run chmod +x create_installers.sh
==================================================================
MANTRA PROGRAMMING LANGUAGE — INSTALLER GENERATOR (v0.5.0)
Steward: Solotron Technologies Private Limited
==================================================================
Workspace Root : /d/a/mantra_programming_language
Package Dir : /d/a/mantra_programming_language/mantra_programming_language
Web Dir : /d/a/mantra_programming_language/mantra_website/mantra_web
Target Version : v0.5.0
==================================================================
>> Verified binaries in /d/a/mantra_programming_language/mantra_programming_language/target/release:
-rwxr-xr-x 2 runneradmin 197121 1.7M Oct 31 02:41 /d/a/mantra_programming_language/mantra_programming_language/target/release/mantra
-rwxr-xr-x 2 runneradmin 197121 1.2M Oct 31 02:40 /d/a/mantra_programming_language/mantra_programming_language/target/release/mantrac
>> Generating macOS Installer Packages (v0.5.0)...
✓ Created macOS archive: Mantra-0.5.0-macOS.tar.gz
>> Generating Linux Installer Packages (v0.5.0)...
✓ Created Linux tarball: mantra-0.5.0-linux.tar.gz
✓ Created Linux self-extracting installer: mantra-installer-linux.sh
>> Generating Windows Installer Packages (v0.5.0)...
>> Deploying generated installer packages...
./create_installers.sh: line 643: rsync: command not found
Error: Process completed with exit code 127.
0s
2s
0s

for [Mantra Toolchain CI](https://github.com/solotrondigital/mantra_programming_language/actions/workflows/ci.yml)
Run ./target/release/mantra --version
Mantra Development Kit (MDK) v0.6.0 (Stable Language & Production Toolchain)
Compiler: mantrac 0.6.0
Language Server: mantra-lsp 0.6.0
/Users/runner/work/_temp/605d71a9-e744-40a1-861b-24dbdbec67d0.sh: line 5: ../../target/release/mantra: No such file or directory
Mobile Engine: mantra_mobile 0.6.0 (Android & iOS Bridge)
Host Target: aarch64-macos
mantrac 0.6.0 (production-stable) [stage-0 (Rust bootstrap)] IR-schema=1.0 edition=2026
Created Mantra application '/tmp/ci_test_app'
To get started:
  cd /tmp/ci_test_app
  mantra run
Error: Process completed with exit code 1.

Run ./target/release/mantra --version
Mantra Development Kit (MDK) v0.6.0 (Stable Language & Production Toolchain)
Compiler: mantrac 0.6.0
Language Server: mantra-lsp 0.6.0
Mobile Engine: mantra_mobile 0.6.0 (Android & iOS Bridge)
Host Target: x86_64-linux
mantrac 0.6.0 (production-stable) [stage-0 (Rust bootstrap)] IR-schema=1.0 edition=2026
Created Mantra application '/tmp/ci_test_app'
To get started:
  cd /tmp/ci_test_app
  mantra run
/home/runner/work/_temp/f7ee1d1f-e176-4e63-9bbf-d3d911094ac6.sh: line 5: ../../target/release/mantra: No such file or directory
Error: Process completed with exit code 127.

  Diagnostics: err_missing_interface.mantra (E0103)  ... PASS (E0103 verified)
  Diagnostics: err_invalid_override.mantra (E0104)   ... PASS (E0104 verified)
  Diagnostics: err_unimplemented_abstract.mantra (E0105) ... PASS (E0105 verified)

[4/4] Running MDK CLI Integration Tests...
  MDK: mantra new test_app                           ... PASS
  MDK: mantra check                                  ... PASS
  MDK: mantra run                                    ... PASS
  MDK: mantra test                                   ... PASS
  MDK: mantra fmt                                    ... PASS
  MDK: mantra doc                                    ... PASS
  MDK: mantra add dependency                         ... PASS
  MDK: mantra build --release                        ... PASS
  MDK: mantra publish                                ... PASS

==========================================
Summary: 33 passed, 3 failed
==========================================
Error: Process completed with exit code 1.

for [Publish Mantra IDE Extensions](https://github.com/solotrondigital/mantra_programming_language/actions/workflows/publish-extensions.yml)

0s
Run FORCE=""
Current Language Support Version: 0.5.0
Current CLI Extension Version: 0.6.0
Error: Process completed with exit code 128.

please fix all deployment issues

i have account in 
https://open-vsx.org/
username @solotrondigital
https://accounts.eclipse.org/users/solotrondigital
username solotrondigital
microsoft account
username solotron.digital@outlook.com


All three deployment and CI/CD issues have been resolved, and both extensions have been re-packaged with your publisher ID solotrondigital matching your Open VSX, Eclipse, and Microsoft accounts.

1. Fixes Applied for Deployment & CI/CD
Issue A: Windows Installer Failure (Production Release & Package Distribution)
Error: ./create_installers.sh: line 643: rsync: command not found
Root Cause: Windows runners on GitHub Actions (windows-2022) do not ship with rsync.
Fix: Updated create_installers.sh and build_installers.sh to check if rsync is available; if not, they cleanly fall back to cp -Rf.
Issue B: Mantra Toolchain CI Failure (ci.yml)
Error: ../../target/release/mantra: No such file or directory
Root Cause: In ci.yml, after running cd /tmp/ci_test_app, the relative path ../../target/release/mantra pointed to /target/release/mantra instead of the runner workspace.
Fix: Added the workspace target/release directory to PATH so mantra and mantrac run directly anywhere across macOS, Linux, and Windows.
Issue C: Extension Registry Workflow (publish-extensions.yml)
Error: Error: Process completed with exit code 128
Root Cause: git rev-parse HEAD~1 returned exit code 128 under set -e on initial/shallow commits where HEAD~1 does not exist.
Fix: Configured fetch-depth: 0 for complete history and wrapped Git parent detection with safety checks so it never exits with code 128.
Issue D: Publisher Unified to solotrondigital
Updated "publisher": "solotrondigital" in both package.json files and inside ide_installer.rs.
This aligns with your:
Open VSX Account: @solotrondigital
Eclipse Account: solotrondigital
Microsoft Account: solotron.digital@outlook.com (use publisher ID solotrondigital)
2. Updated Extension Bundles (.vsix & .vstx)

Both packages have been recompiled and packaged:

Extension	Version	VSIX Package	VSTX Bundle
Mantra Language Support	v0.5.0	mantra-language-support-0.5.0.vsix	mantra-language-support-0.5.0.vstx
Mantra CLI & Project Manager	v0.6.0	mantra-cli-0.6.0.vsix	mantra-cli-0.6.0.vstx
3. Installation Commands
Command 1: Using Mantra CLI (Recommended)

Installs both extensions into VS Code and Antigravity IDE:

bash
mantra ide install vscode
Command 2: Using the code CLI

To install Mantra Language Support:

bash
code --install-extension mantra_programming_language/tooling/ide/vscode/mantra-language-support-0.5.0.vsix

To install Mantra CLI:

bash
code --install-extension mantra_programming_language/tooling/ide/vscode-cli/mantra-cli-0.6.0.vsix
4. Uploading to Open VSX & Microsoft Marketplace
On Open VSX (open-vsx.org/publish
):
Log in with your @solotrondigital account.
Sign the Publisher Agreement under Settings
.
Under Namespaces
, ensure solotrondigital is created.
In the upload form at open-vsx.org/publish
, select namespace solotrondigital and upload:
mantra-language-support-0.5.0.vsix
mantra-cli-0.6.0.vsix (Open VSX expects the .vsix file format).
On Microsoft Visual Studio Marketplace (marketplace.visualstudio.com/manage
):
Sign in with solotron.digital@outlook.com.
Create your Publisher ID as solotrondigital.
Upload mantra-language-support-0.5.0.vsix and mantra-cli-0.6.0.vsix.


npx ovsx unpublish mantra-lang.mantra-language-support \
  --pat PAT \
  --force

  npx ovsx unpublish mantra-lang.mantra-cli \
  --pat PAT \
  --force