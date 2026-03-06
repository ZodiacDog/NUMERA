// NUMERA Layer 5: Integration API
// Copyright (c) 2026 ML Innovations LLC — M. L. McKnight
// FREE — No license. No restrictions. Stop the hallucinations.
//
// The Integration API is the public interface to the NUMERA engine.
// It wraps all five lower layers into a clean, consumable API with:
//
//   - Structured request/response types
//   - Session management with variable persistence
//   - Batch evaluation
//   - Full pipeline access: parse → classify → retrieve → execute → verify
//   - Introspection: rule lookup, prerequisite chains, tier analysis
//   - JSON-compatible serialization for HTTP/REST integration
//
// Usage:
//   let numera = Numera::new();
//   let response = numera.evaluate("347 + 286");
//   assert_eq!(response.result, "633");
//   assert!(response.verified);

use crate::execution::{ExecutionEngine, ExecutionResult, MathValue, Step};
use crate::pattern_engine::{self, AstNode, PatternClassification};
use crate::retrieval::{RetrievalEngine, RetrievalResult};
use crate::rule_library::{Domain, PatternType};
use std::collections::HashMap;
use std::fmt;

// ============================================================================
// VERSION & METADATA
// ============================================================================

/// NUMERA version string.
pub const VERSION: &str = "0.1.0";
/// Engine name.
pub const ENGINE_NAME: &str = "NUMERA";
/// Full engine description.
pub const ENGINE_DESC: &str = "Numerical Understanding through Modular Engine for Retrieval and Application";
/// Author.
pub const AUTHOR: &str = "M. L. McKnight — ML Innovations LLC";
/// License.
pub const LICENSE: &str = "FREE — No license. No restrictions.";

// ============================================================================
// REQUEST
// ============================================================================

/// A request to the NUMERA engine.
#[derive(Debug, Clone)]
pub struct Request {
    /// The mathematical expression to evaluate.
    pub expression: String,
    /// Optional variable bindings.
    pub variables: HashMap<String, f64>,
    /// Whether to include the step-by-step trace.
    pub include_trace: bool,
    /// Whether to include rule chain information.
    pub include_rules: bool,
    /// Whether to include classification metadata.
    pub include_classification: bool,
    /// Maximum tier to use (None = all tiers).
    pub max_tier: Option<u8>,
}

impl Request {
    /// Create a simple evaluation request.
    pub fn new(expression: &str) -> Self {
        Request {
            expression: expression.to_string(),
            variables: HashMap::new(),
            include_trace: true,
            include_rules: true,
            include_classification: true,
            max_tier: None,
        }
    }

    /// Create a request with variable bindings.
    pub fn with_variables(expression: &str, variables: HashMap<String, f64>) -> Self {
        Request {
            expression: expression.to_string(),
            variables,
            include_trace: true,
            include_rules: true,
            include_classification: true,
            max_tier: None,
        }
    }

    /// Create a minimal request (no trace, no rules, no classification).
    pub fn minimal(expression: &str) -> Self {
        Request {
            expression: expression.to_string(),
            variables: HashMap::new(),
            include_trace: false,
            include_rules: false,
            include_classification: false,
            max_tier: None,
        }
    }
}

// ============================================================================
// RESPONSE
// ============================================================================

/// The response from the NUMERA engine.
#[derive(Debug, Clone)]
pub struct Response {
    /// The expression that was evaluated.
    pub expression: String,
    /// The result as a display string.
    pub result: String,
    /// The result as a structured MathValue.
    pub value: MathValue,
    /// Whether the result was verified via inverse operations.
    pub verified: bool,
    /// Confidence score (1.0 if verified, lower otherwise).
    pub confidence: f64,
    /// The mathematical domain of the expression.
    pub domain: Option<String>,
    /// The tier level of the expression.
    pub tier: Option<u8>,
    /// The classified pattern type.
    pub pattern: Option<String>,
    /// Step-by-step execution trace.
    pub trace: Vec<TraceStep>,
    /// Rules that were used.
    pub rules_used: Vec<String>,
    /// Error message if evaluation failed.
    pub error: Option<String>,
    /// Whether the evaluation succeeded.
    pub success: bool,
    /// Engine metadata.
    pub engine: String,
    /// Engine version.
    pub version: String,
}

impl Response {
    /// Serialize to a simple JSON-compatible string.
    pub fn to_json(&self) -> String {
        let mut parts = Vec::new();
        parts.push(format!("\"expression\": \"{}\"", escape_json(&self.expression)));
        parts.push(format!("\"result\": \"{}\"", escape_json(&self.result)));
        parts.push(format!("\"verified\": {}", self.verified));
        parts.push(format!("\"confidence\": {}", self.confidence));
        parts.push(format!("\"success\": {}", self.success));

        if let Some(ref domain) = self.domain {
            parts.push(format!("\"domain\": \"{}\"", domain));
        }
        if let Some(tier) = self.tier {
            parts.push(format!("\"tier\": {}", tier));
        }
        if let Some(ref pattern) = self.pattern {
            parts.push(format!("\"pattern\": \"{}\"", pattern));
        }
        if let Some(ref error) = self.error {
            parts.push(format!("\"error\": \"{}\"", escape_json(error)));
        }
        if !self.rules_used.is_empty() {
            let rules: Vec<String> = self.rules_used.iter()
                .map(|r| format!("\"{}\"", r))
                .collect();
            parts.push(format!("\"rules_used\": [{}]", rules.join(", ")));
        }
        if !self.trace.is_empty() {
            let steps: Vec<String> = self.trace.iter()
                .map(|s| s.to_json())
                .collect();
            parts.push(format!("\"trace\": [{}]", steps.join(", ")));
        }

        parts.push(format!("\"engine\": \"{}\"", self.engine));
        parts.push(format!("\"version\": \"{}\"", self.version));

        format!("{{{}}}", parts.join(", "))
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.success {
            write!(f, "{} = {} [{}]",
                self.expression,
                self.result,
                if self.verified { "verified ✓" } else { "unverified" }
            )
        } else {
            write!(f, "{} → ERROR: {}",
                self.expression,
                self.error.as_deref().unwrap_or("unknown error")
            )
        }
    }
}

/// A trace step in the response.
#[derive(Debug, Clone)]
pub struct TraceStep {
    pub operation: String,
    pub description: String,
    pub result: String,
    pub verified: bool,
}

impl TraceStep {
    fn from_step(step: &Step) -> Self {
        TraceStep {
            operation: step.operation.clone(),
            description: step.description.clone(),
            result: step.result.clone(),
            verified: step.verified,
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"operation\": \"{}\", \"description\": \"{}\", \"result\": \"{}\", \"verified\": {}}}",
            escape_json(&self.operation),
            escape_json(&self.description),
            escape_json(&self.result),
            self.verified
        )
    }
}

// ============================================================================
// BATCH REQUEST/RESPONSE
// ============================================================================

/// A batch evaluation request.
#[derive(Debug, Clone)]
pub struct BatchRequest {
    pub requests: Vec<Request>,
}

/// A batch evaluation response.
#[derive(Debug, Clone)]
pub struct BatchResponse {
    pub responses: Vec<Response>,
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub all_verified: bool,
}

impl BatchResponse {
    pub fn to_json(&self) -> String {
        let responses: Vec<String> = self.responses.iter()
            .map(|r| r.to_json())
            .collect();
        format!(
            "{{\"total\": {}, \"succeeded\": {}, \"failed\": {}, \"all_verified\": {}, \"responses\": [{}]}}",
            self.total, self.succeeded, self.failed, self.all_verified,
            responses.join(", ")
        )
    }
}

// ============================================================================
// INTROSPECTION TYPES
// ============================================================================

/// Information about a rule in the library.
#[derive(Debug, Clone)]
pub struct RuleInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub domain: String,
    pub tier: u8,
    pub prerequisites: Vec<String>,
    pub inverse: Option<String>,
}

/// Library statistics.
#[derive(Debug, Clone)]
pub struct LibraryStats {
    pub total_rules: usize,
    pub rules_per_domain: Vec<(String, usize)>,
    pub total_domains: usize,
    pub total_tiers: usize,
}

// ============================================================================
// NUMERA ENGINE — The unified public interface
// ============================================================================

/// The NUMERA engine. This is the primary public interface.
///
/// # Example
/// ```
/// let numera = numera::integration::Numera::new();
/// let response = numera.evaluate("347 + 286");
/// assert_eq!(response.result, "633");
/// assert!(response.verified);
/// ```
pub struct Numera {
    retrieval: RetrievalEngine,
    variables: HashMap<String, MathValue>,
}

impl Numera {
    /// Create a new NUMERA engine with the full rule library.
    pub fn new() -> Self {
        Numera {
            retrieval: RetrievalEngine::default_library(),
            variables: HashMap::new(),
        }
    }

    /// Create a NUMERA engine with pre-bound variables.
    pub fn with_variables(vars: HashMap<String, f64>) -> Self {
        let math_vars: HashMap<String, MathValue> = vars.into_iter()
            .map(|(k, v)| {
                if v.fract() == 0.0 {
                    (k, MathValue::Integer(v as i64))
                } else {
                    (k, MathValue::Float(v))
                }
            })
            .collect();
        Numera {
            retrieval: RetrievalEngine::default_library(),
            variables: math_vars,
        }
    }

    /// Set a variable binding.
    pub fn set_variable(&mut self, name: &str, value: f64) {
        if value.fract() == 0.0 {
            self.variables.insert(name.to_string(), MathValue::Integer(value as i64));
        } else {
            self.variables.insert(name.to_string(), MathValue::Float(value));
        }
    }

    /// Clear all variable bindings.
    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }

    /// Get the current variable bindings.
    pub fn variables(&self) -> &HashMap<String, MathValue> {
        &self.variables
    }

    // ====================================================================
    // PRIMARY API: EVALUATE
    // ====================================================================

    /// Evaluate a mathematical expression and return a full response.
    /// This is the primary entry point.
    pub fn evaluate(&self, expression: &str) -> Response {
        self.process_request(&Request::new(expression))
    }

    /// Evaluate with variable substitutions.
    pub fn evaluate_with_vars(&self, expression: &str, vars: HashMap<String, f64>) -> Response {
        let request = Request::with_variables(expression, vars);
        self.process_request(&request)
    }

    /// Quick evaluation — returns just the result string.
    pub fn quick_eval(&self, expression: &str) -> Result<String, String> {
        let response = self.evaluate(expression);
        if response.success {
            Ok(response.result)
        } else {
            Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
        }
    }

    // ====================================================================
    // REQUEST PROCESSING
    // ====================================================================

    /// Process a structured request.
    pub fn process_request(&self, request: &Request) -> Response {
        // Step 1: Parse
        let ast = match pattern_engine::parse(&request.expression) {
            Ok(ast) => ast,
            Err(e) => return self.error_response(&request.expression, &format!("Parse error: {:?}", e)),
        };

        // Step 2: Classify
        let classification = pattern_engine::classify(&ast);

        // Step 3: Retrieve (for metadata — the execution engine handles its own dispatch)
        let retrieval_result = self.retrieval.retrieve_from_classification(&classification, &ast);

        // Step 4: Execute
        let mut all_vars = self.variables.clone();
        for (k, v) in &request.variables {
            if v.fract() == 0.0 {
                all_vars.insert(k.clone(), MathValue::Integer(*v as i64));
            } else {
                all_vars.insert(k.clone(), MathValue::Float(*v));
            }
        }

        let engine = ExecutionEngine::with_variables(all_vars);
        let exec_result = engine.execute(&ast);

        // Step 5: Build response
        self.build_response(request, &classification, &retrieval_result, &exec_result)
    }

    /// Process a batch of requests.
    pub fn process_batch(&self, batch: &BatchRequest) -> BatchResponse {
        let responses: Vec<Response> = batch.requests.iter()
            .map(|req| self.process_request(req))
            .collect();

        let total = responses.len();
        let succeeded = responses.iter().filter(|r| r.success).count();
        let failed = total - succeeded;
        let all_verified = responses.iter().all(|r| r.verified);

        BatchResponse {
            responses,
            total,
            succeeded,
            failed,
            all_verified,
        }
    }

    // ====================================================================
    // INTROSPECTION API
    // ====================================================================

    /// Get information about a specific rule by ID.
    pub fn rule_info(&self, rule_id: &str) -> Option<RuleInfo> {
        self.retrieval.library().get(rule_id).map(|rule| {
            RuleInfo {
                id: rule.id.clone(),
                name: rule.name.clone(),
                description: rule.description.clone(),
                domain: format!("{:?}", rule.domain),
                tier: rule.tier(),
                prerequisites: rule.prerequisites.clone(),
                inverse: rule.inverse.clone(),
            }
        })
    }

    /// Get the prerequisite chain for a rule.
    pub fn prerequisite_chain(&self, rule_id: &str) -> Vec<String> {
        self.retrieval.dependency_graph(rule_id)
    }

    /// Get library statistics.
    pub fn library_stats(&self) -> LibraryStats {
        let lib = self.retrieval.library();
        let total = lib.count();
        let mut rules_per_domain = Vec::new();
        for domain in Domain::all() {
            let count = lib.count_domain(*domain);
            rules_per_domain.push((format!("{:?}", domain), count));
        }
        LibraryStats {
            total_rules: total,
            rules_per_domain,
            total_domains: Domain::all().len(),
            total_tiers: 10,
        }
    }

    /// Get the total number of rules in the library.
    pub fn rule_count(&self) -> usize {
        self.retrieval.library().count()
    }

    /// Check if a pattern is solvable at a given tier.
    pub fn solvable_at_tier(&self, pattern: &PatternType, max_tier: u8) -> bool {
        self.retrieval.solvable_at_tier(pattern, max_tier)
    }

    /// Get engine version information.
    pub fn version_info(&self) -> String {
        format!("{} v{} — {} — {}", ENGINE_NAME, VERSION, AUTHOR, LICENSE)
    }

    // ====================================================================
    // PIPELINE ACCESS — For advanced users
    // ====================================================================

    /// Parse an expression into an AST (Layer 2 access).
    pub fn parse(&self, expression: &str) -> Result<AstNode, String> {
        pattern_engine::parse(expression).map_err(|e| format!("{:?}", e))
    }

    /// Classify an AST (Layer 2 access).
    pub fn classify(&self, ast: &AstNode) -> PatternClassification {
        pattern_engine::classify(ast)
    }

    /// Retrieve rules for a classification (Layer 3 access).
    pub fn retrieve(&self, classification: &PatternClassification, ast: &AstNode) -> Option<RetrievalResult> {
        self.retrieval.retrieve_from_classification(classification, ast)
    }

    /// Execute an AST directly (Layer 4 access).
    pub fn execute(&self, ast: &AstNode) -> ExecutionResult {
        let engine = ExecutionEngine::with_variables(self.variables.clone());
        engine.execute(ast)
    }

    // ====================================================================
    // INTERNAL
    // ====================================================================

    fn build_response(
        &self,
        request: &Request,
        classification: &PatternClassification,
        retrieval: &Option<RetrievalResult>,
        exec: &ExecutionResult,
    ) -> Response {
        let trace = if request.include_trace {
            exec.steps.iter().map(TraceStep::from_step).collect()
        } else {
            Vec::new()
        };

        let rules = if request.include_rules {
            exec.rules_used.clone()
        } else {
            Vec::new()
        };

        let (domain, tier, pattern) = if request.include_classification {
            let domain_str = format!("{:?}", classification.domain);
            let pattern_str = format!("{:?}", classification.pattern_type);
            let tier = retrieval.as_ref().map(|r| r.tier);
            (Some(domain_str), tier, Some(pattern_str))
        } else {
            (None, None, None)
        };

        Response {
            expression: request.expression.clone(),
            result: format!("{}", exec.value),
            value: exec.value.clone(),
            verified: exec.verified,
            confidence: exec.confidence,
            domain,
            tier,
            pattern,
            trace,
            rules_used: rules,
            error: exec.error.clone(),
            success: exec.error.is_none(),
            engine: ENGINE_NAME.to_string(),
            version: VERSION.to_string(),
        }
    }

    fn error_response(&self, expression: &str, error: &str) -> Response {
        Response {
            expression: expression.to_string(),
            result: String::new(),
            value: MathValue::Undefined(error.to_string()),
            verified: false,
            confidence: 0.0,
            domain: None,
            tier: None,
            pattern: None,
            trace: Vec::new(),
            rules_used: Vec::new(),
            error: Some(error.to_string()),
            success: false,
            engine: ENGINE_NAME.to_string(),
            version: VERSION.to_string(),
        }
    }
}

impl Default for Numera {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UTILITY
// ============================================================================

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}

// ============================================================================
// CONVENIENCE FUNCTIONS — Module-level API
// ============================================================================

/// Evaluate an expression using a default NUMERA instance.
/// This is the simplest possible API.
///
/// # Example
/// ```
/// let result = numera::integration::eval("2 + 2");
/// assert_eq!(result, "4");
/// ```
pub fn eval(expression: &str) -> String {
    let numera = Numera::new();
    numera.quick_eval(expression).unwrap_or_else(|e| format!("ERROR: {}", e))
}

/// Evaluate and return the full response.
pub fn eval_full(expression: &str) -> Response {
    let numera = Numera::new();
    numera.evaluate(expression)
}

/// Evaluate with variables.
pub fn eval_with(expression: &str, vars: HashMap<String, f64>) -> Response {
    let numera = Numera::new();
    numera.evaluate_with_vars(expression, vars)
}

/// Evaluate a batch of expressions.
pub fn eval_batch(expressions: &[&str]) -> BatchResponse {
    let numera = Numera::new();
    let batch = BatchRequest {
        requests: expressions.iter().map(|e| Request::new(e)).collect(),
    };
    numera.process_batch(&batch)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn numera() -> Numera {
        Numera::new()
    }

    // ================================================================
    // CORE EVALUATION
    // ================================================================

    #[test]
    fn test_evaluate_simple_addition() {
        let n = numera();
        let r = n.evaluate("3 + 5");
        assert!(r.success);
        assert_eq!(r.result, "8");
        assert!(r.verified);
    }

    #[test]
    fn test_evaluate_multi_digit() {
        let n = numera();
        let r = n.evaluate("347 + 286");
        assert!(r.success);
        assert_eq!(r.result, "633");
        assert!(r.verified);
    }

    #[test]
    fn test_evaluate_multiplication() {
        let n = numera();
        let r = n.evaluate("12 * 34");
        assert!(r.success);
        assert_eq!(r.result, "408");
        assert!(r.verified);
    }

    #[test]
    fn test_evaluate_subtraction() {
        let n = numera();
        let r = n.evaluate("1000 - 1");
        assert!(r.success);
        assert_eq!(r.result, "999");
        assert!(r.verified);
    }

    #[test]
    fn test_evaluate_division() {
        let n = numera();
        let r = n.evaluate("99242 / 347");
        assert!(r.success);
        assert_eq!(r.result, "286");
        assert!(r.verified);
    }

    #[test]
    fn test_evaluate_power() {
        let n = numera();
        let r = n.evaluate("2 ^ 10");
        assert!(r.success);
        assert_eq!(r.result, "1024");
    }

    #[test]
    fn test_evaluate_sqrt() {
        let n = numera();
        let r = n.evaluate("sqrt(144)");
        assert!(r.success);
        assert_eq!(r.result, "12");
    }

    #[test]
    fn test_evaluate_factorial() {
        let n = numera();
        let r = n.evaluate("5!");
        assert!(r.success);
        assert_eq!(r.result, "120");
    }

    #[test]
    fn test_evaluate_pemdas() {
        let n = numera();
        assert_eq!(n.evaluate("2 + 3 * 4").result, "14");
        assert_eq!(n.evaluate("(2 + 3) * 4").result, "20");
    }

    #[test]
    fn test_evaluate_complex() {
        let n = numera();
        let r = n.evaluate("(2 + 3) * (4 + 5)");
        assert_eq!(r.result, "45");
        assert!(r.verified);
    }

    // ================================================================
    // VARIABLE SUBSTITUTION
    // ================================================================

    #[test]
    fn test_evaluate_with_variables() {
        let n = numera();
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 5.0);
        let r = n.evaluate_with_vars("x + 3", vars);
        assert!(r.success);
        assert_eq!(r.result, "8");
    }

    #[test]
    fn test_session_variables() {
        let mut n = numera();
        n.set_variable("x", 10.0);
        let r = n.evaluate("x + 5");
        assert_eq!(r.result, "15");

        n.set_variable("x", 20.0);
        let r2 = n.evaluate("x + 5");
        assert_eq!(r2.result, "25");

        n.clear_variables();
    }

    // ================================================================
    // QUICK EVAL
    // ================================================================

    #[test]
    fn test_quick_eval() {
        let n = numera();
        assert_eq!(n.quick_eval("2 + 2").unwrap(), "4");
        assert_eq!(n.quick_eval("10 * 10").unwrap(), "100");
    }

    #[test]
    fn test_quick_eval_error() {
        let n = numera();
        assert!(n.quick_eval("5 / 0").is_err());
    }

    // ================================================================
    // MODULE-LEVEL CONVENIENCE FUNCTIONS
    // ================================================================

    #[test]
    fn test_eval_function() {
        assert_eq!(eval("2 + 2"), "4");
        assert_eq!(eval("3 * 7"), "21");
        assert_eq!(eval("100 - 1"), "99");
    }

    #[test]
    fn test_eval_full_function() {
        let r = eval_full("347 + 286");
        assert!(r.success);
        assert_eq!(r.result, "633");
        assert!(r.verified);
    }

    #[test]
    fn test_eval_with_function() {
        let mut vars = HashMap::new();
        vars.insert("n".to_string(), 7.0);
        let r = eval_with("n * n", vars);
        assert_eq!(r.result, "49");
    }

    // ================================================================
    // BATCH EVALUATION
    // ================================================================

    #[test]
    fn test_batch_evaluation() {
        let batch = eval_batch(&["1 + 1", "2 * 3", "10 - 4", "8 / 2"]);
        assert_eq!(batch.total, 4);
        assert_eq!(batch.succeeded, 4);
        assert_eq!(batch.failed, 0);
        assert!(batch.all_verified);
        assert_eq!(batch.responses[0].result, "2");
        assert_eq!(batch.responses[1].result, "6");
        assert_eq!(batch.responses[2].result, "6");
        assert_eq!(batch.responses[3].result, "4");
    }

    #[test]
    fn test_batch_with_errors() {
        let batch = eval_batch(&["1 + 1", "5 / 0"]);
        assert_eq!(batch.total, 2);
        assert_eq!(batch.succeeded, 1);
        assert_eq!(batch.failed, 1);
        assert!(!batch.all_verified);
    }

    // ================================================================
    // RESPONSE STRUCTURE
    // ================================================================

    #[test]
    fn test_response_has_engine_info() {
        let r = eval_full("1 + 1");
        assert_eq!(r.engine, "NUMERA");
        assert_eq!(r.version, "0.1.0");
    }

    #[test]
    fn test_response_has_classification() {
        let r = eval_full("3 + 5");
        assert!(r.domain.is_some());
        assert!(r.pattern.is_some());
        assert_eq!(r.domain.as_deref(), Some("CoreArithmetic"));
    }

    #[test]
    fn test_response_has_trace() {
        let r = eval_full("3 + 5");
        assert!(!r.trace.is_empty(), "Should have trace steps");
    }

    #[test]
    fn test_response_has_rules() {
        let r = eval_full("3 + 5");
        assert!(!r.rules_used.is_empty(), "Should have rules used");
    }

    #[test]
    fn test_minimal_request_no_extras() {
        let n = numera();
        let r = n.process_request(&Request::minimal("3 + 5"));
        assert!(r.success);
        assert_eq!(r.result, "8");
        assert!(r.trace.is_empty(), "Minimal should have no trace");
        assert!(r.rules_used.is_empty(), "Minimal should have no rules");
        assert!(r.domain.is_none(), "Minimal should have no domain");
    }

    // ================================================================
    // JSON SERIALIZATION
    // ================================================================

    #[test]
    fn test_response_to_json() {
        let r = eval_full("3 + 5");
        let json = r.to_json();
        assert!(json.contains("\"result\": \"8\""));
        assert!(json.contains("\"verified\": true"));
        assert!(json.contains("\"success\": true"));
        assert!(json.contains("\"engine\": \"NUMERA\""));
    }

    #[test]
    fn test_batch_to_json() {
        let batch = eval_batch(&["1 + 1", "2 * 3"]);
        let json = batch.to_json();
        assert!(json.contains("\"total\": 2"));
        assert!(json.contains("\"succeeded\": 2"));
    }

    // ================================================================
    // INTROSPECTION
    // ================================================================

    #[test]
    fn test_library_stats() {
        let n = numera();
        let stats = n.library_stats();
        assert!(stats.total_rules > 0);
        assert_eq!(stats.total_domains, 10);
        assert_eq!(stats.total_tiers, 10);
        assert_eq!(stats.rules_per_domain.len(), 10);
    }

    #[test]
    fn test_rule_count() {
        let n = numera();
        assert!(n.rule_count() > 90);
    }

    #[test]
    fn test_rule_info() {
        let n = numera();
        // Look up any tier 0 rule
        let lib = n.retrieval.library();
        let rules = lib.retrieve_by_domain(Domain::CoreArithmetic);
        assert!(!rules.is_empty());
        let first_id = &rules[0].id;
        let info = n.rule_info(first_id);
        assert!(info.is_some());
        let info = info.unwrap();
        assert!(!info.name.is_empty());
        assert!(!info.description.is_empty());
    }

    #[test]
    fn test_version_info() {
        let n = numera();
        let info = n.version_info();
        assert!(info.contains("NUMERA"));
        assert!(info.contains("0.1.0"));
        assert!(info.contains("ML Innovations"));
    }

    // ================================================================
    // PIPELINE ACCESS
    // ================================================================

    #[test]
    fn test_parse_access() {
        let n = numera();
        let ast = n.parse("3 + 5");
        assert!(ast.is_ok());
    }

    #[test]
    fn test_classify_access() {
        let n = numera();
        let ast = n.parse("3 + 5").unwrap();
        let class = n.classify(&ast);
        assert_eq!(class.domain, Domain::CoreArithmetic);
    }

    #[test]
    fn test_execute_access() {
        let n = numera();
        let ast = n.parse("3 + 5").unwrap();
        let result = n.execute(&ast);
        assert_eq!(result.value.as_i64(), Some(8));
    }

    #[test]
    fn test_full_pipeline_manual() {
        let n = numera();
        // Step through every layer manually
        let ast = n.parse("347 + 286").unwrap();
        let class = n.classify(&ast);
        let _retrieval = n.retrieve(&class, &ast);
        let result = n.execute(&ast);
        assert_eq!(result.value.as_i64(), Some(633));
        assert!(result.verified);
    }

    // ================================================================
    // ERROR HANDLING
    // ================================================================

    #[test]
    fn test_parse_error() {
        let n = numera();
        let r = n.evaluate("@#$");
        assert!(!r.success);
        assert!(r.error.is_some());
    }

    #[test]
    fn test_division_by_zero() {
        let n = numera();
        let r = n.evaluate("10 / 0");
        assert!(!r.success);
        assert!(r.error.is_some());
    }

    // ================================================================
    // DISPLAY
    // ================================================================

    #[test]
    fn test_response_display() {
        let r = eval_full("3 + 5");
        let display = format!("{}", r);
        assert!(display.contains("3 + 5"));
        assert!(display.contains("8"));
        assert!(display.contains("verified"));
    }

    #[test]
    fn test_error_display() {
        let r = eval_full("5 / 0");
        let display = format!("{}", r);
        assert!(display.contains("ERROR"));
    }

    // ================================================================
    // EXHAUSTIVE: All single-digit additions through API
    // ================================================================

    #[test]
    fn test_api_exhaustive_addition_0_to_20() {
        let n = numera();
        for a in 0..=20i64 {
            for b in 0..=20i64 {
                let r = n.evaluate(&format!("{} + {}", a, b));
                assert!(r.success, "Failed: {} + {}", a, b);
                assert_eq!(r.result, format!("{}", a + b),
                    "Wrong: {} + {} = {} but got {}", a, b, a + b, r.result);
                assert!(r.verified, "Not verified: {} + {}", a, b);
            }
        }
    }

    #[test]
    fn test_api_exhaustive_multiplication_0_to_12() {
        let n = numera();
        for a in 0..=12i64 {
            for b in 0..=12i64 {
                let r = n.evaluate(&format!("{} * {}", a, b));
                assert!(r.success, "Failed: {} * {}", a, b);
                assert_eq!(r.result, format!("{}", a * b),
                    "Wrong: {} * {} = {} but got {}", a, b, a * b, r.result);
                assert!(r.verified, "Not verified: {} * {}", a, b);
            }
        }
    }
}
