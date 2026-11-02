use mantra_intelligence::audit::CodeAuditor;
use mantra_intelligence::codegen::CodeGenerator;
use mantra_intelligence::completion::AiCompletionEngine;
use mantra_intelligence::context::CompilerIntelligenceContext;
use mantra_intelligence::doc_gen::DocGenerator;
use mantra_intelligence::error_intelligence::ErrorIntelligenceEngine;
use mantra_intelligence::project_graph::ProjectGraph;
use mantra_intelligence::quick_fix::QuickFixEngine;
use mantra_intelligence::refactor::RefactoringEngine;
use mantra_intelligence::test_gen::TestGenerator;
use mantra_intelligence::MantraIntelligence;

#[test]
fn test_compiler_intelligence_context() {
    let source = r#"
class UserService:
    pub user_id: Int
    pub email: String

    def __init__(self, id: Int, email: String):
        self.user_id = id
        self.email = email

    pub def getUser(self) -> String:
        return self.email
"#;
    let ctx = CompilerIntelligenceContext::from_source("src/user.mantra", source, 10, 5);
    assert_eq!(ctx.file_path, "src/user.mantra");
    assert_eq!(ctx.scope.enclosing_class.as_deref(), Some("UserService"));
    assert_eq!(ctx.scope.enclosing_function.as_deref(), Some("getUser"));
    assert!(!ctx.symbols.is_empty());

    let header = ctx.format_prompt_header();
    assert!(header.contains("UserService"));
    assert!(header.contains("getUser"));
}

#[test]
fn test_error_intelligence_explanations() {
    // 1. Keyword collision
    let exp1 = ErrorIntelligenceEngine::explain(
        "E0021",
        "Expected field. Found 'from'",
        "    pub from: String",
        None,
    );
    assert!(exp1.title.contains("Reserved Keyword Collision"));
    assert!(exp1.suggested_remedy.contains("from_email"));

    // 2. Type mismatch
    let exp2 = ErrorIntelligenceEngine::explain(
        "E0012",
        "Type mismatch: expected Int, found String",
        "let count: Int = \"10\"",
        None,
    );
    assert!(exp2.title.contains("Type Mismatch"));
    assert!(exp2.suggested_remedy.contains("Int"));

    // 3. Undefined symbol
    let exp3 = ErrorIntelligenceEngine::explain(
        "E0010",
        "Undefined symbol 'PropertiesTOML'",
        "let p = PropertiesTOML()",
        None,
    );
    assert!(exp3.title.contains("PropertiesTOML"));
    assert!(exp3.suggested_remedy.contains("mantra_util"));
}

#[test]
fn test_quick_fix_generation_and_apply() {
    let source = "class EmailMessage:\n    pub from: String\n";
    let fixes =
        QuickFixEngine::suggest_fixes("src/email.mantra", source, "E0021", "Found 'from'", 2, 9);
    assert!(!fixes.is_empty());
    assert!(fixes[0].replacement.contains("from_email"));

    let applied = QuickFixEngine::apply_patch(source, &fixes[0]);
    assert!(applied.contains("pub from_email: String"));
}

#[test]
fn test_ai_completion_intent_ghost_text() {
    let source = r#"
class UserService:
    pub def getUser(self, id: Int) -> User:
        # fetch user by id from db
"#;
    let ctx = CompilerIntelligenceContext::from_source("src/user.mantra", source, 4, 10);
    let suggestions = AiCompletionEngine::complete(&ctx, "");
    assert!(!suggestions.is_empty());
    let ghost = suggestions
        .iter()
        .find(|s| s.is_ai_ghost_text)
        .expect("Ghost text suggestion expected");
    assert!(ghost.insert_text.contains("SELECT"));
}

#[test]
fn test_codegen_and_syntax_validation() {
    let code = CodeGenerator::generate_class("Create a User entity with email and id")
        .expect("Class generation failed");
    assert!(code.contains("class User:"));
    assert!(code.contains("pub id: Int"));
    assert!(code.contains("pub email: String"));
}

#[test]
fn test_test_generation() {
    let source = r#"
class Calculator:
    pub def add(self, a: Int, b: Int) -> Int:
        return a + b
"#;
    let tests = TestGenerator::generate_tests_for_source("src/calc.mantra", source)
        .expect("Test generation failed");
    assert!(tests.contains("def test_calculator_initialization"));
    assert!(tests.contains("def test_calculator_add"));
    assert!(tests.contains("assert("));
}

#[test]
fn test_doc_generation() {
    let source = r#"
class Account:
    pub balance: Float
    pub def deposit(self, amount: Float) -> Bool:
        return True
"#;
    let md = DocGenerator::generate_markdown_doc("src/account.mantra", source)
        .expect("Doc generation failed");
    assert!(md.contains("# API Reference: `src/account.mantra`"));
    assert!(md.contains("## Class `Account`"));
    assert!(md.contains("### `def deposit"));
}

#[test]
fn test_refactoring_engine() {
    let source =
        "def calculate_price():\n    let base = 100\n    let tax = 20\n    return base + tax\n";
    let renamed = RefactoringEngine::rename_symbol(source, "tax", "tax_rate");
    assert!(renamed.contains("let tax_rate = 20"));

    let async_code = RefactoringEngine::convert_to_async(source, "calculate_price");
    assert!(async_code.contains("async def calculate_price():"));
}

#[test]
fn test_code_auditor() {
    let source = r#"
let open_ai_key = "sk-proj-1234567890abcdef"
let password = "SecretPassword123"
panic("Fatal crash")
"#;
    let issues = CodeAuditor::audit_source("src/app.mantra", source);
    assert!(issues
        .iter()
        .any(|i| i.category == "Security" && i.message.contains("credential")));
    assert!(issues
        .iter()
        .any(|i| i.category == "Security" && i.message.contains("password")));
    assert!(issues
        .iter()
        .any(|i| i.category == "CodeQuality" && i.message.contains("panic")));
}

#[test]
fn test_mantra_intelligence_facade() {
    let desc = MantraIntelligence::describe("AI Error Intelligence");
    assert!(desc.contains("MVP 2"));
}

#[test]
fn test_project_graph_indexes_manifest_modules_and_imports() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "mantra_intelligence_graph_{}_{}",
        std::process::id(),
        nonce
    ));
    fs::create_dir_all(root.join("src/services")).unwrap();
    fs::write(
        root.join("mantra.toml"),
        "[project]\nname = \"shop\"\ntype = \"backend\"\n\n[dependencies]\nmantra_rest = \"0.7\"\n",
    )
    .unwrap();
    fs::write(
        root.join("src/services/order.mantra"),
        "from mantra_rest import Router\nclass OrderService:\n    def create_order(self):\n        return True\n",
    )
    .unwrap();

    let graph = ProjectGraph::index(&root).unwrap();
    assert_eq!(graph.project_name, "shop");
    assert_eq!(graph.project_type, "backend");
    assert!(graph.dependencies.contains(&"mantra_rest".to_string()));
    let module = graph.modules.get("order").expect("order module indexed");
    assert!(module.classes.contains(&"OrderService".to_string()));
    assert!(module
        .imports
        .iter()
        .any(|import| import.contains("mantra_rest")));

    fs::remove_dir_all(root).unwrap();
}
