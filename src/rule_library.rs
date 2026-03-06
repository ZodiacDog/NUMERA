// NUMERA Layer 1: Rule Library
// Copyright (c) 2026 ML Innovations LLC — M. L. McKnight
// FREE — No license. No restrictions. Stop the hallucinations.
//
// The Rule Library is a structured, retrievable knowledge base of mathematical
// operations. Each rule is stored as an executable definition — not a text
// description, not a pattern to predict, but actual logic that can be applied.
//
// Rules are organized hierarchically by domain and complexity (Tier 0-9).
// No rule can be applied unless all its prerequisites are satisfied.
// Every rule traces its logic back to the ten foundational values.
//
// Design Principle: Foundation First.
// Nothing exists here that can't be grounded in 0-9 and deterministic rules.

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// DOMAIN AND TIER DEFINITIONS
// ============================================================================

/// Mathematical domains, ordered by tier (prerequisite depth).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Domain {
    CoreArithmetic,      // Tier 0: Single-digit +, -, *, /
    ExtendedArithmetic,  // Tier 1: Multi-digit, place value, carry/borrow, order of ops
    Properties,          // Tier 2: Commutativity, associativity, distributive, identity
    PreAlgebra,          // Tier 3: Fractions, decimals, percentages, exponents, roots
    Algebra,             // Tier 4: Variables, equations, inequalities, functions
    AdvancedAlgebra,     // Tier 5: Systems, matrices, complex numbers, sequences
    Trigonometry,        // Tier 6: Trig functions, identities, unit circle
    Calculus,            // Tier 7: Limits, derivatives, integrals, series
    LinearAlgebra,       // Tier 8: Vector spaces, eigenvalues, transformations
    AbstractApplied,     // Tier 9: Number theory, graph theory, statistics, probability
}

impl Domain {
    /// The tier number for this domain.
    pub fn tier(&self) -> u8 {
        match self {
            Domain::CoreArithmetic => 0,
            Domain::ExtendedArithmetic => 1,
            Domain::Properties => 2,
            Domain::PreAlgebra => 3,
            Domain::Algebra => 4,
            Domain::AdvancedAlgebra => 5,
            Domain::Trigonometry => 6,
            Domain::Calculus => 7,
            Domain::LinearAlgebra => 8,
            Domain::AbstractApplied => 9,
        }
    }

    /// All domains in tier order.
    pub fn all() -> &'static [Domain] {
        &[
            Domain::CoreArithmetic,
            Domain::ExtendedArithmetic,
            Domain::Properties,
            Domain::PreAlgebra,
            Domain::Algebra,
            Domain::AdvancedAlgebra,
            Domain::Trigonometry,
            Domain::Calculus,
            Domain::LinearAlgebra,
            Domain::AbstractApplied,
        ]
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Domain::CoreArithmetic => write!(f, "Core Arithmetic"),
            Domain::ExtendedArithmetic => write!(f, "Extended Arithmetic"),
            Domain::Properties => write!(f, "Properties"),
            Domain::PreAlgebra => write!(f, "Pre-Algebra"),
            Domain::Algebra => write!(f, "Algebra"),
            Domain::AdvancedAlgebra => write!(f, "Advanced Algebra"),
            Domain::Trigonometry => write!(f, "Trigonometry"),
            Domain::Calculus => write!(f, "Calculus"),
            Domain::LinearAlgebra => write!(f, "Linear Algebra"),
            Domain::AbstractApplied => write!(f, "Abstract/Applied"),
        }
    }
}

// ============================================================================
// MATHEMATICAL PROPERTIES
// ============================================================================

/// Properties that a rule or operation may possess.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Property {
    Commutative,    // a op b = b op a
    Associative,    // (a op b) op c = a op (b op c)
    Distributive,   // a * (b + c) = a*b + a*c
    HasIdentity,    // There exists e such that a op e = a
    HasInverse,     // For every a, there exists a' such that a op a' = e
    Idempotent,     // a op a = a
    Closure,        // Result stays in the same set
    Transitive,     // If a R b and b R c, then a R c
    Symmetric,      // If a R b, then b R a
    Reflexive,      // a R a
}

// ============================================================================
// PATTERN TYPES — What triggers rule retrieval
// ============================================================================

/// Pattern types that the Pattern Engine (Layer 2) classifies expressions into.
/// The Retrieval Layer (Layer 3) uses these to look up applicable rules.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternType {
    // Tier 0: Core Arithmetic
    SingleDigitAddition,
    SingleDigitSubtraction,
    SingleDigitMultiplication,
    SingleDigitDivision,

    // Tier 1: Extended Arithmetic
    MultiDigitAddition,
    MultiDigitSubtraction,
    MultiDigitMultiplication,
    MultiDigitDivision,
    OrderOfOperations,
    PlaceValueDecomposition,
    PlaceValueComposition,

    // Tier 2: Properties
    CommutativeApplication,
    AssociativeApplication,
    DistributiveExpansion,
    DistributiveFactoring,
    IdentityApplication,
    InverseApplication,

    // Tier 3: Pre-Algebra
    FractionAddition,
    FractionSubtraction,
    FractionMultiplication,
    FractionDivision,
    FractionSimplification,
    DecimalConversion,
    PercentageCalculation,
    ExponentEvaluation,
    ExponentRules,
    SquareRoot,
    NthRoot,
    AbsoluteValue,
    Ratio,
    Proportion,
    ScientificNotation,
    GCD,
    LCM,
    PrimeFactorization,

    // Tier 4: Algebra
    VariableSubstitution,
    LikeTermsCombination,
    LinearEquationSolve,
    LinearInequalitySolve,
    QuadraticEquationSolve,
    PolynomialAddition,
    PolynomialMultiplication,
    PolynomialDivision,
    Factoring,
    FunctionEvaluation,
    FunctionComposition,
    DomainRange,
    SlopeIntercept,
    PointSlope,

    // Tier 5: Advanced Algebra
    SystemOfEquations,
    MatrixAddition,
    MatrixMultiplication,
    MatrixDeterminant,
    MatrixInverse,
    ComplexAddition,
    ComplexMultiplication,
    ComplexConjugate,
    ComplexModulus,
    ArithmeticSequence,
    GeometricSequence,
    SeriesSum,
    Logarithm,
    LogarithmRules,
    ExponentialEquation,

    // Tier 6: Trigonometry
    TrigEvaluation,
    TrigIdentity,
    InverseTrig,
    UnitCircleLookup,
    LawOfSines,
    LawOfCosines,
    TrigEquationSolve,
    PolarRectangularConversion,

    // Tier 7: Calculus
    LimitEvaluation,
    DerivativeBasic,
    DerivativeChainRule,
    DerivativeProductRule,
    DerivativeQuotientRule,
    IntegralBasic,
    IntegralBySubstitution,
    IntegralByParts,
    DefiniteIntegral,
    SeriesConvergence,
    TaylorSeries,
    DifferentialEquation,
    PartialDerivative,
    MultipleIntegral,

    // Tier 8: Linear Algebra
    VectorAddition,
    VectorDotProduct,
    VectorCrossProduct,
    VectorNorm,
    MatrixTranspose,
    Eigenvalue,
    Eigenvector,
    LinearTransformation,
    MatrixDecomposition,
    VectorProjection,
    GramSchmidt,
    RankNullity,

    // Tier 9: Abstract/Applied
    ModularArithmetic,
    Permutation,
    Combination,
    ProbabilityBasic,
    ProbabilityConditional,
    BayesTheorem,
    ExpectedValue,
    Variance,
    StandardDeviation,
    NormalDistribution,
    HypothesisTest,
    GraphAdjacency,
    ShortestPath,
    Optimization,

    // Catch-all for extensibility
    Custom(String),
}

// ============================================================================
// RULE DEFINITION
// ============================================================================

/// A mathematical rule: a retrievable, executable unit of mathematical knowledge.
/// Every rule has prerequisites, a pattern it matches, and an operation it performs.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Unique identifier (e.g., "core.add", "algebra.like_terms")
    pub id: String,
    /// Mathematical domain this rule belongs to
    pub domain: Domain,
    /// Human-readable name
    pub name: String,
    /// Human-readable description of what this rule does
    pub description: String,
    /// Rule IDs that must be available before this rule can apply
    pub prerequisites: Vec<String>,
    /// Pattern types this rule matches
    pub patterns: Vec<PatternType>,
    /// Mathematical properties this rule exhibits
    pub properties: Vec<Property>,
    /// The inverse rule, if one exists (e.g., addition <-> subtraction)
    pub inverse: Option<String>,
    /// Operation type for the execution engine
    pub operation: Operation,
}

impl Rule {
    /// Check if all prerequisites are satisfied given a set of available rule IDs.
    pub fn prerequisites_met(&self, available: &[&str]) -> bool {
        self.prerequisites.iter().all(|prereq| available.contains(&prereq.as_str()))
    }

    /// The tier of this rule (derived from its domain).
    pub fn tier(&self) -> u8 {
        self.domain.tier()
    }
}

// ============================================================================
// OPERATION TYPES — What the execution engine does
// ============================================================================

/// The type of operation a rule performs. These are the executable definitions
/// that the Execution Engine (Layer 4) uses to actually compute results.
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    // ---- Tier 0: Core Arithmetic ----
    /// Add two single-digit values via Value Core lookup
    ValueCoreAdd,
    /// Subtract two single-digit values via Value Core lookup
    ValueCoreSub,
    /// Multiply two single-digit values via Value Core lookup
    ValueCoreMul,
    /// Divide two single-digit values via Value Core lookup
    ValueCoreDiv,

    // ---- Tier 1: Extended Arithmetic ----
    /// Multi-digit addition with carry propagation
    MultiDigitAdd,
    /// Multi-digit subtraction with borrow propagation
    MultiDigitSub,
    /// Multi-digit multiplication (long multiplication)
    MultiDigitMul,
    /// Multi-digit division (long division)
    MultiDigitDiv,
    /// Decompose number into place value components
    Decompose,
    /// Compose number from place value components
    Compose,
    /// Apply order of operations (PEMDAS/BODMAS)
    OrderOfOps,

    // ---- Tier 2: Properties ----
    /// Swap operand order (a + b -> b + a)
    Commute,
    /// Regroup operands ((a+b)+c -> a+(b+c))
    Associate,
    /// Expand distribution (a*(b+c) -> a*b + a*c)
    Distribute,
    /// Factor common terms (a*b + a*c -> a*(b+c))
    Factor,
    /// Apply identity element (a + 0 = a, a * 1 = a)
    ApplyIdentity,
    /// Apply inverse (a + (-a) = 0, a * (1/a) = 1)
    ApplyInverse,

    // ---- Tier 3: Pre-Algebra ----
    /// Find GCD of two numbers
    ComputeGCD,
    /// Find LCM of two numbers
    ComputeLCM,
    /// Find prime factorization
    PrimeFactorize,
    /// Add fractions (find common denominator, add numerators)
    FractionAdd,
    /// Subtract fractions
    FractionSub,
    /// Multiply fractions (multiply numerators, multiply denominators)
    FractionMul,
    /// Divide fractions (multiply by reciprocal)
    FractionDiv,
    /// Simplify fraction by GCD
    FractionSimplify,
    /// Convert between decimal and fraction
    DecimalConvert,
    /// Calculate percentage
    PercentCalc,
    /// Evaluate exponent (base^exp via repeated multiplication)
    ExponentEval,
    /// Apply exponent rules (product rule, quotient rule, power rule)
    ExponentRule,
    /// Calculate square root
    SqrtCalc,
    /// Calculate nth root
    NthRootCalc,
    /// Evaluate absolute value
    AbsoluteVal,
    /// Convert to/from scientific notation
    SciNotation,

    // ---- Tier 4: Algebra ----
    /// Substitute a value for a variable
    Substitute,
    /// Combine like terms (2x + 3x -> 5x)
    CombineLikeTerms,
    /// Solve linear equation (ax + b = c)
    SolveLinear,
    /// Solve linear inequality
    SolveLinearInequality,
    /// Solve quadratic equation (ax^2 + bx + c = 0)
    SolveQuadratic,
    /// Add polynomials
    PolyAdd,
    /// Multiply polynomials (FOIL, distribution)
    PolyMul,
    /// Divide polynomials (long/synthetic division)
    PolyDiv,
    /// Factor polynomial expressions
    PolyFactor,
    /// Evaluate function at a point
    FuncEval,
    /// Compose functions f(g(x))
    FuncCompose,
    /// Determine domain and range
    DomainRangeCalc,
    /// Compute slope from two points
    SlopeCalc,

    // ---- Tier 5: Advanced Algebra ----
    /// Solve system of equations (substitution, elimination, Cramer's)
    SolveSystem,
    /// Matrix addition
    MatAdd,
    /// Matrix multiplication
    MatMul,
    /// Matrix determinant
    MatDet,
    /// Matrix inverse
    MatInv,
    /// Complex number addition
    ComplexAdd,
    /// Complex number multiplication
    ComplexMul,
    /// Complex conjugate
    ComplexConj,
    /// Complex modulus |z|
    ComplexMod,
    /// Evaluate arithmetic sequence term/sum
    ArithSeq,
    /// Evaluate geometric sequence term/sum
    GeomSeq,
    /// Evaluate series sum (finite or convergent infinite)
    SeriesEval,
    /// Evaluate logarithm
    LogEval,
    /// Apply logarithm rules
    LogRule,
    /// Solve exponential equation
    SolveExponential,

    // ---- Tier 6: Trigonometry ----
    /// Evaluate trig function at a value
    TrigEval,
    /// Apply trig identity
    TrigIdentityApply,
    /// Evaluate inverse trig function
    InverseTrigEval,
    /// Look up unit circle value
    UnitCircle,
    /// Apply law of sines
    LawSines,
    /// Apply law of cosines
    LawCosines,
    /// Solve trigonometric equation
    SolveTrigEq,
    /// Convert polar <-> rectangular
    PolarRectConvert,

    // ---- Tier 7: Calculus ----
    /// Evaluate a limit
    LimitEval,
    /// Basic derivative (power rule, constant, etc.)
    DerivBasic,
    /// Chain rule derivative
    DerivChain,
    /// Product rule derivative
    DerivProduct,
    /// Quotient rule derivative
    DerivQuotient,
    /// Basic integral (power rule, etc.)
    IntegralBasic,
    /// Integration by substitution (u-sub)
    IntegralUSub,
    /// Integration by parts
    IntegralParts,
    /// Evaluate definite integral
    DefiniteIntegralEval,
    /// Test series convergence
    SeriesConvergenceTest,
    /// Compute Taylor/Maclaurin series
    TaylorExpand,
    /// Solve differential equation
    SolveDE,
    /// Compute partial derivative
    PartialDeriv,
    /// Compute multiple integral
    MultiIntegral,

    // ---- Tier 8: Linear Algebra ----
    /// Add vectors
    VecAdd,
    /// Dot product
    DotProduct,
    /// Cross product
    CrossProduct,
    /// Vector norm/magnitude
    VecNorm,
    /// Transpose matrix
    Transpose,
    /// Find eigenvalues
    EigenvalueCalc,
    /// Find eigenvectors
    EigenvectorCalc,
    /// Apply linear transformation
    LinearTransform,
    /// Matrix decomposition (LU, QR, SVD)
    MatDecompose,
    /// Project vector onto another
    VecProject,
    /// Gram-Schmidt orthogonalization
    GramSchmidtCalc,
    /// Compute rank and nullity
    RankNullityCalc,

    // ---- Tier 9: Abstract/Applied ----
    /// Modular arithmetic operation
    ModArith,
    /// Count permutations
    PermutationCount,
    /// Count combinations
    CombinationCount,
    /// Basic probability calculation
    ProbCalc,
    /// Conditional probability
    CondProbCalc,
    /// Apply Bayes' theorem
    BayesCalc,
    /// Compute expected value
    ExpectedValCalc,
    /// Compute variance
    VarianceCalc,
    /// Compute standard deviation
    StdDevCalc,
    /// Normal distribution calculations
    NormalDistCalc,
    /// Hypothesis testing
    HypTestCalc,
    /// Build/query adjacency representation
    GraphAdjCalc,
    /// Find shortest path
    ShortestPathCalc,
    /// Optimization (min/max with constraints)
    OptimizeCalc,

    // Extensibility
    Custom(String),
}

// ============================================================================
// RULE LIBRARY — The Knowledge Base
// ============================================================================

/// The Rule Library: a complete, indexed collection of mathematical rules.
/// Indexed by ID, domain, and pattern type for O(1) retrieval.
pub struct RuleLibrary {
    /// All rules, keyed by their unique ID
    rules: HashMap<String, Rule>,
    /// Index: domain -> list of rule IDs in that domain
    domain_index: HashMap<Domain, Vec<String>>,
    /// Index: pattern type -> list of rule IDs matching that pattern
    pattern_index: HashMap<PatternType, Vec<String>>,
}

impl RuleLibrary {
    /// Create a new Rule Library populated with all built-in rules.
    pub fn new() -> Self {
        let mut lib = Self {
            rules: HashMap::new(),
            domain_index: HashMap::new(),
            pattern_index: HashMap::new(),
        };

        // Register all tiers
        lib.register_tier0();
        lib.register_tier1();
        lib.register_tier2();
        lib.register_tier3();
        lib.register_tier4();
        lib.register_tier5();
        lib.register_tier6();
        lib.register_tier7();
        lib.register_tier8();
        lib.register_tier9();

        lib
    }

    /// Register a rule into the library, updating all indexes.
    pub fn register(&mut self, rule: Rule) {
        let id = rule.id.clone();
        let domain = rule.domain;
        let patterns = rule.patterns.clone();

        // Domain index
        self.domain_index
            .entry(domain)
            .or_insert_with(Vec::new)
            .push(id.clone());

        // Pattern index
        for pattern in patterns {
            self.pattern_index
                .entry(pattern)
                .or_insert_with(Vec::new)
                .push(id.clone());
        }

        // Main store
        self.rules.insert(id, rule);
    }

    /// Retrieve a rule by its ID.
    pub fn get(&self, id: &str) -> Option<&Rule> {
        self.rules.get(id)
    }

    /// Retrieve all rules matching a pattern type, ordered by specificity (most specific first).
    pub fn retrieve_by_pattern(&self, pattern: &PatternType) -> Vec<&Rule> {
        self.pattern_index
            .get(pattern)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.rules.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Retrieve all rules in a domain.
    pub fn retrieve_by_domain(&self, domain: Domain) -> Vec<&Rule> {
        self.domain_index
            .get(&domain)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.rules.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Retrieve all rules in a specific tier.
    pub fn retrieve_by_tier(&self, tier: u8) -> Vec<&Rule> {
        self.rules
            .values()
            .filter(|r| r.tier() == tier)
            .collect()
    }

    /// Get all available rule IDs.
    pub fn all_ids(&self) -> Vec<&str> {
        self.rules.keys().map(|s| s.as_str()).collect()
    }

    /// Total number of rules in the library.
    pub fn count(&self) -> usize {
        self.rules.len()
    }

    /// Number of rules in a specific domain.
    pub fn count_domain(&self, domain: Domain) -> usize {
        self.domain_index
            .get(&domain)
            .map(|ids| ids.len())
            .unwrap_or(0)
    }

    /// Check if a rule's prerequisites are all present in the library.
    pub fn prerequisites_satisfied(&self, rule_id: &str) -> bool {
        if let Some(rule) = self.rules.get(rule_id) {
            rule.prerequisites.iter().all(|prereq| self.rules.contains_key(prereq))
        } else {
            false
        }
    }

    /// Resolve the full prerequisite chain for a rule (depth-first).
    /// Returns ordered list: prerequisites first, target rule last.
    pub fn resolve_chain(&self, rule_id: &str) -> Vec<&Rule> {
        let mut chain = Vec::new();
        let mut visited = Vec::new();
        self.resolve_recursive(rule_id, &mut chain, &mut visited);
        chain
    }

    fn resolve_recursive<'a>(
        &'a self,
        rule_id: &str,
        chain: &mut Vec<&'a Rule>,
        visited: &mut Vec<String>,
    ) {
        if visited.contains(&rule_id.to_string()) {
            return; // Already resolved (prevent cycles)
        }
        visited.push(rule_id.to_string());

        if let Some(rule) = self.rules.get(rule_id) {
            // Resolve prerequisites first
            for prereq_id in &rule.prerequisites {
                self.resolve_recursive(prereq_id, chain, visited);
            }
            chain.push(rule);
        }
    }

    // ====================================================================
    // TIER REGISTRATION
    // ====================================================================

    fn register_tier0(&mut self) {
        // TIER 0: Core Arithmetic — Single-digit operations on Value Core

        self.register(Rule {
            id: "core.add".into(),
            domain: Domain::CoreArithmetic,
            name: "Single-Digit Addition".into(),
            description: "Retrieve addition fact from Value Core table. a + b for digits 0-9.".into(),
            prerequisites: vec![],
            patterns: vec![PatternType::SingleDigitAddition],
            properties: vec![Property::Commutative, Property::Associative, Property::HasIdentity],
            inverse: Some("core.sub".into()),
            operation: Operation::ValueCoreAdd,
        });

        self.register(Rule {
            id: "core.sub".into(),
            domain: Domain::CoreArithmetic,
            name: "Single-Digit Subtraction".into(),
            description: "Retrieve subtraction fact from Value Core table. a - b where a >= b.".into(),
            prerequisites: vec![],
            patterns: vec![PatternType::SingleDigitSubtraction],
            properties: vec![Property::HasIdentity],
            inverse: Some("core.add".into()),
            operation: Operation::ValueCoreSub,
        });

        self.register(Rule {
            id: "core.mul".into(),
            domain: Domain::CoreArithmetic,
            name: "Single-Digit Multiplication".into(),
            description: "Retrieve multiplication fact from Value Core table. a * b for digits 0-9.".into(),
            prerequisites: vec![],
            patterns: vec![PatternType::SingleDigitMultiplication],
            properties: vec![Property::Commutative, Property::Associative, Property::HasIdentity],
            inverse: Some("core.div".into()),
            operation: Operation::ValueCoreMul,
        });

        self.register(Rule {
            id: "core.div".into(),
            domain: Domain::CoreArithmetic,
            name: "Single-Digit Division".into(),
            description: "Retrieve division fact from Value Core. a / b where b != 0 and a divisible by b.".into(),
            prerequisites: vec![],
            patterns: vec![PatternType::SingleDigitDivision],
            properties: vec![],
            inverse: Some("core.mul".into()),
            operation: Operation::ValueCoreDiv,
        });
    }

    fn register_tier1(&mut self) {
        // TIER 1: Extended Arithmetic — Multi-digit operations built on Tier 0

        self.register(Rule {
            id: "ext.add".into(),
            domain: Domain::ExtendedArithmetic,
            name: "Multi-Digit Addition".into(),
            description: "Add multi-digit numbers using positional addition with carry propagation. Each digit-pair addition uses core.add.".into(),
            prerequisites: vec!["core.add".into()],
            patterns: vec![PatternType::MultiDigitAddition],
            properties: vec![Property::Commutative, Property::Associative, Property::HasIdentity],
            inverse: Some("ext.sub".into()),
            operation: Operation::MultiDigitAdd,
        });

        self.register(Rule {
            id: "ext.sub".into(),
            domain: Domain::ExtendedArithmetic,
            name: "Multi-Digit Subtraction".into(),
            description: "Subtract multi-digit numbers using positional subtraction with borrow propagation.".into(),
            prerequisites: vec!["core.sub".into(), "core.add".into()],
            patterns: vec![PatternType::MultiDigitSubtraction],
            properties: vec![Property::HasIdentity],
            inverse: Some("ext.add".into()),
            operation: Operation::MultiDigitSub,
        });

        self.register(Rule {
            id: "ext.mul".into(),
            domain: Domain::ExtendedArithmetic,
            name: "Multi-Digit Multiplication".into(),
            description: "Multiply multi-digit numbers using long multiplication. Each partial product uses core.mul and accumulates via ext.add.".into(),
            prerequisites: vec!["core.mul".into(), "ext.add".into()],
            patterns: vec![PatternType::MultiDigitMultiplication],
            properties: vec![Property::Commutative, Property::Associative, Property::HasIdentity],
            inverse: Some("ext.div".into()),
            operation: Operation::MultiDigitMul,
        });

        self.register(Rule {
            id: "ext.div".into(),
            domain: Domain::ExtendedArithmetic,
            name: "Multi-Digit Division".into(),
            description: "Divide multi-digit numbers using long division. Returns quotient and remainder.".into(),
            prerequisites: vec!["core.div".into(), "ext.mul".into(), "ext.sub".into()],
            patterns: vec![PatternType::MultiDigitDivision],
            properties: vec![],
            inverse: Some("ext.mul".into()),
            operation: Operation::MultiDigitDiv,
        });

        self.register(Rule {
            id: "ext.decompose".into(),
            domain: Domain::ExtendedArithmetic,
            name: "Place Value Decomposition".into(),
            description: "Decompose a number into digit-place pairs. 347 = 3*100 + 4*10 + 7*1.".into(),
            prerequisites: vec![],
            patterns: vec![PatternType::PlaceValueDecomposition],
            properties: vec![],
            inverse: Some("ext.compose".into()),
            operation: Operation::Decompose,
        });

        self.register(Rule {
            id: "ext.compose".into(),
            domain: Domain::ExtendedArithmetic,
            name: "Place Value Composition".into(),
            description: "Compose a number from digit-place pairs. Inverse of decomposition.".into(),
            prerequisites: vec!["core.mul".into(), "core.add".into()],
            patterns: vec![PatternType::PlaceValueComposition],
            properties: vec![],
            inverse: Some("ext.decompose".into()),
            operation: Operation::Compose,
        });

        self.register(Rule {
            id: "ext.order_of_ops".into(),
            domain: Domain::ExtendedArithmetic,
            name: "Order of Operations".into(),
            description: "Apply PEMDAS/BODMAS: Parentheses, Exponents, Multiplication/Division (L-R), Addition/Subtraction (L-R).".into(),
            prerequisites: vec!["core.add".into(), "core.sub".into(), "core.mul".into(), "core.div".into()],
            patterns: vec![PatternType::OrderOfOperations],
            properties: vec![],
            inverse: None,
            operation: Operation::OrderOfOps,
        });
    }

    fn register_tier2(&mut self) {
        // TIER 2: Properties — Fundamental mathematical properties

        self.register(Rule {
            id: "prop.commutative".into(),
            domain: Domain::Properties,
            name: "Commutative Property".into(),
            description: "For addition and multiplication: a + b = b + a, a * b = b * a. Order doesn't matter.".into(),
            prerequisites: vec!["core.add".into(), "core.mul".into()],
            patterns: vec![PatternType::CommutativeApplication],
            properties: vec![Property::Commutative],
            inverse: None,
            operation: Operation::Commute,
        });

        self.register(Rule {
            id: "prop.associative".into(),
            domain: Domain::Properties,
            name: "Associative Property".into(),
            description: "For addition and multiplication: (a+b)+c = a+(b+c). Grouping doesn't matter.".into(),
            prerequisites: vec!["core.add".into(), "core.mul".into()],
            patterns: vec![PatternType::AssociativeApplication],
            properties: vec![Property::Associative],
            inverse: None,
            operation: Operation::Associate,
        });

        self.register(Rule {
            id: "prop.distributive".into(),
            domain: Domain::Properties,
            name: "Distributive Property (Expansion)".into(),
            description: "a * (b + c) = a*b + a*c. Multiplication distributes over addition.".into(),
            prerequisites: vec!["core.mul".into(), "core.add".into()],
            patterns: vec![PatternType::DistributiveExpansion],
            properties: vec![Property::Distributive],
            inverse: Some("prop.factor".into()),
            operation: Operation::Distribute,
        });

        self.register(Rule {
            id: "prop.factor".into(),
            domain: Domain::Properties,
            name: "Distributive Property (Factoring)".into(),
            description: "a*b + a*c = a*(b+c). The reverse of distribution — extract common factor.".into(),
            prerequisites: vec!["core.mul".into(), "core.add".into(), "core.div".into()],
            patterns: vec![PatternType::DistributiveFactoring],
            properties: vec![Property::Distributive],
            inverse: Some("prop.distributive".into()),
            operation: Operation::Factor,
        });

        self.register(Rule {
            id: "prop.identity".into(),
            domain: Domain::Properties,
            name: "Identity Elements".into(),
            description: "Additive identity: a + 0 = a. Multiplicative identity: a * 1 = a.".into(),
            prerequisites: vec!["core.add".into(), "core.mul".into()],
            patterns: vec![PatternType::IdentityApplication],
            properties: vec![Property::HasIdentity],
            inverse: None,
            operation: Operation::ApplyIdentity,
        });

        self.register(Rule {
            id: "prop.inverse".into(),
            domain: Domain::Properties,
            name: "Inverse Elements".into(),
            description: "Additive inverse: a + (-a) = 0. Multiplicative inverse: a * (1/a) = 1.".into(),
            prerequisites: vec!["core.add".into(), "core.mul".into(), "core.sub".into(), "core.div".into()],
            patterns: vec![PatternType::InverseApplication],
            properties: vec![Property::HasInverse],
            inverse: None,
            operation: Operation::ApplyInverse,
        });
    }

    fn register_tier3(&mut self) {
        // TIER 3: Pre-Algebra — Fractions, decimals, exponents, roots

        self.register(Rule {
            id: "pre.gcd".into(),
            domain: Domain::PreAlgebra,
            name: "Greatest Common Divisor".into(),
            description: "Find the GCD of two numbers using the Euclidean algorithm.".into(),
            prerequisites: vec!["ext.div".into()],
            patterns: vec![PatternType::GCD],
            properties: vec![Property::Commutative],
            inverse: None,
            operation: Operation::ComputeGCD,
        });

        self.register(Rule {
            id: "pre.lcm".into(),
            domain: Domain::PreAlgebra,
            name: "Least Common Multiple".into(),
            description: "Find LCM via: LCM(a,b) = (a*b) / GCD(a,b).".into(),
            prerequisites: vec!["pre.gcd".into(), "ext.mul".into(), "ext.div".into()],
            patterns: vec![PatternType::LCM],
            properties: vec![Property::Commutative],
            inverse: None,
            operation: Operation::ComputeLCM,
        });

        self.register(Rule {
            id: "pre.prime_factor".into(),
            domain: Domain::PreAlgebra,
            name: "Prime Factorization".into(),
            description: "Decompose a number into its prime factors.".into(),
            prerequisites: vec!["ext.div".into()],
            patterns: vec![PatternType::PrimeFactorization],
            properties: vec![],
            inverse: None,
            operation: Operation::PrimeFactorize,
        });

        self.register(Rule {
            id: "pre.frac_simplify".into(),
            domain: Domain::PreAlgebra,
            name: "Fraction Simplification".into(),
            description: "Simplify a/b by dividing numerator and denominator by their GCD.".into(),
            prerequisites: vec!["pre.gcd".into(), "ext.div".into()],
            patterns: vec![PatternType::FractionSimplification],
            properties: vec![],
            inverse: None,
            operation: Operation::FractionSimplify,
        });

        self.register(Rule {
            id: "pre.frac_add".into(),
            domain: Domain::PreAlgebra,
            name: "Fraction Addition".into(),
            description: "a/b + c/d = (a*d + c*b) / (b*d), then simplify.".into(),
            prerequisites: vec!["ext.mul".into(), "ext.add".into(), "pre.lcm".into(), "pre.frac_simplify".into()],
            patterns: vec![PatternType::FractionAddition],
            properties: vec![Property::Commutative, Property::Associative],
            inverse: Some("pre.frac_sub".into()),
            operation: Operation::FractionAdd,
        });

        self.register(Rule {
            id: "pre.frac_sub".into(),
            domain: Domain::PreAlgebra,
            name: "Fraction Subtraction".into(),
            description: "a/b - c/d = (a*d - c*b) / (b*d), then simplify.".into(),
            prerequisites: vec!["ext.mul".into(), "ext.sub".into(), "pre.lcm".into(), "pre.frac_simplify".into()],
            patterns: vec![PatternType::FractionSubtraction],
            properties: vec![],
            inverse: Some("pre.frac_add".into()),
            operation: Operation::FractionSub,
        });

        self.register(Rule {
            id: "pre.frac_mul".into(),
            domain: Domain::PreAlgebra,
            name: "Fraction Multiplication".into(),
            description: "(a/b) * (c/d) = (a*c) / (b*d), then simplify.".into(),
            prerequisites: vec!["ext.mul".into(), "pre.frac_simplify".into()],
            patterns: vec![PatternType::FractionMultiplication],
            properties: vec![Property::Commutative, Property::Associative],
            inverse: Some("pre.frac_div".into()),
            operation: Operation::FractionMul,
        });

        self.register(Rule {
            id: "pre.frac_div".into(),
            domain: Domain::PreAlgebra,
            name: "Fraction Division".into(),
            description: "(a/b) / (c/d) = (a/b) * (d/c) = (a*d) / (b*c). Multiply by reciprocal.".into(),
            prerequisites: vec!["pre.frac_mul".into()],
            patterns: vec![PatternType::FractionDivision],
            properties: vec![],
            inverse: Some("pre.frac_mul".into()),
            operation: Operation::FractionDiv,
        });

        self.register(Rule {
            id: "pre.decimal".into(),
            domain: Domain::PreAlgebra,
            name: "Decimal Conversion".into(),
            description: "Convert between fractions and decimals. a/b = a divided by b.".into(),
            prerequisites: vec!["ext.div".into()],
            patterns: vec![PatternType::DecimalConversion],
            properties: vec![],
            inverse: None,
            operation: Operation::DecimalConvert,
        });

        self.register(Rule {
            id: "pre.percent".into(),
            domain: Domain::PreAlgebra,
            name: "Percentage Calculation".into(),
            description: "x% of n = (x/100) * n. Convert between percentage, decimal, and fraction.".into(),
            prerequisites: vec!["pre.decimal".into(), "ext.mul".into(), "ext.div".into()],
            patterns: vec![PatternType::PercentageCalculation],
            properties: vec![],
            inverse: None,
            operation: Operation::PercentCalc,
        });

        self.register(Rule {
            id: "pre.exponent".into(),
            domain: Domain::PreAlgebra,
            name: "Exponent Evaluation".into(),
            description: "a^n = a * a * ... * a (n times). Built on repeated multiplication.".into(),
            prerequisites: vec!["ext.mul".into()],
            patterns: vec![PatternType::ExponentEvaluation],
            properties: vec![],
            inverse: Some("pre.nth_root".into()),
            operation: Operation::ExponentEval,
        });

        self.register(Rule {
            id: "pre.exponent_rules".into(),
            domain: Domain::PreAlgebra,
            name: "Exponent Rules".into(),
            description: "Product: a^m * a^n = a^(m+n). Quotient: a^m / a^n = a^(m-n). Power: (a^m)^n = a^(m*n). Zero: a^0 = 1. Negative: a^(-n) = 1/a^n.".into(),
            prerequisites: vec!["pre.exponent".into(), "ext.add".into(), "ext.sub".into(), "ext.mul".into()],
            patterns: vec![PatternType::ExponentRules],
            properties: vec![],
            inverse: None,
            operation: Operation::ExponentRule,
        });

        self.register(Rule {
            id: "pre.sqrt".into(),
            domain: Domain::PreAlgebra,
            name: "Square Root".into(),
            description: "sqrt(a) = b where b^2 = a. Inverse of squaring.".into(),
            prerequisites: vec!["pre.exponent".into()],
            patterns: vec![PatternType::SquareRoot],
            properties: vec![],
            inverse: Some("pre.exponent".into()),
            operation: Operation::SqrtCalc,
        });

        self.register(Rule {
            id: "pre.nth_root".into(),
            domain: Domain::PreAlgebra,
            name: "Nth Root".into(),
            description: "n-th root of a = b where b^n = a. Generalization of square root.".into(),
            prerequisites: vec!["pre.exponent".into()],
            patterns: vec![PatternType::NthRoot],
            properties: vec![],
            inverse: Some("pre.exponent".into()),
            operation: Operation::NthRootCalc,
        });

        self.register(Rule {
            id: "pre.abs".into(),
            domain: Domain::PreAlgebra,
            name: "Absolute Value".into(),
            description: "|a| = a if a >= 0, -a if a < 0. Distance from zero.".into(),
            prerequisites: vec![],
            patterns: vec![PatternType::AbsoluteValue],
            properties: vec![],
            inverse: None,
            operation: Operation::AbsoluteVal,
        });

        self.register(Rule {
            id: "pre.sci_notation".into(),
            domain: Domain::PreAlgebra,
            name: "Scientific Notation".into(),
            description: "Express as a * 10^n where 1 <= a < 10.".into(),
            prerequisites: vec!["pre.exponent".into(), "pre.decimal".into()],
            patterns: vec![PatternType::ScientificNotation],
            properties: vec![],
            inverse: None,
            operation: Operation::SciNotation,
        });

        self.register(Rule {
            id: "pre.ratio".into(),
            domain: Domain::PreAlgebra,
            name: "Ratio".into(),
            description: "Express the relationship a:b and simplify.".into(),
            prerequisites: vec!["pre.gcd".into(), "ext.div".into()],
            patterns: vec![PatternType::Ratio],
            properties: vec![],
            inverse: None,
            operation: Operation::FractionSimplify, // Ratios simplify like fractions
        });

        self.register(Rule {
            id: "pre.proportion".into(),
            domain: Domain::PreAlgebra,
            name: "Proportion".into(),
            description: "If a/b = c/d, then a*d = b*c (cross multiplication).".into(),
            prerequisites: vec!["ext.mul".into(), "ext.div".into()],
            patterns: vec![PatternType::Proportion],
            properties: vec![],
            inverse: None,
            operation: Operation::FractionDiv, // Cross-multiplication is fraction division
        });
    }

    fn register_tier4(&mut self) {
        // TIER 4: Algebra

        self.register(Rule {
            id: "alg.substitute".into(),
            domain: Domain::Algebra,
            name: "Variable Substitution".into(),
            description: "Replace a variable with a known value and evaluate.".into(),
            prerequisites: vec!["core.add".into(), "core.mul".into()],
            patterns: vec![PatternType::VariableSubstitution],
            properties: vec![],
            inverse: None,
            operation: Operation::Substitute,
        });

        self.register(Rule {
            id: "alg.like_terms".into(),
            domain: Domain::Algebra,
            name: "Combine Like Terms".into(),
            description: "ax + bx = (a+b)x. Terms with the same variable and exponent combine by adding coefficients.".into(),
            prerequisites: vec!["ext.add".into(), "prop.distributive".into()],
            patterns: vec![PatternType::LikeTermsCombination],
            properties: vec![Property::Commutative],
            inverse: None,
            operation: Operation::CombineLikeTerms,
        });

        self.register(Rule {
            id: "alg.linear_solve".into(),
            domain: Domain::Algebra,
            name: "Solve Linear Equation".into(),
            description: "Solve ax + b = c for x. x = (c - b) / a.".into(),
            prerequisites: vec!["ext.sub".into(), "ext.div".into(), "alg.like_terms".into()],
            patterns: vec![PatternType::LinearEquationSolve],
            properties: vec![],
            inverse: None,
            operation: Operation::SolveLinear,
        });

        self.register(Rule {
            id: "alg.linear_inequality".into(),
            domain: Domain::Algebra,
            name: "Solve Linear Inequality".into(),
            description: "Solve ax + b < c. Same as linear equation but flip inequality when dividing by negative.".into(),
            prerequisites: vec!["alg.linear_solve".into()],
            patterns: vec![PatternType::LinearInequalitySolve],
            properties: vec![],
            inverse: None,
            operation: Operation::SolveLinearInequality,
        });

        self.register(Rule {
            id: "alg.quadratic_solve".into(),
            domain: Domain::Algebra,
            name: "Solve Quadratic Equation".into(),
            description: "Solve ax^2 + bx + c = 0 via quadratic formula: x = (-b +/- sqrt(b^2-4ac)) / (2a).".into(),
            prerequisites: vec!["pre.sqrt".into(), "ext.mul".into(), "ext.sub".into(), "ext.div".into()],
            patterns: vec![PatternType::QuadraticEquationSolve],
            properties: vec![],
            inverse: None,
            operation: Operation::SolveQuadratic,
        });

        self.register(Rule {
            id: "alg.poly_add".into(),
            domain: Domain::Algebra,
            name: "Polynomial Addition".into(),
            description: "Add polynomials by combining like terms.".into(),
            prerequisites: vec!["alg.like_terms".into()],
            patterns: vec![PatternType::PolynomialAddition],
            properties: vec![Property::Commutative, Property::Associative],
            inverse: None,
            operation: Operation::PolyAdd,
        });

        self.register(Rule {
            id: "alg.poly_mul".into(),
            domain: Domain::Algebra,
            name: "Polynomial Multiplication".into(),
            description: "Multiply polynomials by distributing each term. FOIL for binomials.".into(),
            prerequisites: vec!["prop.distributive".into(), "alg.like_terms".into(), "pre.exponent_rules".into()],
            patterns: vec![PatternType::PolynomialMultiplication],
            properties: vec![Property::Commutative, Property::Associative],
            inverse: Some("alg.poly_div".into()),
            operation: Operation::PolyMul,
        });

        self.register(Rule {
            id: "alg.poly_div".into(),
            domain: Domain::Algebra,
            name: "Polynomial Division".into(),
            description: "Divide polynomials using long division or synthetic division.".into(),
            prerequisites: vec!["ext.div".into(), "alg.like_terms".into()],
            patterns: vec![PatternType::PolynomialDivision],
            properties: vec![],
            inverse: Some("alg.poly_mul".into()),
            operation: Operation::PolyDiv,
        });

        self.register(Rule {
            id: "alg.factor".into(),
            domain: Domain::Algebra,
            name: "Factoring".into(),
            description: "Factor polynomial expressions: GCF, difference of squares, trinomials, grouping.".into(),
            prerequisites: vec!["prop.factor".into(), "pre.gcd".into()],
            patterns: vec![PatternType::Factoring],
            properties: vec![],
            inverse: Some("alg.poly_mul".into()),
            operation: Operation::PolyFactor,
        });

        self.register(Rule {
            id: "alg.func_eval".into(),
            domain: Domain::Algebra,
            name: "Function Evaluation".into(),
            description: "Evaluate f(x) at x = a by substituting and computing.".into(),
            prerequisites: vec!["alg.substitute".into()],
            patterns: vec![PatternType::FunctionEvaluation],
            properties: vec![],
            inverse: None,
            operation: Operation::FuncEval,
        });

        self.register(Rule {
            id: "alg.func_compose".into(),
            domain: Domain::Algebra,
            name: "Function Composition".into(),
            description: "f(g(x)): substitute g(x) into f.".into(),
            prerequisites: vec!["alg.func_eval".into(), "alg.substitute".into()],
            patterns: vec![PatternType::FunctionComposition],
            properties: vec![],
            inverse: None,
            operation: Operation::FuncCompose,
        });

        self.register(Rule {
            id: "alg.slope".into(),
            domain: Domain::Algebra,
            name: "Slope Calculation".into(),
            description: "m = (y2-y1)/(x2-x1). Rise over run between two points.".into(),
            prerequisites: vec!["ext.sub".into(), "ext.div".into()],
            patterns: vec![PatternType::SlopeIntercept, PatternType::PointSlope],
            properties: vec![],
            inverse: None,
            operation: Operation::SlopeCalc,
        });

        self.register(Rule {
            id: "alg.domain_range".into(),
            domain: Domain::Algebra,
            name: "Domain and Range".into(),
            description: "Determine valid input (domain) and output (range) of a function.".into(),
            prerequisites: vec!["alg.func_eval".into()],
            patterns: vec![PatternType::DomainRange],
            properties: vec![],
            inverse: None,
            operation: Operation::DomainRangeCalc,
        });
    }

    fn register_tier5(&mut self) {
        // TIER 5: Advanced Algebra

        self.register(Rule {
            id: "adv.system_solve".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Solve System of Equations".into(),
            description: "Solve systems via substitution, elimination, or Cramer's rule.".into(),
            prerequisites: vec!["alg.linear_solve".into(), "ext.mul".into(), "ext.sub".into()],
            patterns: vec![PatternType::SystemOfEquations],
            properties: vec![],
            inverse: None,
            operation: Operation::SolveSystem,
        });

        self.register(Rule {
            id: "adv.mat_add".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Matrix Addition".into(),
            description: "Add matrices element-wise. Matrices must have same dimensions.".into(),
            prerequisites: vec!["ext.add".into()],
            patterns: vec![PatternType::MatrixAddition],
            properties: vec![Property::Commutative, Property::Associative],
            inverse: None,
            operation: Operation::MatAdd,
        });

        self.register(Rule {
            id: "adv.mat_mul".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Matrix Multiplication".into(),
            description: "Multiply matrices. (AB)_ij = sum of A_ik * B_kj. Dimensions must be compatible.".into(),
            prerequisites: vec!["ext.mul".into(), "ext.add".into()],
            patterns: vec![PatternType::MatrixMultiplication],
            properties: vec![Property::Associative], // NOT commutative
            inverse: None,
            operation: Operation::MatMul,
        });

        self.register(Rule {
            id: "adv.mat_det".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Matrix Determinant".into(),
            description: "Compute determinant. 2x2: ad-bc. Larger: cofactor expansion.".into(),
            prerequisites: vec!["ext.mul".into(), "ext.sub".into()],
            patterns: vec![PatternType::MatrixDeterminant],
            properties: vec![],
            inverse: None,
            operation: Operation::MatDet,
        });

        self.register(Rule {
            id: "adv.mat_inv".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Matrix Inverse".into(),
            description: "Find A^(-1) such that A * A^(-1) = I. Requires det(A) != 0.".into(),
            prerequisites: vec!["adv.mat_det".into(), "adv.mat_mul".into()],
            patterns: vec![PatternType::MatrixInverse],
            properties: vec![],
            inverse: None,
            operation: Operation::MatInv,
        });

        self.register(Rule {
            id: "adv.complex_add".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Complex Addition".into(),
            description: "(a+bi) + (c+di) = (a+c) + (b+d)i. Add real and imaginary parts separately.".into(),
            prerequisites: vec!["ext.add".into()],
            patterns: vec![PatternType::ComplexAddition],
            properties: vec![Property::Commutative, Property::Associative],
            inverse: None,
            operation: Operation::ComplexAdd,
        });

        self.register(Rule {
            id: "adv.complex_mul".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Complex Multiplication".into(),
            description: "(a+bi)(c+di) = (ac-bd) + (ad+bc)i. FOIL with i^2 = -1.".into(),
            prerequisites: vec!["ext.mul".into(), "ext.sub".into(), "ext.add".into()],
            patterns: vec![PatternType::ComplexMultiplication],
            properties: vec![Property::Commutative, Property::Associative],
            inverse: None,
            operation: Operation::ComplexMul,
        });

        self.register(Rule {
            id: "adv.complex_conj".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Complex Conjugate".into(),
            description: "Conjugate of a+bi is a-bi.".into(),
            prerequisites: vec![],
            patterns: vec![PatternType::ComplexConjugate],
            properties: vec![],
            inverse: None,
            operation: Operation::ComplexConj,
        });

        self.register(Rule {
            id: "adv.complex_mod".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Complex Modulus".into(),
            description: "|a+bi| = sqrt(a^2 + b^2).".into(),
            prerequisites: vec!["pre.sqrt".into(), "pre.exponent".into(), "ext.add".into()],
            patterns: vec![PatternType::ComplexModulus],
            properties: vec![],
            inverse: None,
            operation: Operation::ComplexMod,
        });

        self.register(Rule {
            id: "adv.arith_seq".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Arithmetic Sequence".into(),
            description: "a_n = a_1 + (n-1)d. Sum: S_n = n/2 * (a_1 + a_n).".into(),
            prerequisites: vec!["ext.add".into(), "ext.mul".into(), "ext.div".into()],
            patterns: vec![PatternType::ArithmeticSequence],
            properties: vec![],
            inverse: None,
            operation: Operation::ArithSeq,
        });

        self.register(Rule {
            id: "adv.geom_seq".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Geometric Sequence".into(),
            description: "a_n = a_1 * r^(n-1). Sum: S_n = a_1 * (1-r^n)/(1-r).".into(),
            prerequisites: vec!["ext.mul".into(), "pre.exponent".into(), "ext.div".into()],
            patterns: vec![PatternType::GeometricSequence],
            properties: vec![],
            inverse: None,
            operation: Operation::GeomSeq,
        });

        self.register(Rule {
            id: "adv.series".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Series Sum".into(),
            description: "Evaluate finite or convergent infinite series.".into(),
            prerequisites: vec!["adv.arith_seq".into(), "adv.geom_seq".into()],
            patterns: vec![PatternType::SeriesSum],
            properties: vec![],
            inverse: None,
            operation: Operation::SeriesEval,
        });

        self.register(Rule {
            id: "adv.log".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Logarithm Evaluation".into(),
            description: "log_b(x) = y means b^y = x. The inverse of exponentiation.".into(),
            prerequisites: vec!["pre.exponent".into()],
            patterns: vec![PatternType::Logarithm],
            properties: vec![],
            inverse: Some("pre.exponent".into()),
            operation: Operation::LogEval,
        });

        self.register(Rule {
            id: "adv.log_rules".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Logarithm Rules".into(),
            description: "Product: log(ab) = log(a)+log(b). Quotient: log(a/b) = log(a)-log(b). Power: log(a^n) = n*log(a). Change of base: log_b(x) = log(x)/log(b).".into(),
            prerequisites: vec!["adv.log".into(), "ext.add".into(), "ext.sub".into(), "ext.mul".into(), "ext.div".into()],
            patterns: vec![PatternType::LogarithmRules],
            properties: vec![],
            inverse: None,
            operation: Operation::LogRule,
        });

        self.register(Rule {
            id: "adv.exp_equation".into(),
            domain: Domain::AdvancedAlgebra,
            name: "Solve Exponential Equation".into(),
            description: "Solve b^x = c by taking logarithm of both sides.".into(),
            prerequisites: vec!["adv.log".into(), "adv.log_rules".into()],
            patterns: vec![PatternType::ExponentialEquation],
            properties: vec![],
            inverse: None,
            operation: Operation::SolveExponential,
        });
    }

    fn register_tier6(&mut self) {
        // TIER 6: Trigonometry

        self.register(Rule {
            id: "trig.eval".into(),
            domain: Domain::Trigonometry,
            name: "Trigonometric Function Evaluation".into(),
            description: "Evaluate sin, cos, tan, csc, sec, cot at a given angle.".into(),
            prerequisites: vec!["ext.div".into(), "pre.sqrt".into()],
            patterns: vec![PatternType::TrigEvaluation],
            properties: vec![],
            inverse: Some("trig.inverse".into()),
            operation: Operation::TrigEval,
        });

        self.register(Rule {
            id: "trig.identity".into(),
            domain: Domain::Trigonometry,
            name: "Trigonometric Identity".into(),
            description: "Apply identities: Pythagorean (sin^2+cos^2=1), double angle, sum/difference, half angle.".into(),
            prerequisites: vec!["trig.eval".into(), "pre.exponent".into(), "ext.add".into()],
            patterns: vec![PatternType::TrigIdentity],
            properties: vec![],
            inverse: None,
            operation: Operation::TrigIdentityApply,
        });

        self.register(Rule {
            id: "trig.inverse".into(),
            domain: Domain::Trigonometry,
            name: "Inverse Trigonometric Functions".into(),
            description: "Evaluate arcsin, arccos, arctan. Returns angle for given ratio.".into(),
            prerequisites: vec!["trig.eval".into()],
            patterns: vec![PatternType::InverseTrig],
            properties: vec![],
            inverse: Some("trig.eval".into()),
            operation: Operation::InverseTrigEval,
        });

        self.register(Rule {
            id: "trig.unit_circle".into(),
            domain: Domain::Trigonometry,
            name: "Unit Circle Lookup".into(),
            description: "Retrieve exact trig values for standard angles (0, 30, 45, 60, 90, etc.).".into(),
            prerequisites: vec!["pre.sqrt".into(), "ext.div".into()],
            patterns: vec![PatternType::UnitCircleLookup],
            properties: vec![],
            inverse: None,
            operation: Operation::UnitCircle,
        });

        self.register(Rule {
            id: "trig.law_sines".into(),
            domain: Domain::Trigonometry,
            name: "Law of Sines".into(),
            description: "a/sin(A) = b/sin(B) = c/sin(C).".into(),
            prerequisites: vec!["trig.eval".into(), "ext.div".into()],
            patterns: vec![PatternType::LawOfSines],
            properties: vec![],
            inverse: None,
            operation: Operation::LawSines,
        });

        self.register(Rule {
            id: "trig.law_cosines".into(),
            domain: Domain::Trigonometry,
            name: "Law of Cosines".into(),
            description: "c^2 = a^2 + b^2 - 2ab*cos(C). Generalization of Pythagorean theorem.".into(),
            prerequisites: vec!["trig.eval".into(), "pre.exponent".into(), "pre.sqrt".into(), "ext.mul".into(), "ext.sub".into()],
            patterns: vec![PatternType::LawOfCosines],
            properties: vec![],
            inverse: None,
            operation: Operation::LawCosines,
        });

        self.register(Rule {
            id: "trig.solve_eq".into(),
            domain: Domain::Trigonometry,
            name: "Solve Trigonometric Equation".into(),
            description: "Solve equations involving trig functions for unknown angles.".into(),
            prerequisites: vec!["trig.inverse".into(), "trig.identity".into()],
            patterns: vec![PatternType::TrigEquationSolve],
            properties: vec![],
            inverse: None,
            operation: Operation::SolveTrigEq,
        });

        self.register(Rule {
            id: "trig.polar_rect".into(),
            domain: Domain::Trigonometry,
            name: "Polar-Rectangular Conversion".into(),
            description: "Rectangular to polar: r=sqrt(x^2+y^2), theta=atan(y/x). Polar to rectangular: x=r*cos(theta), y=r*sin(theta).".into(),
            prerequisites: vec!["trig.eval".into(), "trig.inverse".into(), "pre.sqrt".into(), "pre.exponent".into()],
            patterns: vec![PatternType::PolarRectangularConversion],
            properties: vec![],
            inverse: None,
            operation: Operation::PolarRectConvert,
        });
    }

    fn register_tier7(&mut self) {
        // TIER 7: Calculus

        self.register(Rule { id: "calc.limit".into(), domain: Domain::Calculus, name: "Limit Evaluation".into(),
            description: "Evaluate lim(x->a) f(x). Direct substitution, factoring, L'Hopital's rule.".into(),
            prerequisites: vec!["alg.substitute".into(), "alg.factor".into()],
            patterns: vec![PatternType::LimitEvaluation], properties: vec![], inverse: None, operation: Operation::LimitEval });

        self.register(Rule { id: "calc.deriv_basic".into(), domain: Domain::Calculus, name: "Basic Derivative".into(),
            description: "Power rule: d/dx[x^n] = n*x^(n-1). Constant: d/dx[c] = 0. Sum rule.".into(),
            prerequisites: vec!["pre.exponent_rules".into(), "ext.mul".into(), "ext.sub".into()],
            patterns: vec![PatternType::DerivativeBasic], properties: vec![], inverse: Some("calc.integral_basic".into()), operation: Operation::DerivBasic });

        self.register(Rule { id: "calc.deriv_chain".into(), domain: Domain::Calculus, name: "Chain Rule".into(),
            description: "d/dx[f(g(x))] = f'(g(x)) * g'(x).".into(),
            prerequisites: vec!["calc.deriv_basic".into(), "alg.func_compose".into()],
            patterns: vec![PatternType::DerivativeChainRule], properties: vec![], inverse: None, operation: Operation::DerivChain });

        self.register(Rule { id: "calc.deriv_product".into(), domain: Domain::Calculus, name: "Product Rule".into(),
            description: "d/dx[f(x)*g(x)] = f'(x)*g(x) + f(x)*g'(x).".into(),
            prerequisites: vec!["calc.deriv_basic".into(), "ext.mul".into(), "ext.add".into()],
            patterns: vec![PatternType::DerivativeProductRule], properties: vec![], inverse: None, operation: Operation::DerivProduct });

        self.register(Rule { id: "calc.deriv_quotient".into(), domain: Domain::Calculus, name: "Quotient Rule".into(),
            description: "d/dx[f/g] = (f'g - fg') / g^2.".into(),
            prerequisites: vec!["calc.deriv_basic".into(), "ext.mul".into(), "ext.sub".into(), "pre.exponent".into()],
            patterns: vec![PatternType::DerivativeQuotientRule], properties: vec![], inverse: None, operation: Operation::DerivQuotient });

        self.register(Rule { id: "calc.integral_basic".into(), domain: Domain::Calculus, name: "Basic Integral".into(),
            description: "Power rule: integral of x^n = x^(n+1)/(n+1) + C. Constant multiple. Sum rule.".into(),
            prerequisites: vec!["pre.exponent_rules".into(), "ext.div".into(), "ext.add".into()],
            patterns: vec![PatternType::IntegralBasic], properties: vec![], inverse: Some("calc.deriv_basic".into()), operation: Operation::IntegralBasic });

        self.register(Rule { id: "calc.integral_usub".into(), domain: Domain::Calculus, name: "Integration by Substitution".into(),
            description: "U-substitution: integral of f(g(x))*g'(x) dx = integral of f(u) du.".into(),
            prerequisites: vec!["calc.integral_basic".into(), "calc.deriv_chain".into()],
            patterns: vec![PatternType::IntegralBySubstitution], properties: vec![], inverse: None, operation: Operation::IntegralUSub });

        self.register(Rule { id: "calc.integral_parts".into(), domain: Domain::Calculus, name: "Integration by Parts".into(),
            description: "integral of u dv = uv - integral of v du.".into(),
            prerequisites: vec!["calc.integral_basic".into(), "calc.deriv_basic".into()],
            patterns: vec![PatternType::IntegralByParts], properties: vec![], inverse: None, operation: Operation::IntegralParts });

        self.register(Rule { id: "calc.definite_integral".into(), domain: Domain::Calculus, name: "Definite Integral".into(),
            description: "Evaluate integral from a to b = F(b) - F(a) where F is antiderivative.".into(),
            prerequisites: vec!["calc.integral_basic".into(), "alg.substitute".into(), "ext.sub".into()],
            patterns: vec![PatternType::DefiniteIntegral], properties: vec![], inverse: None, operation: Operation::DefiniteIntegralEval });

        self.register(Rule { id: "calc.series_conv".into(), domain: Domain::Calculus, name: "Series Convergence".into(),
            description: "Test convergence: ratio test, root test, comparison, integral test, alternating series.".into(),
            prerequisites: vec!["calc.limit".into(), "adv.series".into()],
            patterns: vec![PatternType::SeriesConvergence], properties: vec![], inverse: None, operation: Operation::SeriesConvergenceTest });

        self.register(Rule { id: "calc.taylor".into(), domain: Domain::Calculus, name: "Taylor Series".into(),
            description: "f(x) = sum of f^(n)(a)/n! * (x-a)^n. Expand function as infinite polynomial.".into(),
            prerequisites: vec!["calc.deriv_basic".into(), "adv.series".into(), "pre.exponent".into()],
            patterns: vec![PatternType::TaylorSeries], properties: vec![], inverse: None, operation: Operation::TaylorExpand });

        self.register(Rule { id: "calc.diff_eq".into(), domain: Domain::Calculus, name: "Differential Equation".into(),
            description: "Solve ODEs: separable, first-order linear, second-order with constant coefficients.".into(),
            prerequisites: vec!["calc.integral_basic".into(), "calc.deriv_basic".into(), "alg.linear_solve".into()],
            patterns: vec![PatternType::DifferentialEquation], properties: vec![], inverse: None, operation: Operation::SolveDE });

        self.register(Rule { id: "calc.partial_deriv".into(), domain: Domain::Calculus, name: "Partial Derivative".into(),
            description: "Differentiate with respect to one variable, treating others as constants.".into(),
            prerequisites: vec!["calc.deriv_basic".into()],
            patterns: vec![PatternType::PartialDerivative], properties: vec![], inverse: None, operation: Operation::PartialDeriv });

        self.register(Rule { id: "calc.multi_integral".into(), domain: Domain::Calculus, name: "Multiple Integral".into(),
            description: "Evaluate double/triple integrals by iterated integration.".into(),
            prerequisites: vec!["calc.definite_integral".into()],
            patterns: vec![PatternType::MultipleIntegral], properties: vec![], inverse: None, operation: Operation::MultiIntegral });
    }

    fn register_tier8(&mut self) {
        // TIER 8: Linear Algebra

        self.register(Rule { id: "la.vec_add".into(), domain: Domain::LinearAlgebra, name: "Vector Addition".into(),
            description: "Add vectors component-wise.".into(),
            prerequisites: vec!["ext.add".into()],
            patterns: vec![PatternType::VectorAddition], properties: vec![Property::Commutative, Property::Associative], inverse: None, operation: Operation::VecAdd });

        self.register(Rule { id: "la.dot_product".into(), domain: Domain::LinearAlgebra, name: "Dot Product".into(),
            description: "a . b = sum of a_i * b_i. Produces scalar.".into(),
            prerequisites: vec!["ext.mul".into(), "ext.add".into()],
            patterns: vec![PatternType::VectorDotProduct], properties: vec![Property::Commutative], inverse: None, operation: Operation::DotProduct });

        self.register(Rule { id: "la.cross_product".into(), domain: Domain::LinearAlgebra, name: "Cross Product".into(),
            description: "a x b = (a2b3-a3b2, a3b1-a1b3, a1b2-a2b1). Produces vector. 3D only.".into(),
            prerequisites: vec!["ext.mul".into(), "ext.sub".into()],
            patterns: vec![PatternType::VectorCrossProduct], properties: vec![], inverse: None, operation: Operation::CrossProduct });

        self.register(Rule { id: "la.vec_norm".into(), domain: Domain::LinearAlgebra, name: "Vector Norm".into(),
            description: "||v|| = sqrt(sum of v_i^2). Euclidean length.".into(),
            prerequisites: vec!["pre.sqrt".into(), "pre.exponent".into(), "ext.add".into()],
            patterns: vec![PatternType::VectorNorm], properties: vec![], inverse: None, operation: Operation::VecNorm });

        self.register(Rule { id: "la.transpose".into(), domain: Domain::LinearAlgebra, name: "Matrix Transpose".into(),
            description: "(A^T)_ij = A_ji. Swap rows and columns.".into(),
            prerequisites: vec![],
            patterns: vec![PatternType::MatrixTranspose], properties: vec![], inverse: None, operation: Operation::Transpose });

        self.register(Rule { id: "la.eigenvalue".into(), domain: Domain::LinearAlgebra, name: "Eigenvalue".into(),
            description: "Find lambda where det(A - lambda*I) = 0.".into(),
            prerequisites: vec!["adv.mat_det".into(), "alg.quadratic_solve".into()],
            patterns: vec![PatternType::Eigenvalue], properties: vec![], inverse: None, operation: Operation::EigenvalueCalc });

        self.register(Rule { id: "la.eigenvector".into(), domain: Domain::LinearAlgebra, name: "Eigenvector".into(),
            description: "Find v where Av = lambda*v. Solve (A - lambda*I)v = 0.".into(),
            prerequisites: vec!["la.eigenvalue".into(), "adv.system_solve".into()],
            patterns: vec![PatternType::Eigenvector], properties: vec![], inverse: None, operation: Operation::EigenvectorCalc });

        self.register(Rule { id: "la.linear_transform".into(), domain: Domain::LinearAlgebra, name: "Linear Transformation".into(),
            description: "T(v) = Av. Apply matrix to vector.".into(),
            prerequisites: vec!["adv.mat_mul".into()],
            patterns: vec![PatternType::LinearTransformation], properties: vec![], inverse: None, operation: Operation::LinearTransform });

        self.register(Rule { id: "la.mat_decompose".into(), domain: Domain::LinearAlgebra, name: "Matrix Decomposition".into(),
            description: "LU, QR, SVD, Cholesky decompositions.".into(),
            prerequisites: vec!["adv.mat_mul".into(), "adv.mat_det".into(), "la.vec_norm".into()],
            patterns: vec![PatternType::MatrixDecomposition], properties: vec![], inverse: None, operation: Operation::MatDecompose });

        self.register(Rule { id: "la.vec_project".into(), domain: Domain::LinearAlgebra, name: "Vector Projection".into(),
            description: "proj_b(a) = (a.b / b.b) * b.".into(),
            prerequisites: vec!["la.dot_product".into(), "ext.div".into(), "ext.mul".into()],
            patterns: vec![PatternType::VectorProjection], properties: vec![], inverse: None, operation: Operation::VecProject });

        self.register(Rule { id: "la.gram_schmidt".into(), domain: Domain::LinearAlgebra, name: "Gram-Schmidt".into(),
            description: "Orthogonalize a set of vectors.".into(),
            prerequisites: vec!["la.vec_project".into(), "la.vec_norm".into()],
            patterns: vec![PatternType::GramSchmidt], properties: vec![], inverse: None, operation: Operation::GramSchmidtCalc });

        self.register(Rule { id: "la.rank_nullity".into(), domain: Domain::LinearAlgebra, name: "Rank and Nullity".into(),
            description: "rank(A) + nullity(A) = n. Compute via row echelon form.".into(),
            prerequisites: vec!["adv.system_solve".into()],
            patterns: vec![PatternType::RankNullity], properties: vec![], inverse: None, operation: Operation::RankNullityCalc });
    }

    fn register_tier9(&mut self) {
        // TIER 9: Abstract/Applied Mathematics

        self.register(Rule { id: "abs.mod_arith".into(), domain: Domain::AbstractApplied, name: "Modular Arithmetic".into(),
            description: "a mod n. Clock arithmetic. Congruence relations.".into(),
            prerequisites: vec!["ext.div".into()],
            patterns: vec![PatternType::ModularArithmetic], properties: vec![], inverse: None, operation: Operation::ModArith });

        self.register(Rule { id: "abs.permutation".into(), domain: Domain::AbstractApplied, name: "Permutation".into(),
            description: "P(n,r) = n! / (n-r)!. Ordered arrangements.".into(),
            prerequisites: vec!["ext.mul".into(), "ext.div".into()],
            patterns: vec![PatternType::Permutation], properties: vec![], inverse: None, operation: Operation::PermutationCount });

        self.register(Rule { id: "abs.combination".into(), domain: Domain::AbstractApplied, name: "Combination".into(),
            description: "C(n,r) = n! / (r! * (n-r)!). Unordered selections.".into(),
            prerequisites: vec!["abs.permutation".into(), "ext.div".into()],
            patterns: vec![PatternType::Combination], properties: vec![], inverse: None, operation: Operation::CombinationCount });

        self.register(Rule { id: "abs.prob_basic".into(), domain: Domain::AbstractApplied, name: "Basic Probability".into(),
            description: "P(A) = favorable outcomes / total outcomes. 0 <= P(A) <= 1.".into(),
            prerequisites: vec!["ext.div".into(), "abs.combination".into()],
            patterns: vec![PatternType::ProbabilityBasic], properties: vec![], inverse: None, operation: Operation::ProbCalc });

        self.register(Rule { id: "abs.prob_cond".into(), domain: Domain::AbstractApplied, name: "Conditional Probability".into(),
            description: "P(A|B) = P(A and B) / P(B).".into(),
            prerequisites: vec!["abs.prob_basic".into(), "ext.div".into()],
            patterns: vec![PatternType::ProbabilityConditional], properties: vec![], inverse: None, operation: Operation::CondProbCalc });

        self.register(Rule { id: "abs.bayes".into(), domain: Domain::AbstractApplied, name: "Bayes' Theorem".into(),
            description: "P(A|B) = P(B|A) * P(A) / P(B).".into(),
            prerequisites: vec!["abs.prob_cond".into(), "ext.mul".into(), "ext.div".into()],
            patterns: vec![PatternType::BayesTheorem], properties: vec![], inverse: None, operation: Operation::BayesCalc });

        self.register(Rule { id: "abs.expected_val".into(), domain: Domain::AbstractApplied, name: "Expected Value".into(),
            description: "E[X] = sum of x_i * P(x_i). Weighted average of outcomes.".into(),
            prerequisites: vec!["abs.prob_basic".into(), "ext.mul".into(), "ext.add".into()],
            patterns: vec![PatternType::ExpectedValue], properties: vec![], inverse: None, operation: Operation::ExpectedValCalc });

        self.register(Rule { id: "abs.variance".into(), domain: Domain::AbstractApplied, name: "Variance".into(),
            description: "Var(X) = E[X^2] - (E[X])^2. Measure of spread.".into(),
            prerequisites: vec!["abs.expected_val".into(), "pre.exponent".into(), "ext.sub".into()],
            patterns: vec![PatternType::Variance], properties: vec![], inverse: None, operation: Operation::VarianceCalc });

        self.register(Rule { id: "abs.std_dev".into(), domain: Domain::AbstractApplied, name: "Standard Deviation".into(),
            description: "sigma = sqrt(Var(X)). Square root of variance.".into(),
            prerequisites: vec!["abs.variance".into(), "pre.sqrt".into()],
            patterns: vec![PatternType::StandardDeviation], properties: vec![], inverse: None, operation: Operation::StdDevCalc });

        self.register(Rule { id: "abs.normal_dist".into(), domain: Domain::AbstractApplied, name: "Normal Distribution".into(),
            description: "Z-scores, area under curve, probability from normal distribution.".into(),
            prerequisites: vec!["abs.std_dev".into(), "abs.expected_val".into(), "ext.div".into()],
            patterns: vec![PatternType::NormalDistribution], properties: vec![], inverse: None, operation: Operation::NormalDistCalc });

        self.register(Rule { id: "abs.hyp_test".into(), domain: Domain::AbstractApplied, name: "Hypothesis Test".into(),
            description: "Test statistical hypotheses: z-test, t-test, chi-square.".into(),
            prerequisites: vec!["abs.normal_dist".into(), "abs.std_dev".into()],
            patterns: vec![PatternType::HypothesisTest], properties: vec![], inverse: None, operation: Operation::HypTestCalc });

        self.register(Rule { id: "abs.graph_adj".into(), domain: Domain::AbstractApplied, name: "Graph Adjacency".into(),
            description: "Build and query adjacency matrix/list representations of graphs.".into(),
            prerequisites: vec!["adv.mat_add".into()],
            patterns: vec![PatternType::GraphAdjacency], properties: vec![], inverse: None, operation: Operation::GraphAdjCalc });

        self.register(Rule { id: "abs.shortest_path".into(), domain: Domain::AbstractApplied, name: "Shortest Path".into(),
            description: "Find shortest path in graph: Dijkstra's, Bellman-Ford, Floyd-Warshall.".into(),
            prerequisites: vec!["abs.graph_adj".into(), "ext.add".into()],
            patterns: vec![PatternType::ShortestPath], properties: vec![], inverse: None, operation: Operation::ShortestPathCalc });

        self.register(Rule { id: "abs.optimize".into(), domain: Domain::AbstractApplied, name: "Optimization".into(),
            description: "Find min/max of functions with or without constraints. Lagrange multipliers.".into(),
            prerequisites: vec!["calc.deriv_basic".into(), "adv.system_solve".into()],
            patterns: vec![PatternType::Optimization], properties: vec![], inverse: None, operation: Operation::OptimizeCalc });
    }
}

impl Default for RuleLibrary {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ================================================================
    // LIBRARY CONSTRUCTION TESTS
    // ================================================================

    #[test]
    fn test_library_creates_successfully() {
        let lib = RuleLibrary::new();
        assert!(lib.count() > 0, "Library should contain rules");
    }

    #[test]
    fn test_library_total_count() {
        let lib = RuleLibrary::new();
        // We expect rules across all 10 tiers
        let count = lib.count();
        assert!(count >= 90, "Expected at least 90 rules, got {}", count);
    }

    // ================================================================
    // TIER 0: CORE ARITHMETIC
    // ================================================================

    #[test]
    fn test_tier0_rules_exist() {
        let lib = RuleLibrary::new();
        assert!(lib.get("core.add").is_some());
        assert!(lib.get("core.sub").is_some());
        assert!(lib.get("core.mul").is_some());
        assert!(lib.get("core.div").is_some());
    }

    #[test]
    fn test_tier0_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::CoreArithmetic), 4);
    }

    #[test]
    fn test_tier0_no_prerequisites() {
        let lib = RuleLibrary::new();
        for rule in lib.retrieve_by_tier(0) {
            assert!(rule.prerequisites.is_empty(),
                "Tier 0 rule '{}' should have no prerequisites", rule.id);
        }
    }

    #[test]
    fn test_tier0_properties() {
        let lib = RuleLibrary::new();
        let add = lib.get("core.add").unwrap();
        assert!(add.properties.contains(&Property::Commutative));
        assert!(add.properties.contains(&Property::Associative));
        assert!(add.properties.contains(&Property::HasIdentity));

        let mul = lib.get("core.mul").unwrap();
        assert!(mul.properties.contains(&Property::Commutative));

        let sub = lib.get("core.sub").unwrap();
        assert!(!sub.properties.contains(&Property::Commutative), "Subtraction is NOT commutative");
    }

    #[test]
    fn test_tier0_inverses() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.get("core.add").unwrap().inverse.as_deref(), Some("core.sub"));
        assert_eq!(lib.get("core.sub").unwrap().inverse.as_deref(), Some("core.add"));
        assert_eq!(lib.get("core.mul").unwrap().inverse.as_deref(), Some("core.div"));
        assert_eq!(lib.get("core.div").unwrap().inverse.as_deref(), Some("core.mul"));
    }

    #[test]
    fn test_tier0_pattern_retrieval() {
        let lib = RuleLibrary::new();
        let rules = lib.retrieve_by_pattern(&PatternType::SingleDigitAddition);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].id, "core.add");
    }

    // ================================================================
    // TIER 1: EXTENDED ARITHMETIC
    // ================================================================

    #[test]
    fn test_tier1_rules_exist() {
        let lib = RuleLibrary::new();
        assert!(lib.get("ext.add").is_some());
        assert!(lib.get("ext.sub").is_some());
        assert!(lib.get("ext.mul").is_some());
        assert!(lib.get("ext.div").is_some());
        assert!(lib.get("ext.decompose").is_some());
        assert!(lib.get("ext.compose").is_some());
        assert!(lib.get("ext.order_of_ops").is_some());
    }

    #[test]
    fn test_tier1_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::ExtendedArithmetic), 7);
    }

    #[test]
    fn test_tier1_prerequisites_reference_tier0() {
        let lib = RuleLibrary::new();
        let ext_add = lib.get("ext.add").unwrap();
        assert!(ext_add.prerequisites.contains(&"core.add".to_string()));

        let ext_mul = lib.get("ext.mul").unwrap();
        assert!(ext_mul.prerequisites.contains(&"core.mul".to_string()));
        assert!(ext_mul.prerequisites.contains(&"ext.add".to_string()));
    }

    // ================================================================
    // TIER 2: PROPERTIES
    // ================================================================

    #[test]
    fn test_tier2_rules_exist() {
        let lib = RuleLibrary::new();
        assert!(lib.get("prop.commutative").is_some());
        assert!(lib.get("prop.associative").is_some());
        assert!(lib.get("prop.distributive").is_some());
        assert!(lib.get("prop.factor").is_some());
        assert!(lib.get("prop.identity").is_some());
        assert!(lib.get("prop.inverse").is_some());
    }

    #[test]
    fn test_tier2_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::Properties), 6);
    }

    #[test]
    fn test_distributive_inverse() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.get("prop.distributive").unwrap().inverse.as_deref(), Some("prop.factor"));
        assert_eq!(lib.get("prop.factor").unwrap().inverse.as_deref(), Some("prop.distributive"));
    }

    // ================================================================
    // TIER 3: PRE-ALGEBRA
    // ================================================================

    #[test]
    fn test_tier3_rules_exist() {
        let lib = RuleLibrary::new();
        let expected = vec![
            "pre.gcd", "pre.lcm", "pre.prime_factor", "pre.frac_simplify",
            "pre.frac_add", "pre.frac_sub", "pre.frac_mul", "pre.frac_div",
            "pre.decimal", "pre.percent", "pre.exponent", "pre.exponent_rules",
            "pre.sqrt", "pre.nth_root", "pre.abs", "pre.sci_notation",
            "pre.ratio", "pre.proportion",
        ];
        for id in expected {
            assert!(lib.get(id).is_some(), "Rule '{}' should exist", id);
        }
    }

    #[test]
    fn test_tier3_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::PreAlgebra), 18);
    }

    #[test]
    fn test_fraction_operations_have_prerequisites() {
        let lib = RuleLibrary::new();
        let frac_add = lib.get("pre.frac_add").unwrap();
        assert!(!frac_add.prerequisites.is_empty());
        assert!(frac_add.prerequisites.contains(&"pre.frac_simplify".to_string()));
    }

    // ================================================================
    // TIER 4: ALGEBRA
    // ================================================================

    #[test]
    fn test_tier4_rules_exist() {
        let lib = RuleLibrary::new();
        let expected = vec![
            "alg.substitute", "alg.like_terms", "alg.linear_solve",
            "alg.linear_inequality", "alg.quadratic_solve", "alg.poly_add",
            "alg.poly_mul", "alg.poly_div", "alg.factor", "alg.func_eval",
            "alg.func_compose", "alg.slope", "alg.domain_range",
        ];
        for id in expected {
            assert!(lib.get(id).is_some(), "Rule '{}' should exist", id);
        }
    }

    #[test]
    fn test_tier4_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::Algebra), 13);
    }

    #[test]
    fn test_like_terms_prerequisites() {
        let lib = RuleLibrary::new();
        let rule = lib.get("alg.like_terms").unwrap();
        assert!(rule.prerequisites.contains(&"ext.add".to_string()));
        assert!(rule.prerequisites.contains(&"prop.distributive".to_string()));
    }

    // ================================================================
    // TIER 5: ADVANCED ALGEBRA
    // ================================================================

    #[test]
    fn test_tier5_rules_exist() {
        let lib = RuleLibrary::new();
        let expected = vec![
            "adv.system_solve", "adv.mat_add", "adv.mat_mul", "adv.mat_det",
            "adv.mat_inv", "adv.complex_add", "adv.complex_mul", "adv.complex_conj",
            "adv.complex_mod", "adv.arith_seq", "adv.geom_seq", "adv.series",
            "adv.log", "adv.log_rules", "adv.exp_equation",
        ];
        for id in expected {
            assert!(lib.get(id).is_some(), "Rule '{}' should exist", id);
        }
    }

    #[test]
    fn test_tier5_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::AdvancedAlgebra), 15);
    }

    #[test]
    fn test_matrix_multiplication_not_commutative() {
        let lib = RuleLibrary::new();
        let mat_mul = lib.get("adv.mat_mul").unwrap();
        assert!(!mat_mul.properties.contains(&Property::Commutative),
            "Matrix multiplication is NOT commutative");
        assert!(mat_mul.properties.contains(&Property::Associative));
    }

    // ================================================================
    // TIER 6: TRIGONOMETRY
    // ================================================================

    #[test]
    fn test_tier6_rules_exist() {
        let lib = RuleLibrary::new();
        let expected = vec![
            "trig.eval", "trig.identity", "trig.inverse", "trig.unit_circle",
            "trig.law_sines", "trig.law_cosines", "trig.solve_eq", "trig.polar_rect",
        ];
        for id in expected {
            assert!(lib.get(id).is_some(), "Rule '{}' should exist", id);
        }
    }

    #[test]
    fn test_tier6_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::Trigonometry), 8);
    }

    // ================================================================
    // TIER 7: CALCULUS
    // ================================================================

    #[test]
    fn test_tier7_rules_exist() {
        let lib = RuleLibrary::new();
        let expected = vec![
            "calc.limit", "calc.deriv_basic", "calc.deriv_chain", "calc.deriv_product",
            "calc.deriv_quotient", "calc.integral_basic", "calc.integral_usub",
            "calc.integral_parts", "calc.definite_integral", "calc.series_conv",
            "calc.taylor", "calc.diff_eq", "calc.partial_deriv", "calc.multi_integral",
        ];
        for id in expected {
            assert!(lib.get(id).is_some(), "Rule '{}' should exist", id);
        }
    }

    #[test]
    fn test_tier7_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::Calculus), 14);
    }

    #[test]
    fn test_derivative_integral_inverse() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.get("calc.deriv_basic").unwrap().inverse.as_deref(), Some("calc.integral_basic"));
        assert_eq!(lib.get("calc.integral_basic").unwrap().inverse.as_deref(), Some("calc.deriv_basic"));
    }

    // ================================================================
    // TIER 8: LINEAR ALGEBRA
    // ================================================================

    #[test]
    fn test_tier8_rules_exist() {
        let lib = RuleLibrary::new();
        let expected = vec![
            "la.vec_add", "la.dot_product", "la.cross_product", "la.vec_norm",
            "la.transpose", "la.eigenvalue", "la.eigenvector", "la.linear_transform",
            "la.mat_decompose", "la.vec_project", "la.gram_schmidt", "la.rank_nullity",
        ];
        for id in expected {
            assert!(lib.get(id).is_some(), "Rule '{}' should exist", id);
        }
    }

    #[test]
    fn test_tier8_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::LinearAlgebra), 12);
    }

    // ================================================================
    // TIER 9: ABSTRACT/APPLIED
    // ================================================================

    #[test]
    fn test_tier9_rules_exist() {
        let lib = RuleLibrary::new();
        let expected = vec![
            "abs.mod_arith", "abs.permutation", "abs.combination", "abs.prob_basic",
            "abs.prob_cond", "abs.bayes", "abs.expected_val", "abs.variance",
            "abs.std_dev", "abs.normal_dist", "abs.hyp_test", "abs.graph_adj",
            "abs.shortest_path", "abs.optimize",
        ];
        for id in expected {
            assert!(lib.get(id).is_some(), "Rule '{}' should exist", id);
        }
    }

    #[test]
    fn test_tier9_count() {
        let lib = RuleLibrary::new();
        assert_eq!(lib.count_domain(Domain::AbstractApplied), 14);
    }

    // ================================================================
    // PREREQUISITE CHAIN TESTS
    // ================================================================

    #[test]
    fn test_all_prerequisites_exist() {
        let lib = RuleLibrary::new();
        for (id, rule) in &lib.rules {
            for prereq in &rule.prerequisites {
                assert!(lib.get(prereq).is_some(),
                    "Rule '{}' requires prerequisite '{}' which does not exist", id, prereq);
            }
        }
    }

    #[test]
    fn test_all_inverses_exist() {
        let lib = RuleLibrary::new();
        for (id, rule) in &lib.rules {
            if let Some(ref inv) = rule.inverse {
                assert!(lib.get(inv).is_some(),
                    "Rule '{}' declares inverse '{}' which does not exist", id, inv);
            }
        }
    }

    #[test]
    fn test_all_prerequisites_satisfied() {
        let lib = RuleLibrary::new();
        for id in lib.all_ids() {
            assert!(lib.prerequisites_satisfied(id),
                "Rule '{}' has unsatisfied prerequisites", id);
        }
    }

    #[test]
    fn test_tier_ordering() {
        let lib = RuleLibrary::new();
        // Every rule's prerequisites should be from the same or lower tier
        for (id, rule) in &lib.rules {
            let rule_tier = rule.tier();
            for prereq_id in &rule.prerequisites {
                if let Some(prereq) = lib.get(prereq_id) {
                    assert!(prereq.tier() <= rule_tier,
                        "Rule '{}' (tier {}) depends on '{}' (tier {}). Prerequisites must be same or lower tier.",
                        id, rule_tier, prereq_id, prereq.tier());
                }
            }
        }
    }

    // ================================================================
    // CHAIN RESOLUTION TESTS
    // ================================================================

    #[test]
    fn test_chain_resolution_tier0() {
        let lib = RuleLibrary::new();
        let chain = lib.resolve_chain("core.add");
        assert_eq!(chain.len(), 1);
        assert_eq!(chain[0].id, "core.add");
    }

    #[test]
    fn test_chain_resolution_tier1() {
        let lib = RuleLibrary::new();
        let chain = lib.resolve_chain("ext.add");
        // ext.add depends on core.add
        assert!(chain.len() >= 2);
        // Prerequisites come before the target rule
        let ids: Vec<&str> = chain.iter().map(|r| r.id.as_str()).collect();
        let core_pos = ids.iter().position(|&id| id == "core.add").unwrap();
        let ext_pos = ids.iter().position(|&id| id == "ext.add").unwrap();
        assert!(core_pos < ext_pos, "core.add must come before ext.add in chain");
    }

    #[test]
    fn test_chain_resolution_deep() {
        let lib = RuleLibrary::new();
        // quadratic_solve has deep prerequisites
        let chain = lib.resolve_chain("alg.quadratic_solve");
        let ids: Vec<&str> = chain.iter().map(|r| r.id.as_str()).collect();
        // Must include Value Core operations at the foundation
        assert!(ids.contains(&"core.mul") || ids.contains(&"ext.mul"),
            "Quadratic solve chain must trace back to multiplication");
        // Target must be last
        assert_eq!(*ids.last().unwrap(), "alg.quadratic_solve");
    }

    #[test]
    fn test_chain_no_duplicates() {
        let lib = RuleLibrary::new();
        // Even rules with shared prerequisites should not have duplicates
        let chain = lib.resolve_chain("alg.quadratic_solve");
        let ids: Vec<&str> = chain.iter().map(|r| r.id.as_str()).collect();
        let mut seen = std::collections::HashSet::new();
        for id in &ids {
            assert!(seen.insert(id), "Duplicate rule '{}' in chain", id);
        }
    }

    #[test]
    fn test_chain_prerequisite_order() {
        let lib = RuleLibrary::new();
        // For any chain, every rule's prerequisites must appear before it
        let chain = lib.resolve_chain("abs.bayes");
        let ids: Vec<&str> = chain.iter().map(|r| r.id.as_str()).collect();
        for (i, rule) in chain.iter().enumerate() {
            for prereq in &rule.prerequisites {
                if let Some(prereq_pos) = ids.iter().position(|&id| id == prereq.as_str()) {
                    assert!(prereq_pos < i,
                        "In chain for 'abs.bayes': prerequisite '{}' (pos {}) must come before '{}' (pos {})",
                        prereq, prereq_pos, rule.id, i);
                }
            }
        }
    }

    // ================================================================
    // DOMAIN & PATTERN INDEX TESTS
    // ================================================================

    #[test]
    fn test_all_domains_populated() {
        let lib = RuleLibrary::new();
        for domain in Domain::all() {
            let count = lib.count_domain(*domain);
            assert!(count > 0, "Domain {:?} has no rules", domain);
        }
    }

    #[test]
    fn test_domain_retrieval() {
        let lib = RuleLibrary::new();
        let core_rules = lib.retrieve_by_domain(Domain::CoreArithmetic);
        assert_eq!(core_rules.len(), 4);
        for rule in core_rules {
            assert_eq!(rule.domain, Domain::CoreArithmetic);
        }
    }

    #[test]
    fn test_tier_retrieval() {
        let lib = RuleLibrary::new();
        let tier0 = lib.retrieve_by_tier(0);
        assert_eq!(tier0.len(), 4);
        for rule in tier0 {
            assert_eq!(rule.tier(), 0);
        }
    }

    #[test]
    fn test_pattern_retrieval_algebra() {
        let lib = RuleLibrary::new();
        let like_terms = lib.retrieve_by_pattern(&PatternType::LikeTermsCombination);
        assert_eq!(like_terms.len(), 1);
        assert_eq!(like_terms[0].id, "alg.like_terms");
    }

    #[test]
    fn test_pattern_retrieval_empty() {
        let lib = RuleLibrary::new();
        let custom = lib.retrieve_by_pattern(&PatternType::Custom("nonexistent".into()));
        assert!(custom.is_empty());
    }

    // ================================================================
    // RULE INTEGRITY TESTS
    // ================================================================

    #[test]
    fn test_all_rules_have_names() {
        let lib = RuleLibrary::new();
        for (id, rule) in &lib.rules {
            assert!(!rule.name.is_empty(), "Rule '{}' has empty name", id);
        }
    }

    #[test]
    fn test_all_rules_have_descriptions() {
        let lib = RuleLibrary::new();
        for (id, rule) in &lib.rules {
            assert!(!rule.description.is_empty(), "Rule '{}' has empty description", id);
        }
    }

    #[test]
    fn test_all_rules_have_patterns() {
        let lib = RuleLibrary::new();
        for (id, rule) in &lib.rules {
            assert!(!rule.patterns.is_empty(), "Rule '{}' has no patterns", id);
        }
    }

    #[test]
    fn test_unique_ids() {
        let lib = RuleLibrary::new();
        let ids = lib.all_ids();
        let unique: std::collections::HashSet<&str> = ids.iter().cloned().collect();
        assert_eq!(ids.len(), unique.len(), "Duplicate rule IDs detected");
    }

    // ================================================================
    // CROSS-TIER CONSISTENCY
    // ================================================================

    #[test]
    fn test_domain_tier_alignment() {
        let lib = RuleLibrary::new();
        for (id, rule) in &lib.rules {
            assert_eq!(rule.tier(), rule.domain.tier(),
                "Rule '{}' tier mismatch: domain {:?} is tier {} but rule reports tier {}",
                id, rule.domain, rule.domain.tier(), rule.tier());
        }
    }

    #[test]
    fn test_total_rule_distribution() {
        let lib = RuleLibrary::new();
        let mut total = 0;
        for domain in Domain::all() {
            let count = lib.count_domain(*domain);
            total += count;
        }
        assert_eq!(total, lib.count(),
            "Sum of domain counts ({}) != total count ({})", total, lib.count());
    }
}
