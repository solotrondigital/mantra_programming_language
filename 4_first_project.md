The issue was that installer/macos/bin/ was not populated with the compiled executables when the installer script was executed from the repository directory rather than from inside the mounted .dmg.

What Was Fixed:
Added Binaries: Populated 
installer/macos/bin/
, 
installer/linux/bin/
, and 
installer/windows/bin/
 with the release builds of mantrac and mantra.
Multi-Location Search: Updated 
installer/macos/install.sh
 and 
installer/linux/install.sh
 to automatically check:
./bin (local package folder)
../bin (parent installer directory)
../../target/release (built binaries in repository)
Updated macOS DMG: Rebuilt 
installer/macos/Mantra-0.1.0-macOS.dmg
 with the updated structure.
How to Install Now:
In your terminal, navigate to the installer directory and run:

bash


cd /Users/user/Documents/workspace/Mantra-Development/installer
./install.sh
(If prompted, enter your macOS password for sudo to copy the binaries to /usr/local/bin).

Verifying Your Installation
Open a terminal window and run:

bash


mantra --version
mantrac --help
You should see:

text


Mantra Development Kit (MDK) v0.1.0-draft
Compiler: mantrac 0.1.0-draft
Host Target: x86_64-apple-darwin
Writing & Running Your Own Code
1. Create a sample file my_first_app.mantra:
mantra


# my_first_app.mantra
def main() -> Int:
    print("========================================")
    print("     Mantra is Installed & Working! 🚀   ")
    print("========================================")
    let name = "Developer"
    print("Welcome, " + name + "! Ready to write code.")
    # Simple loop and arithmetic
    mut n = 1
    while n <= 3:
        print("Count: " + str(n) + " - Mantra is blazing fast!")
        n = n + 1
    return 0
2. Run it immediately (VM Mode):
bash


mantra run my_first_app.mantra
or with the compiler:

bash


mantrac my_first_app.mantra --run
3. Compile to a Native Machine Executable:
bash


mantrac my_first_app.mantra -o my_first_app
./my_first_app


  mantra new my_app
  cd my_app
  mantra run
