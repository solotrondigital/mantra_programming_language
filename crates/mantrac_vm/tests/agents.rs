use mantra_agents::{Agent, AgentModel, ChatMessage, ModelReply};
use mantrac_checker::TypeChecker;
use mantrac_lexer::Lexer;
use mantrac_parser::Parser;
use mantrac_vm::{Interpreter, Value};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

struct ScriptedModel(Mutex<VecDeque<String>>);

impl AgentModel for ScriptedModel {
    fn complete(&self, system: &str, _messages: &[ChatMessage]) -> Result<ModelReply, String> {
        assert!(system.contains("Registered tools"));
        let response = self
            .0
            .lock()
            .unwrap()
            .pop_front()
            .ok_or("script exhausted")?;
        Ok(ModelReply {
            text: response,
            prompt_tokens: 3,
            completion_tokens: 2,
            estimated_cost_usd: 0.001,
        })
    }
    fn model_name(&self) -> &str {
        "vm-test-model"
    }
}

fn program(source: &str) -> mantrac_ast::Program {
    let combined = format!(
        "{}\n{}",
        include_str!("../../mantra_agents/lib/mantra_agents.mantra"),
        source
    );
    let tokens = Lexer::new(&combined)
        .tokenize()
        .expect("Mantra source lexes");
    Parser::new(tokens).parse().expect("Mantra source parses")
}

#[test]
fn mantra_api_creates_agent_with_model_policy_and_tool_registration() {
    let source = r#"
pub def lookup(order_id: Int) -> String:
    return "order:" + str(order_id)

def main() -> Int:
    let assistant = Agent.create("Support", "openai", "gpt-4o-mini", "test-key", "Help with orders", "", 4, 3, 20)
    Agent.register_tool(assistant, "lookup", "Find order", "lookup", False)
    return 0
"#;
    let program = program(source);
    TypeChecker::new()
        .check_program(&program)
        .expect("agent declarations type-check");

    let imported_source = r#"
import mantra.agents
def main() -> Int:
    let assistant = Agent.create("Support", "openai", "gpt-4o-mini", "test-key", "Help", "", 4, 3, 20)
    return 0
"#;
    let imported = Parser::new(Lexer::new(imported_source).tokenize().unwrap())
        .parse()
        .unwrap();
    TypeChecker::new()
        .check_program(&imported)
        .expect("Agent is available as a Mantra built-in class");

    let mut interpreter = Interpreter::new();
    assert_eq!(
        interpreter.execute_program(&program).unwrap(),
        Value::Int(0)
    );
}

#[test]
fn mantra_function_runs_as_tool_and_human_approval_can_resume() {
    let source = r#"
pub def send_notice(customer_id: Int) -> String:
    return "notice sent to " + str(customer_id)

def main() -> String:
    Agent.register_tool(support_agent, "send_notice", "Send customer notice", "send_notice", True)
    let pending = Agent.run(support_agent, "customer-9", "Send notice to customer 9")
    assert(pending["status"] == "awaiting_approval")
    let finished = Agent.resume(support_agent, pending["run_id"], True)
    assert(finished["status"] == "completed")
    return finished["output"]
"#;
    let program = program(source);

    let scripted = ScriptedModel(Mutex::new(VecDeque::from([
        r#"{"type":"tool_call","name":"send_notice","arguments":{"customer_id":9},"reason":"The user requested a notice"}"#.into(),
        r#"{"type":"final","content":"Notice sent."}"#.into(),
    ])));
    let agent = Arc::new(Agent::builder("Support", scripted).build().unwrap());
    let mut interpreter = Interpreter::new();
    let agent_id = interpreter.register_agent_for_host(agent);
    interpreter
        .globals
        .insert("support_agent".into(), Value::AgentHandle(agent_id));

    assert_eq!(
        interpreter.execute_program(&program).unwrap(),
        Value::String("Notice sent.".into())
    );
}

#[test]
fn mantra_agents_can_collaborate_with_sequential_handoffs() {
    let source = r#"
def main() -> String:
    let result = Agent.collaborate([research_agent, review_agent], "team-session", "Research release risk")
    return result["final"]
"#;
    let program = program(source);
    let research = Arc::new(
        Agent::builder(
            "Research",
            ScriptedModel(Mutex::new(VecDeque::from([
                r#"{"type":"final","content":"Risk: migration may lose indexes."}"#.into(),
            ]))),
        )
        .build()
        .unwrap(),
    );
    let reviewer = Arc::new(
        Agent::builder(
            "Reviewer",
            ScriptedModel(Mutex::new(VecDeque::from([
                r#"{"type":"final","content":"Mitigate by snapshotting indexes first."}"#.into(),
            ]))),
        )
        .build()
        .unwrap(),
    );

    let mut interpreter = Interpreter::new();
    let research_id = interpreter.register_agent_for_host(research);
    let review_id = interpreter.register_agent_for_host(reviewer);
    interpreter
        .globals
        .insert("research_agent".into(), Value::AgentHandle(research_id));
    interpreter
        .globals
        .insert("review_agent".into(), Value::AgentHandle(review_id));
    let result = interpreter.execute_program(&program).unwrap();
    assert_eq!(
        result,
        Value::String("Mitigate by snapshotting indexes first.".into())
    );
}
