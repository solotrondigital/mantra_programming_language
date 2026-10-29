Mantra should support complete Object-Oriented Programming (OOP) as a core part of the language specification, not merely provide classes as a future package. I'll incorporate OOP into the existing Mantra v0.1 blueprint while preserving the Python-like readability, static typing, Rust-inspired safety, self-hosting compiler, MDK, package ecosystem, and cross-platform goals.

The redesigned specification will define classes and objects, encapsulation, abstraction, inheritance, polymorphism, interfaces, constructors, method overriding, access control, static members, generics, and composition. It will also explain what mantrac must implement to make each feature work, rather than simply showing example syntax.
Mantra Language Specification v0.1 — Redesigned with Complete OOP

Mantra Programming Language
Language specification, OOP model, compiler architecture, runtime, MDK, and cross-platform ecosystem



Compiler

mantrac

Developer kit

MDK

Implementation

Rust bootstrap

Native backend

Cranelift candidate

Primary platforms

Windows, macOS, Linux

Additional targets

WebAssembly, iOS, Android


Proposed specification — not yet implemented

The goal is to make Mantra approachable like Python, statically typed, compiled to native code, and capable of supporting serious object-oriented applications without requiring Python to be installed.

The following document replaces the previous blueprint and incorporates OOP into the language, compiler, runtime, testing, and implementation plan.
Mantra Language Specification v0.1
Complete Language, OOP, Compiler, Runtime, and MDK Blueprint
Status: Proposed specification
Version: 0.1-draft
Language: Mantra
Compiler: mantrac
Development Kit: Mantra Development Kit (MDK)
Bootstrap implementation: Rust
Native code-generation backend: Cranelift, subject to evaluation
Primary platforms: Windows, macOS, Linux
Additional platforms: WebAssembly, Android, iOS

Part I — Mantra Language Vision
1. Purpose
Mantra is a proposed general-purpose programming language designed for developers who want readable syntax, static type checking, native compilation, and a practical developer ecosystem.

Mantra shall combine:
- Python-like readability and indentation.
- Static typing with type inference.
- Object-oriented programming.
- Functional programming capabilities where useful.
- Rust-inspired memory safety concepts.
- Native executable generation.
- Efficient asynchronous networking.
- API-first standard libraries.
- A self-hosting compiler.
- Cross-platform developer tools.
- A unified package system for backend, web, and mobile development.
Mantra must be implemented as an independent language. Python will not be a runtime dependency.
2. Product architecture
Mantra consists of the following products:
Product	Responsibility
Mantra Language	Syntax, types, functions, OOP, modules, and language semantics
mantrac	Compiler, type checker, optimizer, and code generator
Mantra Runtime	Memory support, errors, platform interfaces, and optional execution services
MDK	CLI, build system, formatter, testing, debugging, and documentation tools
Mantra Core	Foundational language and standard-library facilities
Mantra Packages	Official, community, and private libraries
Mantra Web	Backend APIs and browser applications
Mantra Mobile	Hybrid mobile development for iOS and Android
Mantra Registry	Package discovery, publishing, versioning, and integrity verificationAll features described below are design goals until they are implemented and pass their acceptance tests.
Part II — Core Language
3. Source files and indentation
Mantra source files use the .mantra extension.

Example:
module hello

def main() -> Int:
    print("Hello from Mantra")
    return 0
Mantra uses indentation to define blocks.

Proposed rules:
1. Four spaces are the recommended indentation width.
2. Tabs in indentation are rejected.
3. The lexer emits NEWLINE, INDENT, and DEDENT tokens.
4. Statements and declarations follow a documented grammar.
5. Compiler errors include source file, line, column, and an error code.
4. Variables and types
Mantra supports explicit types and type inference.
name: String = "Mantra"
version: Int = 1
price: Float = 19.99
enabled: Bool = True

let language = "Mantra"
mut request_count: Int = 0

request_count = request_count + 1
let defines an immutable binding. mut defines a mutable binding.

The compiler rejects assignments that violate the declared type or mutability rules.
5. Built-in types
Type	Purpose
Int	Signed 64-bit integer in v0.1
Float	IEEE 754 binary64
Bool	Boolean value
String	UTF-8 text
Byte	Unsigned 8-bit integer
List<T>	Ordered collection
Map<K, V>	Key-value collection
Set<T>	Collection of unique values
Option<T>	Optional value
Result<T, E>	Success or error
Tuple<T...>	Fixed group of values
None	Absence of a value
Unit	No meaningful return value, if distinguished from NoneThe language specification must define integer overflow, string indexing, collection mutability, equality, hashing, and conversion rules.
6. Control flow
age = 25

if age >= 18:
    print("Adult")
else:
    print("Minor")

for number in range(5):
    print(number)

while age > 0:
    age = age - 1
The language shall define conditional statements, loops, break, continue, pattern matching, and return behavior.
7. Functions
def add(a: Int, b: Int) -> Int:
    return a + b

result = add(10, 20)
The function system shall support:
- Parameters and return types.
- Default arguments.
- Named arguments.
- Type inference.
- Generic functions.
- Visibility modifiers.
- Compile-time validation.
- Documented error behavior.
Async functions and advanced generic constraints may be introduced in later implementation stages.
Part III — Complete Object-Oriented Programming
8. OOP design goals
Mantra shall support object-oriented programming as a first-class language feature.

The language specification must address:
1. Classes and objects.
2. Attributes and methods.
3. Encapsulation.
4. Abstraction.
5. Inheritance.
6. Polymorphism.
7. Interfaces and contracts.
8. Constructors and initialization.
9. Method overriding and overloading.
10. Static members and class-level functions.
11. Abstract classes and methods.
12. Access control.
13. Properties and controlled attribute access.
14. Composition and aggregation.
15. Generics and bounded type parameters.
16. Runtime type information and safe type tests.
17. Equality, hashing, and string representations.
18. Object lifecycle and resource cleanup.
19. Operator overloading with defined rules.
20. Dynamic dispatch and virtual method tables where required.
These are language requirements, not merely standard-library packages.
9. Classes and objects
A class defines a type containing data and behavior. An object is an instance of that type.

Proposed syntax:
class User:
    def __init__(self, name: String, age: Int):
        self.name = name
        self.age = age

    def greet(self) -> String:
        return "Hello, " + self.name

user = User("Alex", 25)
print(user.greet())
The example illustrates Python-like conventions. The actual compiler must define constructor semantics, attribute declarations, and object layout.
Requirements
- A class declaration introduces a nominal type.
- Objects have a well-defined identity and lifetime.
- Instance methods receive an implicit self parameter.
- Constructors initialize objects before they are exposed to ordinary code.
- The compiler checks attribute access and method calls.
- Fields and methods have explicit visibility rules.
- Recursive and mutually dependent type declarations must be handled according to defined rules.
10. Encapsulation
Encapsulation allows a class to control how its internal state is accessed and modified.

Proposed syntax:
class BankAccount:
    def __init__(self, initial_balance: Float):
        self._balance = initial_balance

    pub def deposit(self, amount: Float) -> Result<Unit, String>:
        if amount <= 0:
            return Err("Amount must be positive")

        self._balance = self._balance + amount
        return Ok(None)

    pub def balance(self) -> Float:
        return self._balance
This example demonstrates the intended API, but the underscore naming convention alone must not be treated as access control.
Language requirements
Mantra shall support compiler-enforced visibility, with a proposed model:
- pub: accessible from permitted external modules.
- private: accessible only within the declaring class or explicitly defined scope.
- protected: accessible within the class and permitted derived classes, if this modifier is retained in the final design.
- Default visibility: private or module-private, as explicitly chosen by the specification.
The final specification must distinguish access control from mutability and ownership.
11. Abstraction
Abstraction exposes essential behavior while hiding implementation details.

Mantra shall support interfaces or abstract classes for defining contracts.

Proposed interface:
interface PaymentProcessor:
    def charge(amount: Float) -> Result<String, PaymentError>
An implementation:
class CardProcessor implements PaymentProcessor:
    def charge(amount: Float) -> Result<String, PaymentError>:
        # Payment integration would be implemented here.
        return Ok("transaction-123")
The compiler must verify that an implementing class provides every required method with compatible parameter and return types.

An interface declaration does not itself perform payment processing. Concrete implementations belong to packages.
12. Inheritance
Mantra shall support single inheritance for classes in the initial OOP design.

Proposed syntax:
class Animal:
    def __init__(self, name: String):
        self.name = name

    pub def speak(self) -> String:
        return "Unknown sound"


class Dog extends Animal:
    def __init__(self, name: String):
        super().__init__(name)

    override pub def speak(self) -> String:
        return "Woof"
Inheritance rules
- A class may extend at most one concrete base class in v0.1.
- Multiple interface implementations may be supported.
- Constructors are not inherited as ordinary methods.
- Derived classes must initialize the base portion of the object correctly.
- Overriding must be explicit using override.
- Overriding a final method or extending a final class is a compile-time error.
- The compiler checks visibility and method-signature compatibility.
- The language must define initialization order and base-constructor requirements.
Single inheritance reduces ambiguity and implementation complexity. Multiple inheritance can be reconsidered after the initial object model is stable.
13. Polymorphism
Polymorphism allows code to operate on different concrete implementations through a common interface or base type.

Proposed example:
interface Shape:
    def area() -> Float


class Circle implements Shape:
    def __init__(self, radius: Float):
        self.radius = radius

    override def area() -> Float:
        return 3.141592653589793 * self.radius * self.radius


class Rectangle implements Shape:
    def __init__(self, width: Float, height: Float):
        self.width = width
        self.height = height

    override def area() -> Float:
        return self.width * self.height


def print_area(shape: Shape) -> None:
    print(shape.area())
Requirements
Mantra shall support subtype polymorphism through interfaces or base classes.

The compiler must:
- Check subtype relationships.
- Validate method overrides.
- Resolve method calls according to static and dynamic dispatch rules.
- Prevent invalid casts.
- Define how interface values are represented.
- Define whether and when dynamic type information is available.
Dynamic dispatch may use virtual method tables or another backend-specific implementation. The source-language semantics must not depend on the choice of backend.
14. Interfaces and contracts
Interfaces define required behavior independently of a concrete implementation.

Proposed syntax:
interface Serializable:
    def to_json() -> String


class User implements Serializable:
    def __init__(self, id: Int, name: String):
        self.id = id
        self.name = name

    override def to_json() -> String:
        return "{\"id\":" + self.id.to_string() + \
               ",\"name\":\"" + self.name + "\"}"
The example is illustrative; a production JSON package should perform correct escaping and serialization rather than manually concatenating strings.

Interfaces should support:
- Method signatures.
- Return types.
- Multiple interface implementation.
- Generic interfaces.
- Interface-based polymorphism.
- Explicit implementation checking.
Default interface methods and associated types can be deferred until their semantics are fully specified.
15. Constructors and object initialization
The initial syntax may use __init__ to feel familiar to Python developers.

Requirements:
- Constructors initialize all required fields.
- Objects cannot be used before successful initialization.
- Base-class initialization follows explicit rules.
- Invalid constructor arguments are rejected.
- Constructor failure does not leak resources.
- Constructor visibility is checked by the compiler.
- Static constructors, if ever supported, must have documented ordering and failure behavior.
A future dedicated init syntax may be considered, but the first version should use one constructor convention consistently.
16. Method overriding and overloading
Overriding replaces an inherited implementation while preserving a compatible method contract.
class Base:
    pub def message(self) -> String:
        return "Base"


class Child extends Base:
    override pub def message(self) -> String:
        return "Child"
The compiler shall reject invalid overrides, including incompatible return types and missing required parameters.

Overloading allows multiple functions with the same name but distinct signatures.
def format(value: Int) -> String:
    return value.to_string()

def format(value: Float) -> String:
    return value.to_string()
The initial implementation should use compile-time overload resolution based on parameter types. The compiler must reject ambiguous overloads.

Overloading is distinct from overriding and must be documented separately.
17. Static members and class-level functions
Mantra should support class-level functions and constants.

Proposed syntax:
class MathUtils:
    static PI: Float = 3.141592653589793

    static def square(value: Int) -> Int:
        return value * value


result = MathUtils.square(5)
The specification must define:
- Whether static members are inherited.
- Whether static fields are mutable.
- How static initialization works.
- Whether static functions have access to instance state.
- How static storage is initialized and cleaned up.
Avoid dynamic class-level initialization in the initial release if it introduces unnecessary runtime complexity.
18. Abstract classes and methods
An abstract class cannot be instantiated directly and may contain abstract methods that subclasses must implement.

Proposed syntax:
abstract class Repository:
    abstract def get_by_id(id: Int) -> Option<String>

    pub def exists(id: Int) -> Bool:
        return self.get_by_id(id).is_some()
A concrete subclass must implement all required abstract methods.

The compiler must reject:
- Instantiation of abstract classes.
- Concrete classes with missing implementations.
- Invalid abstract-method overrides.
- Attempts to override final methods.
Abstract classes and interfaces serve related but distinct purposes: an abstract class can contain shared implementation and state, while an interface primarily defines a behavioral contract.
19. Access control
The proposed access-control model is:
Modifier	Intended scope
pub	Publicly accessible API
private	Restricted to the declaring class or scope
protected	Declaring class and permitted subclasses, if retained
No modifier	Private or module-private according to the final ruleThe final specification must choose exact scope boundaries. It must not leave access behavior to backend implementation.

The compiler shall reject illegal access at compile time wherever visibility is statically known.
20. Properties and controlled access
Properties expose field-like access while allowing validation or computed values.

Proposed syntax:
class Temperature:
    def __init__(self, celsius: Float):
        self._celsius = celsius

    @property
    def celsius(self) -> Float:
        return self._celsius

    @celsius.setter
    def celsius(self, value: Float) -> None:
        if value < -273.15:
            raise ValueError("Below absolute zero")

        self._celsius = value
This example adopts Python-like decorators as a possible syntax. Decorators and property lowering must be implemented explicitly; they cannot be assumed to exist because they appear in an example.

Mantra may instead choose dedicated property declarations. The compiler must settle on one form before promising compatibility.
21. Composition and aggregation
Composition means that an object contains or coordinates other objects rather than inheriting all their behavior.

Proposed example:
class Engine:
    def start(self) -> None:
        print("Engine started")


class Car:
    def __init__(self, engine: Engine):
        self.engine = engine

    pub def start(self) -> None:
        self.engine.start()
Composition should be the recommended default when an object needs another object's behavior.

The language should define ownership of contained objects separately from class relationships. A field holding an Engine does not automatically mean the Car exclusively owns it.
22. Generics and bounded type parameters
Generics allow classes, interfaces, and functions to work with multiple types while preserving compile-time checks.

Proposed syntax:
class Box<T>:
    def __init__(self, value: T):
        self.value = value

    pub def get(self) -> T:
        return self.value
A generic interface:
interface Repository<T>:
    def find(id: Int) -> Option<T>
The compiler must define generic type checking, substitution, bounds, and code generation.

The initial implementation may support basic generics before implementing advanced constraints, associated types, or specialization.
23. Equality, hashing, and string representations
The object model must define:
- Identity comparison.
- Value equality.
- Hashing behavior.
- String conversion.
- Debug representations.
- Equality consistency with hashing.
- Behavior of mutable objects used as map keys.
Default equality must be documented and predictable. The language should not silently use value equality for every object if object identity is intended.

Operator overloading may be supported for selected operators, with rules preventing arbitrary changes to fundamental evaluation semantics.
24. Runtime type information and casts
Mantra shall support safe type tests and checked downcasts if runtime polymorphism requires them.

The compiler and runtime must define:
- Whether type metadata is emitted.
- How interface and base-class values retain dynamic type information.
- Whether a cast can fail.
- How failed casts are represented.
- Whether reflection is supported.
Full reflection and dynamic class modification are not required for v0.1. They should be considered only after runtime-size and performance measurements.
25. Object lifecycle and resource management
The object model must define:
- Allocation.
- Initialization.
- Field destruction.
- Cleanup order.
- Error behavior during initialization.
- Ownership transfer.
- Resource cleanup during early returns or failures.
- Interaction with asynchronous tasks.
Mantra should favor deterministic resource cleanup for files, sockets, and database connections.

A garbage collector is not mandatory. The ownership model must be decided independently of the OOP syntax.
Part IV — Compiler Implementation
26. Compiler pipeline
The initial mantrac compiler will be written in Rust.
Mantra Source
     |
     v
Lexer
     |
     v
Parser
     |
     v
AST
     |
     v
Name and Scope Resolution
     |
     v
Type Checker
     |
     v
OOP Semantic Analysis
     |
     v
Typed Intermediate Representation
     |
     v
Optimization
     |
     v
Cranelift IR
     |
     v
Object File
     |
     v
Platform Linker
     |
     v
Native Executable
The frontend must remain independent of Cranelift.
27. OOP compiler components
The compiler must implement the following subsystems.
27.1 Class and type registry
Track:
- Class declarations.
- Base classes.
- Interfaces.
- Fields.
- Methods.
- Generic parameters.
- Visibility.
- Abstract and final declarations.
27.2 Scope and member resolution
Resolve:
- Local variables.
- Instance fields.
- Methods.
- Static members.
- Inherited members.
- Interface members.
- Imported types.
27.3 Type checker
Validate:
- Class assignments.
- Subtyping.
- Interface implementation.
- Generic instantiation.
- Method overrides.
- Constructor calls.
- Visibility rules.
- Property types.
- Operator overloads.
- Dynamic dispatch compatibility.
27.4 Object layout
Define how fields are laid out in memory and how inheritance affects layout.

Object layout must not be treated as stable ABI until the representation and compatibility policy are explicitly defined.
27.5 Method dispatch
Implement:
- Direct calls for statically resolved methods.
- Dynamic dispatch for polymorphic calls.
- Interface dispatch where needed.
- Correct virtual-method behavior.
- Dispatch-table generation and verification.
27.6 Lowering
Lower classes, constructors, interfaces, and methods into the typed IR.

The IR must preserve language semantics without requiring the backend to understand the entire OOP source language.
28. Cranelift backend evaluation
Cranelift is the initial candidate for native code generation.

Evaluate it for:
- Supported CPU architectures.
- Operating-system ABI compatibility.
- Object-file generation.
- Relocations and linking.
- Debug information.
- Optimization behavior.
- Function calls and dynamic dispatch.
- Executable size.
- Compilation speed.
- Runtime performance.
Do not assume that CPU support automatically guarantees complete operating-system or ABI support.

If Cranelift cannot satisfy a required target or optimization goal, evaluate LLVM or another backend without redesigning the language frontend.
Part V — Mantra Runtime
29. Runtime layers
The runtime should be split into a small core and optional facilities.
Core runtime
- Program entry point.
- Memory and allocation interfaces.
- String and collection support where needed.
- Panic and fatal-error handling.
- Platform abstraction.
- Object allocation support where required.
- Runtime type metadata if required by the object model.
Optional runtime libraries
- Async executor.
- Networking.
- TLS integration.
- Timers.
- Database interfaces.
- Logging.
- Serialization.
- Mobile platform bridges.
A simple command-line application should not need to link the complete web or mobile stack.
30. Runtime object model
The runtime must provide a defined representation for:
- Class instances.
- Fields.
- Base-class relationships.
- Virtual method dispatch.
- Interface values.
- Generic type information where required.
- Object destruction.
- Checked type tests.
Use the simplest representation that satisfies the language semantics and performance requirements.

A possible implementation uses a type descriptor and virtual-method table for classes requiring dynamic dispatch. Classes that do not need runtime polymorphism should not necessarily pay the full cost of a dynamic object model.

The actual representation remains an implementation decision until benchmarks and ABI requirements are established.
Part VI — Self-Hosting Compiler
31. Bootstrap stages
Stage 0: Rust compiler
Implement the initial compiler in Rust.
Stage 1: Mantra-written compiler
Implement the compiler in Mantra and compile it using Stage 0.
Stage 2: Self-compiled compiler
Use Stage 1 to compile the compiler source again.
Stage 3: Verification
Run the compiler test suite, language conformance suite, and reproducibility checks.

Retain the Rust bootstrap compiler as a recovery path.
32. Self-hosting acceptance tests
Required tests:
- Stage 0 successfully builds Stage 1.
- Stage 1 successfully builds Stage 2.
- Compiler tests pass for all supported features.
- Invalid programs produce the expected errors.
- OOP type checking and dispatch tests pass.
- Repeated builds satisfy the documented reproducibility requirements.
- Supported platform targets compile and execute test programs.
- Regression tests detect previously fixed bugs.
Self-hosting is not considered complete merely because the compiler can compile a small subset of itself.
Part VII — MDK
33. MDK command interface
The Mantra Development Kit provides a unified command-line interface.
Command	Purpose
mantra --version	Print toolchain version
mantra new NAME	Create a project
mantra new package NAME	Create a package
mantra run	Build and run a development target
mantra build	Build a target
mantra build --release	Build an optimized release
mantra check	Parse and type-check
mantra test	Run tests
mantra fmt	Format source code
mantra lint	Run static diagnostics
mantra add PACKAGE	Add a dependency
mantra remove PACKAGE	Remove a dependency
mantra doc	Generate documentation
mantra package validate	Validate package structure
mantra publish	Publish a package
mantra target list	List supported targetsThese commands are proposed. Their exact flags, output, exit codes, and configuration behavior must be specified and tested.
34. Developer workflow
A typical developer should be able to run:
mantra new user_api
cd user_api
mantra run
mantra test
mantra doc
mantra build --release
The MDK should coordinate compiler invocation, dependency resolution, testing, documentation generation, and packaging.
Part VIII — Mantra Packages
35. Package categories
Mantra Core
- Collections.
- Strings.
- Filesystem.
- Environment configuration.
- Errors.
- Testing.
Mantra Official Packages
- HTTP.
- JSON.
- Database interfaces.
- Authentication integrations.
- Logging.
- OpenAPI.
- Async utilities.
Custom and Community Packages
- Payment providers.
- Cloud SDKs.
- Third-party API clients.
- Company-specific packages.
- Reusable frameworks.
36. Package manifest
Proposed mantra.toml:
[package]
name = "user_service"
version = "0.1.0"
edition = "2026"
description = "User service built with Mantra"
license = "MIT"

[dependencies]
mantra_http = "0.1"
mantra_json = "0.1"

[build]
entry = "src/main.mantra"
The package manager must validate versions, resolve dependencies, generate a lockfile, verify downloaded package contents, and prevent silent modification of published releases.

OOP types and interfaces should be documented as part of each package's public API.
Part IX — Mantra Web
37. Web architecture
Mantra Web should support:
- Native HTTP servers.
- Routing and middleware.
- Request validation.
- JSON serialization.
- Database integrations.
- Authentication.
- API documentation generation.
- Browser applications.
- Shared models and API client types.
Browser support should be evaluated through WebAssembly and JavaScript interoperability.

WebAssembly alone does not provide browser DOM access; Mantra Web must provide an explicit interoperability layer.
Part X — Mantra Mobile
38. Hybrid mobile architecture
Mantra Mobile will initially use a hybrid WebView model.
Mantra Application
    |
    +-- Business Logic
    |
    +-- Shared Models
    |
    +-- HTML / CSS / JavaScript UI
    |
    +-- Platform Bridge
           |
           +-- iOS Host
           |
           +-- Android Host
Mantra Mobile should provide:
- Navigation.
- Application state management.
- HTTP clients.
- Platform plugin interfaces.
- Lifecycle integration.
- Permission handling.
- Native build and packaging workflows.
iOS and Android still require platform-specific SDKs, permissions, signing, and release processes.

The first mobile milestone should validate a small Android application, followed by an iOS application using the same documented application interfaces.
Part XI — Cross-Platform Distribution
39. Supported platform plan
Platform	Initial distribution goal
Windows x86-64	Signed MSI or executable installer
macOS ARM64	Signed and notarized package or archive
macOS x86-64	Additional target if supported
Linux x86-64	Tarball, followed by DEB and RPM
Linux ARM64	Additional target after validation
Android	APK for testing; AAB distribution workflow
iOS	Signed native project and archive workflow
Browser	Web assets and/or WebAssembly modulesThese are planned targets, not existing compatibility claims.

The installer should include the compiler, CLI, standard-library files, and required runtime components. It should not require developers to install Python or Rust to use the released Mantra toolchain.
Part XII — Testing and OOP Conformance
40. OOP test suite
The compiler must have dedicated tests for every OOP feature.
Feature	Required tests
Classes	Declaration, construction, field access
Encapsulation	Allowed and rejected access
Inheritance	Base and derived types
Polymorphism	Dynamic dispatch and interface dispatch
Interfaces	Missing and valid implementations
Constructors	Initialization order and invalid construction
Overriding	Valid overrides and signature mismatches
Overloading	Resolution and ambiguous calls
Abstract classes	Illegal instantiation and missing implementations
Static members	Access and initialization
Properties	Getter/setter behavior and type checks
Generics	Type substitution and invalid type arguments
Composition	Object references and lifecycle behavior
Equality	Identity, value equality, and hash consistency
Type casts	Valid casts and failed checked casts
Cleanup	Resource release on return and failureEach feature needs positive tests, negative tests, runtime tests where applicable, and regression coverage.
41. Performance evaluation
Measure:
- Compiler startup time.
- Compile time.
- Peak compiler memory.
- Executable size.
- Application startup time.
- Idle memory consumption.
- API throughput and latency.
- Allocation rates.
- Virtual dispatch overhead.
- Memory usage for object-heavy applications.
Compare the results against relevant languages using equivalent workloads.

Do not claim that Mantra is faster or uses less memory than other languages until measurements support the claim.
Part XIII — Implementation Roadmap
42. Milestones
Milestone 1: Language specification
Deliver:
- Grammar.
- Type rules.
- OOP semantics.
- Memory model.
- Error model.
- Module system.
Milestone 2: Bootstrap compiler
Deliver:
- Rust workspace.
- Lexer and parser.
- AST.
- Type checker.
- Typed IR.
- Basic native code generation.
Milestone 3: OOP implementation
Deliver:
- Classes and objects.
- Fields and methods.
- Constructors.
- Encapsulation.
- Single inheritance.
- Interfaces.
- Polymorphism.
- Method overriding.
- Initial generic support.
- OOP conformance tests.
Milestone 4: Runtime
Deliver:
- Allocation interfaces.
- Object representation.
- Method dispatch.
- Strings and collections.
- Error support.
- Resource cleanup.
Milestone 5: MDK and package manager
Deliver:
- CLI.
- Project scaffolding.
- Package manifest and lockfile.
- Testing.
- Formatter.
- Documentation generation.
Milestone 6: API ecosystem
Deliver:
- HTTP and JSON.
- Database interfaces.
- Authentication integrations.
- OpenAPI generation.
- API testing.
Milestone 7: Cross-platform alpha
Deliver:
- Windows, macOS, and Linux builds.
- Installer packages.
- CI matrix.
- Installation tests.
Milestone 8: Self-hosting
Deliver:
- Mantra-written compiler.
- Stage 0/1/2 bootstrap process.
- Reproducibility checks.
- Compiler conformance suite.
Milestone 9: Web and mobile
Deliver:
- Browser target.
- Web framework.
- Android hybrid prototype.
- iOS hybrid prototype.
- Native platform bridge.
Part XIV — v0.1 Acceptance Criteria
The initial developer preview should be accepted only when:
1. The implemented language subset is documented.
2. The compiler parses and type-checks supported programs.
3. The compiler generates working native executables.
4. Implemented OOP features pass their conformance tests.
5. Invalid programs produce source-located diagnostics.
6. The runtime passes its documented tests.
7. MDK can create, build, and test a sample project.
8. The package manifest and dependency rules are documented.
9. Platform support claims are backed by automated tests.
10. The compiler and runtime have reproducible build procedures.
Self-hosting, a mature public registry, and complete mobile tooling are later milestones rather than prerequisites for the first preview.
Part XV — Status and Implementation Boundaries
Component	Status
Mantra language design	Proposed
Static type system	Proposed
OOP model	Proposed
Class and interface semantics	Require full specification and implementation
Rust bootstrap compiler	Planned
Cranelift backend	Candidate requiring evaluation
Mantra runtime	Design required; not implemented
MDK	Command design proposed
Package manager	Manifest and workflow proposed
Mantra Web	Architecture proposed
Mantra Mobile	Hybrid architecture proposed
Cross-platform installers	Planned
Self-hosted compiler	Future milestone
Performance results	Not establishedFinal Recommendation
Implement OOP in the language frontend and type system from the beginning rather than adding it later as a library.

Start with classes, objects, constructors, fields, methods, encapsulation, single inheritance, interfaces, and polymorphism. Then add abstract classes, properties, overloading, generics, and the remaining object-model features according to the acceptance tests.

The Rust bootstrap compiler should compile the initial language, including the OOP subset. The Mantra-written compiler can then be developed using those same features.

The long-term goal is a language that is readable like Python, supports a complete documented OOP model, compiles to native executables, and provides a unified developer experience through MDK on Windows, macOS, Linux, WebAssembly, and hybrid mobile platforms.




What changes technically with this redesign?
Three compiler components become especially important:
1. OOP type checker: understands inheritance, interface implementation, method overriding, visibility, and generic types.
2. Object model and runtime: defines object layout, initialization, dynamic dispatch, type metadata, and cleanup.
3. OOP conformance suite: ensures the language behaves consistently across compiler versions and target platforms.
The OOP examples in the specification are proposed syntax. Some features—particularly decorators, properties, generics, and abstract classes—still require final grammar and semantic decisions before they can be considered part of an implemented compiler.

Recommended first implementation: build mantrac in Rust with classes, constructors, fields, methods, interfaces, single inheritance, and compile-time type checking. Once these work reliably, expand the OOP system and proceed toward self-hosting.