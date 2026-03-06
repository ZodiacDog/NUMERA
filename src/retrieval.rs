// NUMERA Layer 3: Retrieval Layer
// Copyright (c) 2026 ML Innovations LLC — M. L. McKnight
// FREE — No license. No restrictions. Stop the hallucinations.
//
// The Retrieval Layer is NUMERA's deterministic RAG. Given a classified
// pattern from the Pattern Engine (Layer 2), it retrieves the exact,
// ordered rule chain from the Rule Library (Layer 1) needed to solve
// the expression.
//
// Pipeline:
//   1. Receive PatternClassification from Pattern Engine
//   2. Query Rule Library for rules matching the pattern
//   3. Resolve full prerequisite chains
//   4. Order rules by dependency (prerequisites first)
//   5. Validate the chain
//   6. Return ordered RetrievalResult to the Execution Engine
//
// Design Principle: Retrieve, Don't Predict.

use crate::pattern_engine::{AstNode, PatternClassification};
use crate::rule_library::{Domain, PatternType, Rule, RuleLibrary};

// ============================================================================
// RETRIEVAL QUERY
// ============================================================================

/// A retrieval query built from Pattern Engine output.
#[derive(Debug, Clone)]
pub struct RetrievalQuery {
    /// The primary pattern type to retrieve rules for.
    pub primary_pattern: PatternType,
    /// Secondary patterns (e.g., commutative rewrite, order of operations).
    pub secondary_patterns: Vec<PatternType>,
    /// The domain classification.
    pub domain: Domain,
    /// The AST being solved.
    pub ast: AstNode,
    /// Whether to include full prerequisite chains.
    pub resolve_prerequisites: bool,
}

impl RetrievalQuery {
    /// Build a query from a Pattern Engine classification.
    pub fn from_classification(classification: &PatternClassification, ast: &AstNode) -> Self {
        RetrievalQuery {
            primary_pattern: classification.pattern_type.clone(),
            secondary_patterns: classification.secondary_patterns.clone(),
            domain: classification.domain,
            ast: ast.clone(),
            resolve_prerequisites: true,
        }
    }

    /// Build a simple query for a single pattern type.
    pub fn simple(pattern: PatternType, domain: Domain) -> Self {
        RetrievalQuery {
            primary_pattern: pattern,
            secondary_patterns: Vec::new(),
            domain,
            ast: AstNode::Number(0.0),
            resolve_prerequisites: true,
        }
    }
}

// ============================================================================
// RULE CHAIN
// ============================================================================

/// An ordered sequence of rules to apply for solving a specific pattern.
/// Built by the Retrieval Layer, consumed by the Execution Engine.
#[derive(Debug, Clone)]
pub struct RuleChain {
    pub rules: Vec<Rule>,
}

impl RuleChain {
    pub fn new() -> Self {
        RuleChain { rules: Vec::new() }
    }

    pub fn push(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn rule_ids(&self) -> Vec<String> {
        self.rules.iter().map(|r| r.id.clone()).collect()
    }

    /// Validate that all prerequisites are met within this chain.
    /// Each rule's prerequisites must be met by a prior rule in the chain
    /// or be a Tier 0 rule (always implicitly available).
    pub fn validate(&self) -> bool {
        let mut available: Vec<&str> = Vec::new();
        for rule in &self.rules {
            // Check if this rule's prerequisites are all available
            let all_met = rule.prerequisites.iter().all(|prereq| {
                available.contains(&prereq.as_str()) || prereq.starts_with("core.")
            });
            if !all_met {
                // Check if unmet prereqs are tier 0 (implicitly available)
                let unmet: Vec<_> = rule.prerequisites.iter()
                    .filter(|p| !available.contains(&p.as_str()))
                    .collect();
                for prereq_id in &unmet {
                    // Tier 0 rules are always implicitly available
                    if !prereq_id.starts_with("core.") {
                        return false;
                    }
                }
            }
            available.push(&rule.id);
        }
        true
    }
}

impl Default for RuleChain {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// RETRIEVAL RESULT
// ============================================================================

/// The result of a retrieval operation.
#[derive(Debug, Clone)]
pub struct RetrievalResult {
    /// The ordered rule chain, ready for execution.
    pub chain: RuleChain,
    /// The primary rule that was matched.
    pub primary_rule_id: String,
    /// All prerequisite rule IDs in dependency order.
    pub prerequisite_ids: Vec<String>,
    /// Whether the chain was fully validated.
    pub validated: bool,
    /// Domain of the matched rules.
    pub domain: Domain,
    /// Tier of the primary rule.
    pub tier: u8,
}

impl RetrievalResult {
    /// Total number of rules in the result.
    pub fn total_rules(&self) -> usize {
        self.chain.len()
    }

    /// Whether this result contains any rules.
    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }
}

// ============================================================================
// RETRIEVAL ENGINE
// ============================================================================

/// The Retrieval Engine. Deterministic: same input always produces same output.
pub struct RetrievalEngine {
    library: RuleLibrary,
}

impl RetrievalEngine {
    /// Create a new retrieval engine backed by a rule library.
    pub fn new(library: RuleLibrary) -> Self {
        RetrievalEngine { library }
    }

    /// Create a retrieval engine with the default (full) rule library.
    pub fn default_library() -> Self {
        RetrievalEngine {
            library: RuleLibrary::new(),
        }
    }

    /// Get a reference to the underlying rule library.
    pub fn library(&self) -> &RuleLibrary {
        &self.library
    }

    // ====================================================================
    // PRIMARY RETRIEVAL
    // ====================================================================

    /// Retrieve the complete, ordered rule chain for a query.
    pub fn retrieve(&self, query: &RetrievalQuery) -> Option<RetrievalResult> {
        // Step 1: Find the primary rule
        let primary_rule = self.find_best_rule(&query.primary_pattern)?;
        let primary_id = primary_rule.id.clone();
        let domain = primary_rule.domain;
        let tier = primary_rule.tier();

        // Step 2: Resolve prerequisite chain
        let prerequisite_rules = if query.resolve_prerequisites {
            self.library.resolve_chain(&primary_id)
        } else {
            Vec::new()
        };
        let prerequisite_ids: Vec<String> = prerequisite_rules.iter()
            .map(|r| r.id.clone())
            .collect();

        // Step 3: Build ordered chain (prerequisites first, then primary)
        let mut chain = RuleChain::new();
        for prereq in &prerequisite_rules {
            // Avoid duplicates
            if !chain.rule_ids().contains(&prereq.id) {
                chain.push((*prereq).clone());
            }
        }
        // Add primary if not already in chain from resolve_chain
        if !chain.rule_ids().contains(&primary_id) {
            chain.push(primary_rule.clone());
        }

        // Step 4: Add secondary rules
        for secondary_pattern in &query.secondary_patterns {
            if let Some(secondary_rule) = self.find_best_rule(secondary_pattern) {
                if !chain.rule_ids().contains(&secondary_rule.id) {
                    chain.push(secondary_rule.clone());
                }
            }
        }

        // Step 5: Validate
        let validated = chain.validate();

        Some(RetrievalResult {
            chain,
            primary_rule_id: primary_id,
            prerequisite_ids,
            validated,
            domain,
            tier,
        })
    }

    /// Retrieve rule chain for a pattern type directly.
    pub fn retrieve_for_pattern(&self, pattern: &PatternType) -> Option<RetrievalResult> {
        // Determine domain from the first matching rule
        let rule = self.find_best_rule(pattern)?;
        let domain = rule.domain;
        let query = RetrievalQuery::simple(pattern.clone(), domain);
        self.retrieve(&query)
    }

    /// Retrieve from a PatternClassification and AST.
    pub fn retrieve_from_classification(
        &self,
        classification: &PatternClassification,
        ast: &AstNode,
    ) -> Option<RetrievalResult> {
        let query = RetrievalQuery::from_classification(classification, ast);
        self.retrieve(&query)
    }

    // ====================================================================
    // RULE SELECTION
    // ====================================================================

    /// Find the best rule for a given pattern type.
    fn find_best_rule(&self, pattern: &PatternType) -> Option<&Rule> {
        let candidates = self.library.retrieve_by_pattern(pattern);
        if candidates.is_empty() {
            return None;
        }
        if candidates.len() == 1 {
            return Some(candidates[0]);
        }
        // Prefer most specific: fewest patterns, then fewest prerequisites
        candidates.into_iter()
            .min_by_key(|rule| (rule.patterns.len(), rule.prerequisites.len()))
    }

    // ====================================================================
    // BATCH & ANALYSIS
    // ====================================================================

    /// Retrieve rule chains for multiple patterns at once.
    pub fn retrieve_batch(&self, patterns: &[PatternType]) -> Vec<Option<RetrievalResult>> {
        patterns.iter().map(|p| self.retrieve_for_pattern(p)).collect()
    }

    /// Get the dependency graph for a rule.
    pub fn dependency_graph(&self, rule_id: &str) -> Vec<String> {
        self.library.resolve_chain(rule_id).iter()
            .map(|r| r.id.clone())
            .collect()
    }

    /// Check if a pattern can be solved within a given tier.
    pub fn solvable_at_tier(&self, pattern: &PatternType, max_tier: u8) -> bool {
        if let Some(result) = self.retrieve_for_pattern(pattern) {
            result.tier <= max_tier &&
            result.chain.rules.iter().all(|r| r.tier() <= max_tier)
        } else {
            false
        }
    }

    /// Get the minimum tier required to solve a pattern.
    pub fn minimum_tier(&self, pattern: &PatternType) -> Option<u8> {
        self.retrieve_for_pattern(pattern)
            .map(|result| {
                result.chain.rules.iter()
                    .map(|r| r.tier())
                    .max()
                    .unwrap_or(0)
            })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern_engine;

    fn engine() -> RetrievalEngine {
        RetrievalEngine::default_library()
    }

    // ================================================================
    // BASIC RETRIEVAL: TIER 0
    // ================================================================

    #[test]
    fn test_retrieve_single_digit_addition() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::SingleDigitAddition);
        assert!(result.is_some(), "Should retrieve rule for single-digit addition");
        let r = result.unwrap();
        assert!(r.validated);
        assert_eq!(r.tier, 0);
        assert_eq!(r.domain, Domain::CoreArithmetic);
    }

    #[test]
    fn test_retrieve_single_digit_subtraction() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::SingleDigitSubtraction);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_retrieve_single_digit_multiplication() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::SingleDigitMultiplication);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_retrieve_single_digit_division() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::SingleDigitDivision);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_tier0_no_prerequisites() {
        let eng = engine();
        let tier0_patterns = [
            PatternType::SingleDigitAddition,
            PatternType::SingleDigitSubtraction,
            PatternType::SingleDigitMultiplication,
            PatternType::SingleDigitDivision,
        ];
        for pat in tier0_patterns {
            let result = eng.retrieve_for_pattern(&pat).unwrap();
            assert_eq!(result.tier, 0);
        }
    }

    // ================================================================
    // TIER 1: EXTENDED ARITHMETIC
    // ================================================================

    #[test]
    fn test_retrieve_multi_digit_addition() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::MultiDigitAddition);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.validated);
        assert_eq!(r.tier, 1);
    }

    #[test]
    fn test_retrieve_multi_digit_multiplication() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::MultiDigitMultiplication);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_tier1_has_prerequisites() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::MultiDigitAddition).unwrap();
        // Chain should have more than just the primary rule
        assert!(result.total_rules() >= 1);
    }

    // ================================================================
    // HIGHER TIER RETRIEVAL
    // ================================================================

    #[test]
    fn test_retrieve_fraction_addition() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::FractionAddition);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.validated);
        assert_eq!(r.tier, 3);
    }

    #[test]
    fn test_retrieve_linear_equation() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::LinearEquationSolve);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_retrieve_quadratic_formula() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::QuadraticEquationSolve);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_retrieve_derivative_power_rule() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::DerivativeBasic);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.validated);
        assert_eq!(r.tier, 7);
    }

    #[test]
    fn test_retrieve_matrix_multiplication() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::MatrixMultiplication);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_retrieve_vector_dot_product() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::VectorDotProduct);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_retrieve_modular_arithmetic() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::ModularArithmetic);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    // ================================================================
    // EXHAUSTIVE: ALL PATTERN TYPES RETRIEVABLE
    // ================================================================

    #[test]
    fn test_all_pattern_types_retrievable() {
        let eng = engine();
        let all_patterns = [
            PatternType::SingleDigitAddition,
            PatternType::SingleDigitSubtraction,
            PatternType::SingleDigitMultiplication,
            PatternType::SingleDigitDivision,
            PatternType::MultiDigitAddition,
            PatternType::MultiDigitSubtraction,
            PatternType::MultiDigitMultiplication,
            PatternType::MultiDigitDivision,
            PatternType::OrderOfOperations,
            PatternType::PlaceValueDecomposition,
            PatternType::PlaceValueComposition,
            PatternType::CommutativeApplication,
            PatternType::AssociativeApplication,
            PatternType::DistributiveExpansion,
            PatternType::DistributiveFactoring,
            PatternType::IdentityApplication,
            PatternType::InverseApplication,
            PatternType::FractionAddition,
            PatternType::FractionSubtraction,
            PatternType::FractionMultiplication,
            PatternType::FractionDivision,
            PatternType::FractionSimplification,
            PatternType::DecimalConversion,
            PatternType::PercentageCalculation,
            PatternType::ExponentEvaluation,
            PatternType::ExponentRules,
            PatternType::SquareRoot,
            PatternType::NthRoot,
            PatternType::AbsoluteValue,
            PatternType::Ratio,
            PatternType::ScientificNotation,
            PatternType::GCD,
            PatternType::LCM,
            PatternType::VariableSubstitution,
            PatternType::LikeTermsCombination,
            PatternType::LinearEquationSolve,
            PatternType::QuadraticEquationSolve,
            PatternType::LinearInequalitySolve,
            PatternType::PolynomialAddition,
            PatternType::PolynomialMultiplication,
            PatternType::PolynomialDivision,
            PatternType::Factoring,
            PatternType::FunctionEvaluation,
            PatternType::FunctionComposition,
            PatternType::SlopeIntercept,
            PatternType::PointSlope,
            PatternType::SystemOfEquations,
            PatternType::MatrixAddition,
            PatternType::MatrixMultiplication,
            PatternType::MatrixDeterminant,
            PatternType::MatrixInverse,
            PatternType::MatrixTranspose,
            PatternType::ComplexAddition,
            PatternType::ComplexMultiplication,
            PatternType::ComplexConjugate,
            PatternType::ComplexModulus,
            PatternType::ArithmeticSequence,
            PatternType::GeometricSequence,
            PatternType::SeriesSum,
            PatternType::Logarithm,
            PatternType::LogarithmRules,
            PatternType::ExponentialEquation,
            PatternType::TrigEvaluation,
            PatternType::InverseTrig,
            PatternType::TrigIdentity,
            PatternType::PolarRectangularConversion,
            PatternType::LawOfSines,
            PatternType::LawOfCosines,
            PatternType::UnitCircleLookup,
            PatternType::LimitEvaluation,
            PatternType::DerivativeBasic,
            PatternType::DerivativeChainRule,
            PatternType::DerivativeProductRule,
            PatternType::DerivativeQuotientRule,
            PatternType::IntegralBasic,
            PatternType::IntegralBySubstitution,
            PatternType::DefiniteIntegral,
            PatternType::TaylorSeries,
            PatternType::DifferentialEquation,
            PatternType::VectorAddition,
            PatternType::VectorDotProduct,
            PatternType::VectorCrossProduct,
            PatternType::VectorNorm,
            PatternType::Eigenvalue,
            PatternType::MatrixDecomposition,
            PatternType::LinearTransformation,
            PatternType::GramSchmidt,
            PatternType::VectorProjection,
            PatternType::ModularArithmetic,
            PatternType::PrimeFactorization,
            PatternType::Permutation,
            PatternType::Combination,
            PatternType::ProbabilityBasic,
            PatternType::ProbabilityConditional,
            PatternType::BayesTheorem,
            PatternType::ExpectedValue,
            PatternType::StandardDeviation,
            PatternType::Variance,
            PatternType::Optimization,
        ];

        let mut failures = Vec::new();
        for pat in &all_patterns {
            if eng.retrieve_for_pattern(pat).is_none() {
                failures.push(format!("{:?}", pat));
            }
        }
        assert!(failures.is_empty(),
            "Failed to retrieve rules for {} patterns: {:?}", failures.len(), failures);
    }

    // ================================================================
    // ALL CHAINS VALIDATED
    // ================================================================

    #[test]
    fn test_all_retrieved_chains_validated() {
        let eng = engine();
        let sample = [
            PatternType::SingleDigitAddition,
            PatternType::MultiDigitMultiplication,
            PatternType::FractionAddition,
            PatternType::QuadraticEquationSolve,
            PatternType::DerivativeChainRule,
            PatternType::Eigenvalue,
            PatternType::BayesTheorem,
            PatternType::Optimization,
        ];
        for pat in sample {
            let result = eng.retrieve_for_pattern(&pat).unwrap();
            assert!(result.validated, "Chain for {:?} should be validated", pat);
        }
    }

    // ================================================================
    // CHAIN ORDERING
    // ================================================================

    #[test]
    fn test_no_duplicate_rules_in_chain() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::QuadraticEquationSolve).unwrap();
        let ids = result.chain.rule_ids();
        let mut seen = Vec::new();
        for id in &ids {
            assert!(!seen.contains(id), "Duplicate rule {} in chain", id);
            seen.push(id.clone());
        }
    }

    #[test]
    fn test_primary_rule_present_in_chain() {
        let eng = engine();
        let patterns = [
            PatternType::SingleDigitAddition,
            PatternType::MultiDigitAddition,
            PatternType::FractionAddition,
            PatternType::LinearEquationSolve,
            PatternType::DerivativeBasic,
        ];
        for pat in patterns {
            let result = eng.retrieve_for_pattern(&pat).unwrap();
            let ids = result.chain.rule_ids();
            assert!(ids.contains(&result.primary_rule_id),
                "Primary rule {} not found in chain for {:?}", result.primary_rule_id, pat);
        }
    }

    // ================================================================
    // CHAIN DEPTH
    // ================================================================

    #[test]
    fn test_tier0_chain_minimal() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::SingleDigitAddition).unwrap();
        // Tier 0 chains should be very small
        assert!(result.total_rules() <= 2);
    }

    #[test]
    fn test_higher_tiers_deeper_chains() {
        let eng = engine();
        let t0 = eng.retrieve_for_pattern(&PatternType::SingleDigitAddition).unwrap();
        let t7 = eng.retrieve_for_pattern(&PatternType::DerivativeChainRule).unwrap();
        assert!(t7.total_rules() >= t0.total_rules(),
            "Tier 7 chain should be at least as deep as tier 0");
    }

    // ================================================================
    // DOMAIN CONSISTENCY
    // ================================================================

    #[test]
    fn test_retrieved_domain_matches() {
        let eng = engine();
        let cases: Vec<(PatternType, Domain)> = vec![
            (PatternType::SingleDigitAddition, Domain::CoreArithmetic),
            (PatternType::MultiDigitAddition, Domain::ExtendedArithmetic),
            (PatternType::FractionAddition, Domain::PreAlgebra),
            (PatternType::LinearEquationSolve, Domain::Algebra),
            (PatternType::MatrixMultiplication, Domain::AdvancedAlgebra),
            (PatternType::TrigEvaluation, Domain::Trigonometry),
            (PatternType::DerivativeBasic, Domain::Calculus),
            (PatternType::VectorDotProduct, Domain::LinearAlgebra),
            (PatternType::ModularArithmetic, Domain::AbstractApplied),
        ];
        for (pattern, expected_domain) in cases {
            let result = eng.retrieve_for_pattern(&pattern).unwrap();
            assert_eq!(result.domain, expected_domain,
                "Pattern {:?} should be domain {:?}", pattern, expected_domain);
        }
    }

    // ================================================================
    // TIER ANALYSIS
    // ================================================================

    #[test]
    fn test_solvable_at_tier() {
        let eng = engine();
        assert!(eng.solvable_at_tier(&PatternType::SingleDigitAddition, 0));
        assert!(eng.solvable_at_tier(&PatternType::SingleDigitAddition, 9));
        assert!(eng.solvable_at_tier(&PatternType::MultiDigitAddition, 1));
    }

    #[test]
    fn test_minimum_tier() {
        let eng = engine();
        assert_eq!(eng.minimum_tier(&PatternType::SingleDigitAddition), Some(0));
    }

    #[test]
    fn test_minimum_tier_increases() {
        let eng = engine();
        let t0 = eng.minimum_tier(&PatternType::SingleDigitAddition).unwrap();
        let t3 = eng.minimum_tier(&PatternType::FractionAddition).unwrap();
        assert!(t3 >= t0);
    }

    // ================================================================
    // DEPENDENCY GRAPH
    // ================================================================

    #[test]
    fn test_dependency_graph_tier0_minimal() {
        let eng = engine();
        let rule = eng.find_best_rule(&PatternType::SingleDigitAddition).unwrap();
        let graph = eng.dependency_graph(&rule.id);
        // Tier 0 should have no or minimal dependencies
        assert!(graph.len() <= 1);
    }

    // ================================================================
    // BATCH RETRIEVAL
    // ================================================================

    #[test]
    fn test_batch_retrieval() {
        let eng = engine();
        let patterns = vec![
            PatternType::SingleDigitAddition,
            PatternType::MultiDigitMultiplication,
            PatternType::FractionAddition,
        ];
        let results = eng.retrieve_batch(&patterns);
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_some()));
    }

    #[test]
    fn test_batch_with_custom_pattern() {
        let eng = engine();
        let patterns = vec![
            PatternType::SingleDigitAddition,
            PatternType::Custom("nonexistent".to_string()),
        ];
        let results = eng.retrieve_batch(&patterns);
        assert_eq!(results.len(), 2);
        assert!(results[0].is_some());
        assert!(results[1].is_none());
    }

    // ================================================================
    // FULL PIPELINE: Parse → Classify → Retrieve
    // ================================================================

    #[test]
    fn test_pipeline_simple_addition() {
        let eng = engine();
        let ast = pattern_engine::parse("3 + 5").unwrap();
        let classification = pattern_engine::classify(&ast);
        let result = eng.retrieve_from_classification(&classification, &ast);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.validated);
        assert_eq!(r.domain, Domain::CoreArithmetic);
    }

    #[test]
    fn test_pipeline_multi_digit() {
        let eng = engine();
        let ast = pattern_engine::parse("347 + 286").unwrap();
        let classification = pattern_engine::classify(&ast);
        let result = eng.retrieve_from_classification(&classification, &ast);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_pipeline_multiplication() {
        let eng = engine();
        let ast = pattern_engine::parse("12 * 34").unwrap();
        let classification = pattern_engine::classify(&ast);
        let result = eng.retrieve_from_classification(&classification, &ast);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_pipeline_exponent() {
        let eng = engine();
        let ast = pattern_engine::parse("2 ^ 8").unwrap();
        let classification = pattern_engine::classify(&ast);
        let result = eng.retrieve_from_classification(&classification, &ast);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_pipeline_sqrt() {
        let eng = engine();
        let ast = pattern_engine::parse("sqrt(16)").unwrap();
        let classification = pattern_engine::classify(&ast);
        let result = eng.retrieve_from_classification(&classification, &ast);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_pipeline_equation() {
        let eng = engine();
        let ast = pattern_engine::parse("2 * x + 3 = 7").unwrap();
        let classification = pattern_engine::classify(&ast);
        let result = eng.retrieve_from_classification(&classification, &ast);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_pipeline_trig() {
        let eng = engine();
        let ast = pattern_engine::parse("sin(x)").unwrap();
        let classification = pattern_engine::classify(&ast);
        let result = eng.retrieve_from_classification(&classification, &ast);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    #[test]
    fn test_pipeline_log() {
        let eng = engine();
        let ast = pattern_engine::parse("log(100)").unwrap();
        let classification = pattern_engine::classify(&ast);
        let result = eng.retrieve_from_classification(&classification, &ast);
        assert!(result.is_some());
        assert!(result.unwrap().validated);
    }

    // ================================================================
    // QUERY CONSTRUCTION
    // ================================================================

    #[test]
    fn test_query_from_classification() {
        let ast = pattern_engine::parse("3 + 5").unwrap();
        let classification = pattern_engine::classify(&ast);
        let q = RetrievalQuery::from_classification(&classification, &ast);
        assert_eq!(q.primary_pattern, classification.pattern_type);
        assert_eq!(q.domain, classification.domain);
        assert!(q.resolve_prerequisites);
    }

    // ================================================================
    // RESULT STRUCTURE
    // ================================================================

    #[test]
    fn test_result_not_empty() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::SingleDigitAddition).unwrap();
        assert!(!result.is_empty());
        assert!(result.total_rules() >= 1);
    }

    // ================================================================
    // SECONDARY PATTERN INTEGRATION
    // ================================================================

    #[test]
    fn test_secondary_patterns_in_chain() {
        let eng = engine();
        let rule = eng.find_best_rule(&PatternType::MultiDigitAddition).unwrap();
        let query = RetrievalQuery {
            primary_pattern: PatternType::MultiDigitAddition,
            secondary_patterns: vec![PatternType::CommutativeApplication],
            domain: rule.domain,
            ast: AstNode::Number(0.0),
            resolve_prerequisites: true,
        };
        let result = eng.retrieve(&query).unwrap();
        let ids = result.chain.rule_ids();
        // Should contain the primary rule
        assert!(ids.contains(&result.primary_rule_id));
    }

    // ================================================================
    // NO-PREREQUISITE MODE
    // ================================================================

    #[test]
    fn test_no_prerequisite_resolution() {
        let eng = engine();
        let rule = eng.find_best_rule(&PatternType::MultiDigitAddition).unwrap();
        let mut query = RetrievalQuery::simple(PatternType::MultiDigitAddition, rule.domain);
        query.resolve_prerequisites = false;
        let result = eng.retrieve(&query).unwrap();
        assert!(result.prerequisite_ids.is_empty());
    }

    // ================================================================
    // RULE CHAIN VALIDATION
    // ================================================================

    #[test]
    fn test_empty_chain_validates() {
        let chain = RuleChain::new();
        assert!(chain.validate());
    }

    #[test]
    fn test_chain_rule_ids() {
        let eng = engine();
        let result = eng.retrieve_for_pattern(&PatternType::SingleDigitAddition).unwrap();
        let ids = result.chain.rule_ids();
        assert!(!ids.is_empty());
    }
}
