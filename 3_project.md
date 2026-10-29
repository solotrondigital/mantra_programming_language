The complete installation packages for all operating systems and developer documentation are generated and organized.

1. Installer Packages (installer/ directory)

All installation files and packages are located in installer/:

text
installer/
├── install.sh                  # Universal installer (auto-detects macOS vs Linux)
├── README.md                   # Installation guide across all OSes
├── macos/
│   ├── Mantra-0.1.0-macOS.dmg   # Apple macOS Disk Image (.dmg) Installer (7.5 MB)
│   └── install.sh              # macOS terminal installation script
├── linux/
│   ├── mantra-installer-linux.sh # Self-extracting runnable Linux installer (2.3 MB)
│   ├── mantra-0.1.0-linux.tar.gz# Portable Linux archive (2.3 MB)
│   ├── mantra.desktop           # Freedesktop application entry with logo icon
│   └── install.sh              # Linux terminal installation script
└── windows/
    ├── Mantra-Setup.exe         # Windows Setup executable (2.3 MB)
    ├── Mantra-0.1.0-windows.zip # Windows portable ZIP bundle
    ├── install.bat             # Windows Command Prompt batch installer
    └── Install-Mantra.ps1       # Windows PowerShell installer script
How to Install:
macOS: Double-click installer/macos/Mantra-0.1.0-macOS.dmg or run:
bash
./installer/macos/install.sh
Linux: Run the standalone self-extracting installer (configures /usr/local/bin, installs mantra_logo.png to /usr/share/icons/, and registers mantra.desktop):
bash
chmod +x installer/linux/mantra-installer-linux.sh
sudo ./installer/linux/mantra-installer-linux.sh
Windows: Double-click installer/windows/Mantra-Setup.exe or run Install-Mantra.ps1 in PowerShell.
2. Developer Documentation (mantra_docs/ directory)

Comprehensive documentation is available in mantra_docs/:

mantra_docs/README.md: Master documentation index
01_getting_started.md: Installation, verification, and first program
02_syntax_and_basics.md: 4-space indentation rules, variables (let, mut), control flow, loops
03_type_system.md: Int, Float, Bool, String, dynamic List, Any, and diagnostic error codes
04_object_oriented_programming.md: Classes, __init__, encapsulation (pub/private), single inheritance (extends), super(), interfaces (implements), abstract classes, and @property
05_mdk_cli_reference.md: Complete guide to mantra commands (new, run, build, test, fmt, lint, doc)
06_compiler_architecture.md: Bootstrap compiler internals, tree-walking VM, C code generator, and object layouts
07_cookbook_and_examples.md: Practical patterns (dynamic Stacks, Car-Engine composition, Factory pattern)
3. Sample Code to Write & Run
Sample 1: Hello World & Printing (my_code.mantra)
mantra
def main() -> Int:
    print("========================================")
    print("         Hello from Mantra! 🚀          ")
    print("========================================")
    let name = "Developer"
    let language = "Mantra"
    print("Welcome, " + name + "! You are writing " + language + ".")
    let x = 25
    let y = 17
    print("Calculation: " + str(x) + " + " + str(y) + " = " + str(x + y))
    return 0
Sample 2: Object-Oriented Programming & Methods
mantra
class Greeter:
    def __init__(self, greeting: String):
        self.greeting = greeting
        self.times_greeted = 0
    pub def greet(self, person: String) -> String:
        self.times_greeted = self.times_greeted + 1
        return self.greeting + ", " + person + "! (Greeted " + str(self.times_greeted) + " time(s))"
def main() -> Int:
    bot = Greeter("Welcome to Mantra")
    print(bot.greet("Alice"))
    print(bot.greet("Bob"))
    return 0
4. Commands to Run Your Code
Option 1: Instant Run with VM Interpreter (Development Mode)

No compilation wait time:

bash
mantra run my_code.mantra

or directly with the compiler:

bash
mantrac my_code.mantra --run
Option 2: Compile to a Standalone Native Machine Executable

Compiles through Clang/GCC into an optimized native executable:

bash
mantrac my_code.mantra -o my_program
./my_program
Option 3: Create a Full Application Project
bash
# 1. Create new project with full structure
mantra new my_app
# 2. Enter project folder
cd my_app
# 3. Run development code
mantra run
# 4. Run automated tests
mantra test
# 5. Build native release executable
mantra build --release
./target/release/my_app