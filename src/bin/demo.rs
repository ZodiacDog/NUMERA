// ============================================================================
// NUMERA v0.1.0 — Full Stack Demonstration
// ML Innovations LLC — M. L. McKnight — Pheba, Mississippi
// FREE — No license. No restrictions. Stop the hallucinations.
// ============================================================================

use numera::integration::{self, Numera};
use std::collections::HashMap;

fn main() {
    println!("================================================================");
    println!("  NUMERA v0.1.0");
    println!("  Numerical Understanding through Modular Engine");
    println!("  for Retrieval and Application");
    println!("  ML Innovations LLC — M. L. McKnight");
    println!("================================================================");
    println!();

    let numera = Numera::new();
    println!("{}", numera.version_info());
    println!();

    // --- Library Stats ---
    let stats = numera.library_stats();
    println!("Rule Library: {} rules across {} domains ({} tiers)",
        stats.total_rules, stats.total_domains, stats.total_tiers);
    for (domain, count) in &stats.rules_per_domain {
        println!("  {:20} — {} rules", domain, count);
    }
    println!();

    // --- Tier 0: Core Arithmetic (Value Core Direct) ---
    println!("=== Tier 0: Core Arithmetic ===");
    for (expr, expected) in &[("3 + 5", "8"), ("9 - 4", "5"), ("7 * 8", "56"), ("9 / 3", "3")] {
        let r = numera.evaluate(expr);
        println!("  {} = {} [verified: {}] domain: {}",
            expr, r.result, r.verified,
            r.domain.as_deref().unwrap_or("?"));
        assert_eq!(r.result, *expected);
    }
    println!();

    // --- Tier 1: Multi-Digit Arithmetic ---
    println!("=== Tier 1: Multi-Digit Arithmetic ===");
    for (expr, expected) in &[("347 + 286", "633"), ("1000 - 1", "999"), ("12 * 34", "408"), ("99242 / 347", "286")] {
        let r = numera.evaluate(expr);
        println!("  {} = {} [verified: {}]", expr, r.result, r.verified);
        assert_eq!(r.result, *expected);
    }
    println!();

    // --- Exponents & Roots ---
    println!("=== Exponents & Roots ===");
    for (expr, expected) in &[("2 ^ 10", "1024"), ("3 ^ 4", "81"), ("sqrt(144)", "12"), ("sqrt(625)", "25")] {
        let r = numera.evaluate(expr);
        println!("  {} = {} [verified: {}]", expr, r.result, r.verified);
        assert_eq!(r.result, *expected);
    }
    println!();

    // --- Order of Operations ---
    println!("=== Order of Operations (PEMDAS) ===");
    for (expr, expected) in &[("2 + 3 * 4", "14"), ("(2 + 3) * 4", "20"), ("2 * 3 ^ 2", "18"), ("(2 + 3) * (4 + 5)", "45")] {
        let r = numera.evaluate(expr);
        println!("  {} = {} [verified: {}]", expr, r.result, r.verified);
        assert_eq!(r.result, *expected);
    }
    println!();

    // --- Variable Substitution ---
    println!("=== Variable Substitution ===");
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 7.0);
    vars.insert("y".to_string(), 3.0);
    let r = numera.evaluate_with_vars("x * y + 1", vars);
    println!("  x=7, y=3: x * y + 1 = {} [verified: {}]", r.result, r.verified);
    assert_eq!(r.result, "22");
    println!();

    // --- Batch Evaluation ---
    println!("=== Batch Evaluation ===");
    let batch = integration::eval_batch(&[
        "1 + 1", "2 * 3", "100 - 37", "5!",
        "sqrt(81)", "2 ^ 8", "(10 + 5) * 2",
    ]);
    for resp in &batch.responses {
        println!("  {} = {} [verified: {}]", resp.expression, resp.result, resp.verified);
    }
    println!("  Batch: {}/{} succeeded, all verified: {}",
        batch.succeeded, batch.total, batch.all_verified);
    println!();

    // --- Step-by-Step Trace ---
    println!("=== Execution Trace: 347 + 286 ===");
    let r = numera.evaluate("347 + 286");
    for (i, step) in r.trace.iter().enumerate() {
        let check = if step.verified { "+" } else { "?" };
        println!("  Step {}: [{}] {} -> {}", i + 1, check, step.operation, step.description);
    }
    println!("  Final: {} [verified: {}]", r.result, r.verified);
    println!();

    // --- JSON Output ---
    println!("=== JSON Response ===");
    let r = integration::eval_full("2 ^ 10");
    println!("  {}", r.to_json());
    println!();

    // --- Final ---
    println!("================================================================");
    println!("  NUMERA: 6 Layers. {} Rules. Every Result Verified.", stats.total_rules);
    println!("  \"We DO.\" -- ML Innovations LLC");
    println!("================================================================");
}
