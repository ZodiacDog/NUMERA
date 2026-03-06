# NUMERA Integration Examples

## Rust — Direct Usage

```rust
use numera::integration::{Numera, eval_batch, eval_full};
use std::collections::HashMap;

fn main() {
    let numera = Numera::new();

    // Simple evaluation
    let r = numera.evaluate("347 * 892");
    println!("{} = {} [verified: {}]", r.expression, r.result, r.verified);

    // With variables
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 7.0);
    let r = numera.evaluate_with_vars("x * 3 + 1", vars);
    println!("x=7: x*3+1 = {}", r.result);

    // Batch evaluation
    let batch = eval_batch(&["2 + 2", "3 ^ 4", "sqrt(144)", "5!"]);
    for resp in &batch.responses {
        println!("{} = {} [verified: {}]", resp.expression, resp.result, resp.verified);
    }

    // Full JSON response
    let full = eval_full("2 ^ 10");
    println!("{}", full.to_json());
}
```

## OpenAI Tool Use

```python
tools = [{
    "type": "function",
    "function": {
        "name": "numera_solve",
        "description": "Deterministic math engine. Use for ANY calculation.",
        "parameters": {
            "type": "object",
            "properties": {
                "expression": { "type": "string" },
                "show_work": { "type": "boolean" }
            },
            "required": ["expression"]
        }
    }
}]
```

## Anthropic Tool Use

```python
tools = [{
    "name": "numera_solve",
    "description": "Deterministic math engine. Returns verified results.",
    "input_schema": {
        "type": "object",
        "properties": {
            "expression": { "type": "string" },
            "show_work": { "type": "boolean" }
        },
        "required": ["expression"]
    }
}]
```

## Integration Pattern

1. Host LLM detects math in user input
2. Routes computation to NUMERA via tool call
3. Receives verified JSON result with trace
4. Presents answer to user in natural language

The host does language. NUMERA does math. Everybody wins.
