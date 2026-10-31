Yes, you should keep ci.yml. They serve two completely different, essential purposes in software development:

Comparison: ci.yml vs. release.yml
Feature	ci.yml (Quality Gate)	release.yml (Distribution)
Primary Goal	Verifies correctness & stability	Packages & distributes to users
Tests Executed	Core conformance, VM vs Native execution, compiler diagnostics, 250-iteration fuzzing suite, stress tests, MDK CLI	None (only builds and bundles)
Platform Matrix	5 targets (macOS ARM64, macOS Intel, Ubuntu x86_64, Ubuntu ARM64, Windows MSVC)	3 host runners for packaging
Outputs	Pass / Fail status badge	GitHub Releases, .dmg, .tar.gz, .zip, Homebrew formula, Chocolatey .nupkg
When it runs	Pull Requests & commits	Pushes affecting crates/distribution
Why ci.yml is Critical for a Programming Language:
Catches Compiler & Platform Bugs Before You Ship:
For example, the Animal __super bug on Windows MSVC was caught by ci.yml's conformance test suite.
If you only had release.yml, the release workflow would build the binary and publish it to GitHub, Homebrew, and Chocolatey, but end users on Windows would experience compilation errors when trying to run or compile basic Mantra classes.
Pull Request Safety:
When contributors or team members open a PR, ci.yml tests their code across Linux, macOS, and Windows without publishing premature releases.
Recommended Improvements

If having both workflows run on every commit feels redundant or burns too many GitHub Actions runner minutes, here are the two best practices:

Option A: Trigger release.yml Only on Git Tags (Recommended Industry Standard)

Keep ci.yml running on every push to ensure code health, but change release.yml to only trigger when you create an official release tag (e.g. git tag v0.7.0 && git push origin v0.7.0):

yaml
# In .github/workflows/release.yml
on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:
Option B: Keep Both as They Are
ci.yml ensures all 36+ conformance tests and stress tests pass.
release.yml compiles installers and pushes packages to Homebrew and Chocolatey.