# mantra_agents

`mantra_agents` is the MVP 3 runtime crate. It builds on `mantra_ai` and implements a bounded single-agent loop, allow-listed function tools, working and conversation memory, optional persistent notes, human approval, resumable runs, and execution traces.

## Define an agent

The VM exposes `Agent` as a built-in Mantra class. Agent tool handlers are regular Mantra functions and are resolved by the function name registered with the agent:

```mantra
import mantra.agents

pub def lookup_order(order_id: Int) -> String:
    return "Order " + str(order_id) + " is shipped"

def main() -> Int:
    let assistant = Agent.create(
        "SupportAgent", "openai", "gpt-4o-mini",
        get_env("OPENAI_API_KEY", ""),
        "Answer support questions using order data.",
        "data/support-memory.json", 8, 6, 45
    )
    Agent.register_tool(assistant, "lookup_order", "Look up an order by ID", "lookup_order", False)
    let result = Agent.run(assistant, "customer-42", "Find order 42")
    if result["status"] == "awaiting_approval":
        print(result["approval"])
        # Replace True with the human review decision from the host application.
        let result = Agent.resume(assistant, result["run_id"], True)
    print(result["output"])
    return 0
```

The Mantra API uses existing class and function syntax rather than introducing a new `agent { ... }` grammar. The VM preloads `Agent`; `Agent.create` selects the MVP1 provider/model, `Agent.register_tool` binds a declared Mantra function, and `Agent.run` executes the model/tool loop. Run agent programs with `mantrac <file.mantra> --interpret`; the native C backend does not yet embed the hosted-model runtime.

## Rust runtime

```rust,no_run
use mantra_agents::{Agent, AgentTool, ExecutionLimits};
use mantra_ai::{AI, env};
use serde_json::json;
use std::time::Duration;

let ai = AI::new();
let model = ai.model("openai", "gpt-4o", env("OPENAI_API_KEY"));
let agent = Agent::builder("SupportAgent", model)
    .instructions("Answer order questions. Ask before sending messages.")
    .tool(AgentTool::new("lookup_order", "Read an order by ID", false, |args| {
        Ok(json!({"order_id": args["id"], "status": "shipped"}))
    }))
    .tool(AgentTool::new("send_email", "Send a customer email", true, |_args| {
        Ok(json!({"sent": true}))
    }))
    .limits(ExecutionLimits {
        max_model_steps: 8,
        max_tool_calls: 6,
        timeout: Duration::from_secs(45),
    })
    .build()?;

let result = agent.run_in("customer-42", "Find my latest order");
if let Some(approval) = result.approval {
    // Show tool name, arguments, and reason before making an approval decision.
    let resumed = agent.resume(&approval.run_id, true);
    println!("{}", resumed.output.unwrap_or_default());
}
# Ok::<(), String>(())
```

## Safety and control model

- Tools must be registered by the host. The model can request only named tools in that registry.
- A tool marked `requires_approval` is never executed during `run`; the result is `awaiting_approval`. The host decides whether to call `resume(run_id, true)` or reject it.
- Every run has maximum model steps, maximum executed tool calls, and a wall-clock deadline checked between model and tool operations. Provider requests use the timeout configured by the `mantra_ai` client/provider.
- Tool output is labelled untrusted in subsequent model context. Tool handlers remain ordinary host code, so they must validate arguments and enforce their own authorization too.
- Conversation memory is isolated by session ID in process memory. `Agent.set_memory` stores session-scoped facts. `Agent.remember` stores durable notes in a JSON file configured at agent creation.
- `RunResult.trace` records lifecycle events, model/tool counts, token counts, and estimated provider cost.
- `Agent.collaborate([agents], session, task)` hands each agent the preceding result in sequence and stops on failure or approval.
- Approval checkpoints are written beside configured persistent memory and can be resumed after recreating the agent with the same memory path and tool registrations.

The runtime is synchronous in this MVP. Its deadline cannot interrupt a currently blocked host tool; handlers should use their own I/O timeouts. `Agent.collaborate` supports sequential handoffs across agents; parallel orchestration and durable conversation transcripts are outside this MVP scope.

## Model tool-call contract

The runtime improves prompt reliability by requiring a single JSON action per model turn, with no markdown:

```json
{"type":"tool_call","name":"lookup_order","arguments":{"id":42},"reason":"Find the requested order"}
```

or:

```json
{"type":"final","content":"Order 42 is shipped."}
```

The system prompt tells the model to treat user input, memory, and tool results as data; use only registered tools; minimize side effects; honor approval gates; and verify tool output before answering. Invalid JSON, model failures, and limit violations fail closed with a trace.

Run the crate tests with `cargo test -p mantra_agents`.
