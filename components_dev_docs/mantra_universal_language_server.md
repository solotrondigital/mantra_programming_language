MASTER PROMPT: Build Mantra IDE Ecosystem – Universal Language Server, Syntax Highlighting, IntelliSense & Navigation
1. Project Objective
Develop a complete, production-grade IDE integration ecosystem for the Mantra Programming Language.

Mantra is an independent programming language with its own compiler, lexer, parser, AST, semantic analyzer, virtual machine, and native compilation capabilities. Its compiler is primarily implemented using Rust and C++.

The objective is to make Mantra development feel as professional and intelligent as Java, Kotlin, Rust, C++, and C# development in modern IDEs.

Developers must experience rich syntax highlighting, semantic colorization, clickable classes and methods, IntelliSense, autocomplete, navigation, refactoring, diagnostics, documentation, and debugging integration.

Do not build separate compiler implementations for individual IDEs.

Build one centralized Mantra Language Intelligence Engine and expose its functionality through the Language Server Protocol (LSP).

All IDE integrations must consume the same Mantra language intelligence.
2. Supported IDEs and Editors
Provide first-class integration for the following development environments.
Tier 1 – Primary IDEs
1. Visual Studio Code
2. IntelliJ IDEA
3. Android Studio
4. CLion
5. RustRover
Tier 2 – Additional IDEs
6. Neovim
7. Sublime Text
8. Eclipse IDE
9. Emacs
10. Visual Studio (where a suitable language integration architecture is available).
Tier 3 – Future IDE Compatibility
11. Zed
12. Helix
13. Kate
14. Other LSP-compatible editors.
Important:
- Use LSP as the common language intelligence protocol.
- Use native IDE plugin APIs where necessary.
- Do not assume every IDE supports identical LSP functionality.
- For IntelliJ-based IDEs, implement a JetBrains plugin with LSP integration and native PSI/indexing integration where needed.
- Android Studio should reuse the JetBrains integration where compatible.
- For Eclipse, use the Language Server Protocol ecosystem such as LSP4E.
- For Visual Studio, investigate its supported language extension APIs rather than assuming VS Code extension compatibility.
3. Core Architecture
Design the following architecture:

Mantra Compiler
|
|-- Rust Compiler Core
|-- C++ Compiler Components
|-- Lexer
|-- Parser
|-- AST
|-- Semantic Analyzer
|-- Type Checker
|-- Symbol Table
|-- Module Resolver
|-- Compiler Diagnostics
|
v
Mantra Language Intelligence Engine
|
|-- Workspace Manager
|-- Document Manager
|-- Incremental Parser
|-- Semantic Indexer
|-- Symbol Resolver
|-- Type Resolver
|-- Reference Resolver
|-- Completion Engine
|-- Diagnostics Engine
|-- Navigation Engine
|-- Refactoring Engine
|-- Documentation Engine
|
v
Mantra Language Server (LSP)
|
|-- JSON-RPC Communication
|-- LSP Protocol
|-- Semantic Tokens
|-- Diagnostics
|-- Code Actions
|-- Workspace Symbols
|
v
IDE CLIENTS
|
|-- VS Code Extension
|-- IntelliJ Plugin
|-- Android Studio Plugin
|-- CLion Plugin
|-- RustRover Plugin
|-- Neovim Configuration
|-- Sublime Text Package
|-- Eclipse Plugin
|-- Emacs Configuration
|-- Visual Studio Extension
4. Rust and C++ Compiler Integration
The existing Mantra compiler is the source of truth.

Do not independently implement a second lexer, parser, type checker, or semantic analyzer for the IDE.

First inspect the existing compiler repository and identify:
- Lexer interfaces.
- Parser interfaces.
- AST representation.
- Source location representation.
- Semantic analysis interfaces.
- Symbol table implementation.
- Type resolution implementation.
- Module and package resolution.
- Compiler diagnostics.
- Existing Rust and C++ interoperability.
Create a stable compiler integration layer.

Recommended approach:
- Implement the LSP server primarily in Rust.
- Reuse existing Rust compiler libraries directly.
- Expose C++ functionality through a stable C ABI or an appropriate Rust FFI binding.
- Avoid unnecessary serialization between internal compiler components.
- Use structured interfaces between the compiler and language intelligence engine.
- Preserve source locations and symbol identities across Rust/C++ boundaries.
If existing compiler components cannot safely be reused directly, create a dedicated adapter layer.

Never duplicate compiler logic.
5. Mantra Language File Support
Register the following file types:
- .mantra – Mantra source files.
- mantra.toml – Mantra project configuration, if supported by the existing toolchain.
- Mantra.lock – Dependency lockfile, if implemented.
- Additional Mantra-specific configuration files as defined by the compiler.
Recognize Mantra packages, modules, imports, classes, interfaces, functions, methods, constructors, properties, variables, annotations, generics, inheritance, and polymorphism.

Respect Mantra's actual grammar and syntax.

Do not introduce syntax that does not exist in the language.
6. Advanced Syntax Highlighting
Implement two complementary syntax-highlighting systems.
A. TextMate Grammar
Create a TextMate grammar for immediate syntax highlighting.

File:

mantra.tmLanguage.json

Recognize:
- Language keywords.
- Access modifiers.
- Classes.
- Interfaces.
- Functions.
- Methods.
- Constructors.
- Data types.
- Primitive types.
- Variables.
- Constants.
- Properties.
- Strings.
- Numbers.
- Boolean literals.
- Comments.
- Annotations.
- Operators.
- Generic declarations.
- Import statements.
- Package declarations.
- Exception handling.
- Control flow.
- Multiline comments.
- Documentation comments.
B. Semantic Token Provider
Implement semantic token generation using the actual compiler AST and semantic analysis.

Support:
- Namespace.
- Class.
- Interface.
- Enum.
- Enum member.
- Struct, if supported.
- Method.
- Function.
- Constructor.
- Property.
- Field.
- Variable.
- Parameter.
- Type.
- Type parameter.
- Constant.
- Decorator or annotation.
- Module.
- Label.
- Comment.
Support semantic token modifiers:
- declaration
- definition
- readonly
- static
- abstract
- deprecated
- async
- modification
- documentation
- defaultLibrary
The token provider must distinguish between actual semantic entities and ordinary identifiers.

For example, imported class names, method calls, local variables, and types must receive appropriate semantic classifications.
7. Mantra Official Color Theme
Create an official Mantra Dark Theme and Mantra Light Theme.

Recommended dark theme palette:
- Keywords: Purple #C792EA
- Classes: Blue #82AAFF
- Interfaces: Cyan #89DDFF
- Methods: Gold #FFCB6B
- Functions: Yellow #FFCB6B
- Types: Cyan #89DDFF
- Variables: Light Red #F07178
- Constants: Orange #F78C6C
- Strings: Green #C3E88D
- Numbers: Orange #F78C6C
- Comments: Grey #697386
- Operators: Light Grey
- Annotations: Purple
- Documentation: Muted Green
Requirements:
- Support dark and light editor themes.
- Respect semantic token scopes.
- Allow users to customize colors.
- Avoid hardcoding colors into the language server.
- Provide consistent highlighting across supported IDEs.
8. Clickable Class, Method and Symbol Navigation
This is a critical feature.

Implement true compiler-backed symbol navigation.

Developers must be able to:
1. Ctrl + Click a class name to open its declaration.
2. Ctrl + Click an interface to open its declaration.
3. Ctrl + Click a method to open its implementation.
4. Ctrl + Click a variable to navigate to its declaration.
5. Ctrl + Click an imported package to navigate to its source.
6. Navigate between overridden methods and their implementations.
7. Navigate to constructors.
8. Navigate to inherited members.
9. Navigate between interfaces and implementing classes.
10. Navigate across different .mantra files and packages.
Implement:
- textDocument/definition
- textDocument/declaration
- textDocument/implementation
- textDocument/typeDefinition
- textDocument/references
- textDocument/documentSymbol
- workspace/symbol
Support overloaded methods and distinguish symbols by their resolved signatures.

Use actual source ranges from the compiler AST.

Do not use string matching as the primary navigation mechanism.
9. IntelliSense and Autocomplete
Implement intelligent context-aware autocomplete.

Support suggestions for:
- Classes.
- Interfaces.
- Methods.
- Functions.
- Variables.
- Properties.
- Fields.
- Constructors.
- Packages.
- Imports.
- Types.
- Generic parameters.
- Built-in libraries.
- Standard library APIs.
- Project dependencies.
Examples:

When a developer types:

User.

Suggest all accessible members of the resolved User type.

When a developer types:

new Us

Suggest matching classes if the Mantra grammar supports that construction syntax.

When a developer types:

import com.mantra.

Suggest available packages and modules.

When a developer types:

public f

Suggest valid declarations based on the current syntax and semantic context.

Implement:
- Completion ranking.
- Context-aware filtering.
- Auto-import suggestions.
- Method overload suggestions.
- Parameter information.
- Snippet completion.
- Standard library completion.
- Project dependency completion.
Implement textDocument/completion.
10. Hover Documentation and Signature Help
When developers hover over a symbol, display:
- Symbol name.
- Symbol type.
- Class or interface information.
- Method signature.
- Parameter names and types.
- Return type.
- Access modifier.
- Documentation comments.
- Inheritance information.
- Deprecation information.
- Source declaration location.
Example:

getUserName(): String

Hover:

UserService.getUserName()

Returns: String

Access: public

Description: Returns the current user's name.

Implement:
- textDocument/hover
- textDocument/signatureHelp
11. Advanced Refactoring
Implement:
- Rename symbol.
- Rename class.
- Rename method.
- Rename variable.
- Rename property.
- Update references across files.
- Organize imports.
- Remove unused imports.
- Extract method.
- Extract variable.
- Generate constructor.
- Generate getters and setters where supported.
- Implement interface methods.
- Override superclass methods.
- Convert supported code patterns safely.
Use compiler-resolved symbols to ensure refactoring does not modify unrelated identifiers.

Implement:
- textDocument/rename
- textDocument/codeAction
- textDocument/prepareRename
- Workspace edits.
12. Diagnostics and Error Reporting
Provide real-time compiler-backed diagnostics.

Detect:
- Syntax errors.
- Undefined classes.
- Undefined methods.
- Undefined variables.
- Invalid imports.
- Type mismatches.
- Incorrect method signatures.
- Invalid inheritance.
- Interface implementation errors.
- Access modifier violations.
- Duplicate declarations.
- Invalid generic types.
- Missing return statements.
- Invalid constructor calls.
- Unreachable code where supported.
- Unused variables and imports where supported.
Display errors and warnings with:
- Red underlines for errors.
- Yellow underlines for warnings.
- Informational hints.
- Detailed hover messages.
- Exact source locations.
- Suggested quick fixes.
Implement textDocument/publishDiagnostics.

Do not require a complete project compilation for every keystroke.

Use incremental analysis and debounce expensive operations.
13. Workspace and Project Intelligence
The language server must understand complete Mantra projects.

Implement:
- Workspace discovery.
- Project root detection.
- Package indexing.
- Module indexing.
- Dependency indexing.
- Import resolution.
- Cross-file symbol references.
- Standard library indexing.
- External library indexing.
- Build configuration discovery.
- Multi-project workspace support.
- Workspace cache.
- Incremental file indexing.
Reuse Mantra's official project configuration and dependency management.

Do not introduce an incompatible project configuration format.
14. Performance Requirements
The language server must be responsive even for large projects.

Requirements:
- Incremental parsing.
- Incremental semantic analysis.
- Background workspace indexing.
- Cancellation support.
- Efficient symbol lookup.
- Cached type resolution.
- Cached module resolution.
- Memory-efficient document storage.
- Asynchronous operations where appropriate.
- Proper concurrency management.
- File change synchronization.
- Graceful recovery from malformed source code.
Never block the editor while performing full compilation.

Target:
- Fast local syntax feedback.
- Near-instant cached symbol lookup.
- Responsive completion.
- Efficient cross-file navigation.
- Incremental workspace updates.
Establish performance benchmarks instead of assuming these targets are achieved.
15. Debugger Integration
Prepare a common debugger integration architecture.

Investigate and implement Debug Adapter Protocol (DAP) support where the Mantra VM or native runtime provides the necessary debugging capabilities.

Support:
- Breakpoints.
- Conditional breakpoints.
- Step over.
- Step into.
- Step out.
- Continue.
- Pause.
- Variable inspection.
- Call stack.
- Watch expressions.
- Exception breakpoints.
- Source mapping.
- Runtime evaluation where supported.
Create:

mantra-dap

Integrate with the existing Mantra VM and native execution runtime.

Do not simulate debugging features that the runtime cannot actually support.
16. IDE-Specific Implementations
VS Code
Create:

mantra-vscode

Include:
- Language registration.
- TextMate grammar.
- Semantic highlighting.
- Language server client.
- IntelliSense.
- Navigation.
- Diagnostics.
- Refactoring.
- Official Mantra themes.
- Debugger integration.
- Project explorer integration.
- Mantra commands.
- Mantra build and run tasks.
IntelliJ IDEA
Create:

mantra-intellij-plugin

Include:
- .mantra file recognition.
- Syntax highlighting.
- LSP client integration.
- Native PSI integration where needed.
- Code completion.
- Go to declaration.
- Find usages.
- Documentation popup.
- Error annotations.
- Refactoring.
- Project indexing.
- Run configurations.
- Debugger integration when available.
Android Studio
Create a compatible Mantra plugin.

Reuse JetBrains integration components wherever technically compatible.

Include:
- Mantra file recognition.
- Syntax highlighting.
- LSP integration.
- Project navigation.
- Code completion.
- Diagnostics.
- Mantra project support.
CLion
Create Mantra support using JetBrains APIs.

Support:
- Language recognition.
- Syntax highlighting.
- LSP integration.
- Semantic navigation.
- Completion.
- Diagnostics.
- Build and run integration.
RustRover
Create Mantra support through the same JetBrains integration architecture.

Avoid conflicts with Rust language support.
Neovim
Provide:
- LSP configuration.
- Filetype registration.
- Syntax highlighting.
- Semantic tokens.
- Completion integration.
- Diagnostics.
- Formatting.
- Navigation.
- Debugger integration through supported DAP clients.
Sublime Text
Create a Mantra package with:
- Syntax definitions.
- Language Server Protocol integration.
- Autocomplete.
- Navigation.
- Diagnostics.
- Theme support.
Eclipse
Create a Mantra Eclipse plugin using the appropriate Eclipse language tooling APIs.

Provide:
- Syntax highlighting.
- LSP integration.
- Content assist.
- Diagnostics.
- Navigation.
- Project support.
Emacs
Provide an Emacs package using Eglot or another appropriate LSP client.

Support:
- Mantra mode.
- Syntax highlighting.
- Semantic tokens where supported.
- Completion.
- Diagnostics.
- Navigation.
- Formatting.
Visual Studio
Investigate a native extension using the appropriate Visual Studio SDK and language service APIs.

Reuse the Mantra language intelligence engine through supported integration mechanisms.
17. Formatting and Code Quality
Create a dedicated formatter:

mantra fmt

Use the existing Mantra grammar and language rules.

Support:
- Consistent indentation.
- Whitespace normalization.
- Brace formatting.
- Import organization.
- Line wrapping.
- Documentation formatting.
- Configurable formatting preferences.
Respect Mantra's official four-space indentation rules.

Provide IDE integration through:

textDocument/formatting

textDocument/rangeFormatting

textDocument/onTypeFormatting

Do not introduce formatting behavior that changes program semantics.
18. Standard Library Intelligence
Index Mantra's standard libraries.

Provide autocomplete, navigation, documentation, and semantic information for:
- Collections.
- Strings.
- Numbers.
- Date and time.
- File systems.
- Networking.
- Concurrency.
- Async operations.
- JSON.
- HTTP.
- Database APIs.
- UI and frontend APIs where available.
When source code is unavailable, provide generated symbol metadata and documentation from the official library definitions.
19. Suggested Repository Structure
mantra/
compiler/
    rust/
    cpp/
    lexer/
    parser/
    ast/
    semantic/
    symbol_table/

tooling/

    mantra-lsp/
        src/
            server/
            protocol/
            handlers/
            workspace/
            documents/
            symbols/
            resolver/
            completion/
            diagnostics/
            navigation/
            refactoring/
            semantic_tokens/
            documentation/
            formatting/

    mantra-dap/

    ide/
        vscode/
        intellij/
        android-studio/
        clion/
        rustrover/
        neovim/
        sublime/
        eclipse/
        emacs/
        visual-studio/

    grammars/
        mantra.tmLanguage.json

    themes/
        mantra-dark/
        mantra-light/

tests/
    lexer/
    parser/
    semantic/
    lsp/
    navigation/
    completion/
    diagnostics/
    refactoring/
    formatting/
    integration/

docs/
    ide-setup/
    lsp/
    extensions/
    architecture/
Adapt this structure to the existing Mantra repository rather than duplicating existing compiler directories.
20. Distribution and Installation
Distribute the language server through the existing Mantra toolchain.

Provide:

mantra lsp

mantra doctor

mantra ide install vscode

mantra ide install intellij

mantra ide install neovim

Add installation support for other IDEs as appropriate.

Package the Mantra language server for:
- Windows x64.
- Windows ARM64 where supported.
- macOS Intel.
- macOS Apple Silicon.
- Linux x64.
- Linux ARM64 where supported.
Publish IDE-specific integrations through their respective official marketplaces or distribution channels.

Keep compiler and language server version compatibility explicit.

The IDE extension should be able to locate the installed Mantra toolchain or download a compatible version with user consent.
21. Testing and Quality Assurance
Create automated tests for every supported capability.

Test scenarios:
1. Syntax highlighting for all Mantra language constructs.
2. Semantic highlighting for actual compiler-resolved symbols.
3. Go to definition across files.
4. Go to definition across packages.
5. Go to implementation.
6. Find references.
7. Rename symbols safely.
8. Autocomplete imported classes.
9. Autocomplete object members.
10. Method overload resolution.
11. Hover documentation.
12. Type diagnostics.
13. Incremental document changes.
14. Workspace indexing.
15. Formatter correctness.
16. Malformed source recovery.
17. Large-project performance.
18. Windows, macOS and Linux compatibility.
Create a shared IDE conformance test suite wherever possible.
22. Development Phases
Implement in the following sequence.
Phase 1 – Compiler Integration
Inspect and integrate the existing Rust and C++ compiler components.

Establish source location mapping, symbol identity, AST access, and semantic analysis APIs.
Phase 2 – Mantra LSP Foundation
Build the Rust-based LSP server.

Implement document synchronization, workspace management, diagnostics, and semantic token support.
Phase 3 – Syntax Highlighting
Develop TextMate grammar and official Mantra Dark and Light themes.
Phase 4 – Symbol Navigation
Implement class, method, variable, interface, constructor, and cross-package navigation.
Phase 5 – IntelliSense
Implement completion, hover, signature help, imports, and type-aware suggestions.
Phase 6 – Refactoring and Diagnostics
Implement rename, references, code actions, quick fixes, and advanced diagnostics.
Phase 7 – VS Code Release
Publish the first complete Mantra IDE integration.
Phase 8 – JetBrains Ecosystem
Release IntelliJ IDEA, Android Studio, CLion, and RustRover integrations.
Phase 9 – Other Editors
Implement Neovim, Sublime Text, Eclipse, Emacs, and Visual Studio integrations.
Phase 10 – Debugger and Distribution
Integrate DAP, release installers, publish marketplace packages, and establish automated release pipelines.
23. Non-Negotiable Engineering Rules
1. Mantra compiler remains the single source of truth.
2. Do not build a separate parser for every IDE.
3. Do not implement fake IntelliSense using regex.
4. Do not use text matching for semantic navigation.
5. Use actual AST and semantic analyzer information.
6. Preserve exact source locations.
7. Implement incremental analysis.
8. Maintain backward compatibility with Mantra language versions.
9. Use official LSP and DAP protocols.
10. Use native IDE APIs where LSP alone is insufficient.
11. Avoid introducing unnecessary dependencies.
12. Follow Rust and C++ memory safety and interoperability best practices.
13. Provide automated tests for all major features.
14. Ensure cross-platform support.
15. Do not break existing compiler functionality.
16. Do not invent unsupported language constructs.
17. Never claim a feature is implemented until it passes its integration tests.
24. Final Deliverables
The completed project must provide:
1. mantra-lsp – Universal Mantra Language Server.
2. mantra-dap – Debug Adapter Protocol implementation, subject to runtime capabilities.
3. Mantra TextMate Grammar.
4. Mantra Dark Theme.
5. Mantra Light Theme.
6. Mantra VS Code Extension.
7. Mantra IntelliJ IDEA Plugin.
8. Mantra Android Studio Plugin.
9. Mantra CLion Plugin.
10. Mantra RustRover Plugin.
11. Mantra Neovim Integration.
12. Mantra Sublime Text Package.
13. Mantra Eclipse Plugin.
14. Mantra Emacs Integration.
15. Mantra Visual Studio Integration, subject to supported APIs.
16. Installation and configuration commands.
17. Cross-platform distribution packages.
18. Developer documentation.
19. Integration and compatibility test suites.
25. Expected Final Developer Experience
A developer should be able to open any .mantra file in a supported IDE and immediately receive a professional Mantra development experience.

They should be able to:
- Read colorful, semantic Mantra code.
- Click classes and navigate to declarations.
- Click methods and navigate to implementations.
- Find every reference to a symbol.
- Rename classes and methods across the project.
- Receive intelligent autocomplete.
- View method signatures and documentation.
- Detect compiler errors while typing.
- Navigate imported packages and libraries.
- Format source code automatically.
- Build and run Mantra applications.
- Debug supported Mantra programs.
The experience should feel comparable to professional Java development in IntelliJ IDEA and C# development in Visual Studio.

Start by auditing the existing Mantra compiler repository. Produce the architecture and integration plan first. Then implement each phase incrementally, compiling and testing after every phase. Do not rewrite the existing compiler or attempt to implement every IDE simultaneously.

The final objective is to establish a unified, production-grade Mantra Developer Tools Ecosystem powered by the existing Mantra compiler.




My recommended release strategy
Don't release all IDE integrations simultaneously. Build the core once, then expand.
Release	Deliverable
v0.1	Mantra TextMate grammar + themes
v0.2	Rust-based Mantra LSP
v0.3	Go to Definition + Find References
v0.4	IntelliSense + Semantic Highlighting
v0.5	VS Code Marketplace release
v0.6	IntelliJ + Android Studio
v0.7	CLion + RustRover
v0.8	Neovim + Sublime + Eclipse + Emacs
v0.9	DAP debugging + advanced refactoring
v1.0	Complete Mantra IDE ecosystem

One architectural decision I strongly recommend: Keep mantra-lsp independent of the compiler executable. It should be able to reuse compiler libraries directly and operate as a persistent process. This will make IDE performance significantly better than launching a complete compiler process for every editor request.