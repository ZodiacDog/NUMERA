# Contributing to NUMERA

NUMERA is a free contribution to AI safety. Contributions that improve its coverage, correctness, or integration capabilities are welcome.

## How to Contribute

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo test` — all 332 tests must pass
5. Submit a pull request

## What We Need

- **More mathematical rules** — differential equations, number theory, probability distributions, discrete math, numerical methods
- **Integration examples** — LangChain, LlamaIndex, vLLM, HuggingFace Transformers, etc.
- **Language ports** — Python, C, Go, TypeScript
- **Performance optimization** — especially matrix operations and symbolic computation

## Rules for New Mathematical Rules

Every rule added to the Rule Library must have a unique ID, belong to an existing domain (or propose a new Tier 10+ domain), include name/description/formula/examples, list all prerequisite rules, have no forward prerequisites (no rule may depend on a higher tier), and include a pattern string for the Pattern Engine.

## Design Principles — Non-Negotiable

- **Ground, Don't Compute:** Every integer operation routes through Value Core
- **Verify, Don't Trust:** Every result verified via inverse operations
- **Trace, Don't Hide:** Every step recorded
- **Zero Tolerance:** Zero test failures. Zero unverified operations. Zero shortcuts

Do not submit code that introduces non-deterministic behavior into mathematical computation paths.

## Contact

M. L. McKnight — ml.innovations.research.lab@gmail.com
