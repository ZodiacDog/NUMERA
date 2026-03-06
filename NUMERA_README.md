# NUMERA

### Numerical Understanding through Modular Engine for Retrieval and Application

**A deterministic mathematical intelligence engine that eliminates mathematical hallucination in AI systems.**

Built by [M. L. McKnight](mailto:ml.innovations.research.lab@gmail.com) · ML Innovations LLC · Pheba, Mississippi

---

```
Expression: 347 × 892
LLM answer:  ¯\_(ツ)_/¯  (token prediction)
NUMERA answer: 309,524 [VERIFIED ✓] (deterministic computation)
```

---

## The Problem

Every AI model on Earth handles math the same way it handles language: by predicting the next token.

The symbol `+` means addition. Always. Everywhere. There is no mathematical sarcasm. No mathematical idiom. No context-dependent ambiguity. The symbols ARE the meaning.

Yet every LLM treats `2 + 2` the same way it treats "write me a poem" — as a probability distribution over the next token.

**This is not a training problem. It is not a data problem. It is an architectural problem.** You are predicting answers that should be computed.

## The Solution

NUMERA's core principle: **Know, Don't Predict.**

NUMERA does not predict mathematical answers. It retrieves known values and applies verified rules. Every result carries an auditable proof chain back to foundational facts. There is no guessing.

### Architecture — 6 Layers

| Layer | Name | Rust Lines | Tests | Function |
|-------|------|-----------|-------|----------|
| 0 | **Value Core** | 1,132 | 71 | Foundational values 0-9, pre-computed arithmetic tables |
| 1 | **Rule Library** | 2,373 | 52 | 111 verified rules across 10 mathematical domains |
| 2 | **Pattern Engine** | 1,793 | 76 | Pratt parser, AST builder, expression classifier |
| 3 | **Retrieval Layer** | 940 | 42 | Deterministic RAG — exact rule matching, no guessing |
| 4 | **Execution Engine** | 1,624 | 49 | Verified computation with full trace chains |
| 5 | **Integration API** | 1,053 | 42 | JSON in, structured result out — any LLM can call NUMERA |
| | **TOTAL** | **9,046** | **332** | |

### The 10 Mathematical Domains (111 Rules)

| Tier | Domain | Rules | Coverage |
|------|--------|-------|----------|
| 0 | Core Arithmetic | 4 | add, subtract, multiply, divide |
| 1 | Extended Arithmetic | 7 | multi-digit operations, order of operations, place value |
| 2 | Properties | 6 | commutative, associative, distributive, identity, inverse |
| 3 | Pre-Algebra | 16 | fractions, decimals, percentages, exponents, roots, GCD/LCM |
| 4 | Algebra | 14 | variables, equations, polynomials, factoring, functions |
| 5 | Advanced Algebra | 16 | systems, matrices, complex numbers, sequences, logarithms |
| 6 | Trigonometry | 10 | trig functions, identities, unit circle, law of sines/cosines |
| 7 | Calculus | 11 | limits, derivatives, integrals, Taylor series, ODEs |
| 8 | Linear Algebra | 10 | vectors, eigenvalues, determinants, projections |
| 9 | Abstract & Applied | 17 | modular arithmetic, primes, combinatorics, statistics, ML Identity |
| | **TOTAL** | **111** | |

### Dual Implementation

| Implementation | Lines | Tests | Status |
|---------------|-------|-------|--------|
| **Rust** | 9,046 | 332 | Production — compiled, memory-safe, zero dependencies |
| **SCRAWL** | 2,860 | 38 self-tests | Agent-native — ROSETTA pseudocode, SCRAWL VM executable |
| **Grand Total** | **11,906** | **370** | **0 failures** |

### Exhaustive Verification

NUMERA doesn't just test samples. It exhaustively verifies:

| Verification | Range | Pairs Verified |
|-------------|-------|---------------|
| Addition | 0+0 through 50+50 | 2,601 |
| Multiplication | 0×0 through 30×30 | 961 |
| API Addition | 0+0 through 20+20 | 441 |
| API Multiplication | 0×0 through 12×12 | 169 |
| Inverse Verification (Add) | 0-30 range | 961 |
| Inverse Verification (Mul) | 0-20 range | 441 |
| Single-Digit Direct | All 200 facts | 200 |
| **TOTAL** | | **5,774+** |

## Quick Start

```bash
# Clone it
git clone https://github.com/mlinnovationsllc/numera.git
cd numera

# Run the tests — see for yourself
cargo test

# Run the demo
cargo run --bin numera-demo
```

## LLM Integration

NUMERA integrates into any LLM with tool-use / function-calling infrastructure. When the host model detects a mathematical expression, it routes the computation to NUMERA instead of attempting token prediction.

### OpenAI Tool Definition

```json
{
  "type": "function",
  "function": {
    "name": "numera_solve",
    "description": "Deterministic mathematical computation engine. Use for ANY math operation.",
    "parameters": {
      "type": "object",
      "properties": {
        "expression": {
          "type": "string",
          "description": "Mathematical expression to solve"
        },
        "show_work": {
          "type": "boolean",
          "description": "Return step-by-step working"
        }
      },
      "required": ["expression"]
    }
  }
}
```

### Anthropic Tool Definition

```json
{
  "name": "numera_solve",
  "description": "Deterministic mathematical computation engine. Returns verified results with confidence 1.0.",
  "input_schema": {
    "type": "object",
    "properties": {
      "expression": { "type": "string" },
      "show_work": { "type": "boolean" }
    },
    "required": ["expression"]
  }
}
```

### What the Host LLM Receives

```json
{
  "expression": "347 * 892",
  "result": "309524",
  "result_type": "integer",
  "domain": "Core Arithmetic",
  "verified": true,
  "confidence": 1.0,
  "rules_applied": ["multiplication"],
  "trace": [
    { "operation": "multi_digit_mul", "description": "347 × 892 via partial products", "verified": true }
  ],
  "error": null
}
```

**Estimated integration effort: ~1-2 engineering weeks.**

## Usage in Rust

```rust
use numera::integration::Numera;

fn main() {
    let numera = Numera::new();

    // Evaluate any mathematical expression
    let result = numera.evaluate("347 * 892");
    println!("{} = {} [verified: {}]", result.expression, result.result, result.verified);
    // Output: 347 * 892 = 309524 [verified: true]

    // Full JSON response
    let full = numera::integration::eval_full("2 ^ 10");
    println!("{}", full.to_json());

    // Variable substitution
    let mut vars = std::collections::HashMap::new();
    vars.insert("x".to_string(), 7.0);
    let result = numera.evaluate_with_vars("x * 3 + 1", vars);
    println!("{}", result.result); // 22

    // Batch evaluation
    let batch = numera::integration::eval_batch(&["2 + 2", "3 ^ 4", "sqrt(144)"]);
    println!("All verified: {}", batch.all_verified);
}
```

## The Safety Argument

When an AI confidently gives a wrong calculation to:
- A **physician** determining drug dosages
- An **engineer** calculating structural tolerances
- A **financial analyst** modeling risk exposure
- A **student** learning foundational math

That is not a minor bug. That is a **safety failure**.

Mathematical reliability should not be a competitive advantage. It should be a **safety standard**.

NUMERA exists to make that standard achievable for every AI system.

## Free. No License. Stop the Hallucinations.

This code is **free**. Not "free as in MIT/Apache." Not "free as in some open-source license you need a lawyer to read." **Free as in: take it. Use it. Ship it. Modify it. Don't ask permission. Don't pay anyone. Don't sign anything.**

There is no license because you don't need one. The intent is absolute and unconditional:

**Use NUMERA to make AI better at math. Period.**

If you take this work and integrate it into your system and don't credit the source — well, that says more about you than it does about me, and I won't lose sleep over it. But if you've got a shred of decency, a mention would be appreciated. Either way, the math gets fixed, and that's the point.

Mathematical hallucination is a safety problem. The solution should not be monetized. It should be distributed as widely and as freely as possible until every AI system on Earth gets math right every single time.

**No license. No fees. No strings. No catch. No lawyers. Just stop the hallucinations.**

## Design Principles

**Ground, Don't Compute:** Every integer operation routes through Value Core. No native arithmetic for foundational computation.

**Verify, Don't Trust:** Every result verified via inverse operations. Confidence = 1.0 only when verified.

**Trace, Don't Hide:** Every step recorded. Full transparency from input to output.

**Zero Tolerance:** Zero test failures. Zero unverified foundational operations. Zero shortcuts.

## The ML Identity

NUMERA traces its mathematical foundation to the ML Identity equation, discovered in 1999:

```
a + a² + b = b²    where b = a + 1
```

This identity holds for all real numbers and serves as the Mathematical Root of Trust for the ML Innovations research program.

## Project Structure

```
numera/
├── Cargo.toml                              # Zero dependencies, no license
├── README.md
├── PROJECT_SUMMARY.md
├── CONTRIBUTING.md
├── src/
│   ├── lib.rs                              # Module declarations
│   ├── value_core.rs                       # Layer 0 (1,132 lines, 71 tests)
│   ├── rule_library.rs                     # Layer 1 (2,373 lines, 52 tests)
│   ├── pattern_engine.rs                   # Layer 2 (1,793 lines, 76 tests)
│   ├── retrieval.rs                        # Layer 3 (940 lines, 42 tests)
│   ├── execution.rs                        # Layer 4 (1,624 lines, 49 tests)
│   ├── integration.rs                      # Layer 5 (1,053 lines, 42 tests)
│   └── bin/
│       └── demo.rs                         # Full stack demonstration
├── scrawl/                                 # SCRAWL reference implementation
│   ├── numera_main.scrawl                  # Master entry + 38 self-tests
│   ├── numera_layer0_value_core.scrawl
│   ├── numera_layer1_rule_library.scrawl
│   ├── numera_layer2_pattern_engine.scrawl
│   ├── numera_layer3_retrieval.scrawl
│   ├── numera_layer4_execution.scrawl
│   └── numera_layer5_integration.scrawl
├── docs/
│   ├── NUMERA_Architecture_v1_0.docx
│   ├── NUMERA_Integration_Guide_v1_0.docx
│   └── NUMERA_v0_1_0_Project_Document.docx
└── examples/
    └── README.md                           # Integration examples
```

## Contact

**M. L. McKnight**
Founder & CTO, ML Innovations LLC
Pheba, Mississippi

Email: [ml.innovations.research.lab@gmail.com](mailto:ml.innovations.research.lab@gmail.com)

---

*The host handles language. NUMERA handles math. This is not a compromise. It is a division of labor that respects the fundamental nature of both domains.*

*"We DO." — ML Innovations LLC*
