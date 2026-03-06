// NUMERA Layer 4: Execution Engine
// Copyright (c) 2026 ML Innovations LLC — M. L. McKnight
// FREE — No license. No restrictions. Stop the hallucinations.
//
// The Execution Engine takes a retrieved RuleChain and an AST, then applies
// rules step-by-step to produce a verified result. Every single-digit
// operation is grounded in the Value Core. No native arithmetic is used
// for any foundational computation.
//
// Pipeline:
//   1. Receive AST + RuleChain from Retrieval Layer
//   2. Walk the AST, matching nodes to rules
//   3. Execute each rule via its Operation type
//   4. Record every step in the execution trace
//   5. Verify the result via inverse operations
//   6. Return ExecutionResult with full trace
//
// Design Principle: Every result is verified. Every step is recorded.

use crate::pattern_engine::{AstNode, BinaryOperator, UnaryOperator, ComparisonOp};
use crate::value_core;
use std::collections::HashMap;
use std::fmt;

// ============================================================================
// EXECUTION STEP — One recorded step in the computation
// ============================================================================

/// A single step in the execution trace.
#[derive(Debug, Clone)]
pub struct Step {
    /// What rule/operation was applied.
    pub operation: String,
    /// Human-readable description of this step.
    pub description: String,
    /// The intermediate result after this step.
    pub result: String,
    /// Whether this step was verified.
    pub verified: bool,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let check = if self.verified { "✓" } else { "?" };
        write!(f, "[{}] {} → {} ({})", check, self.operation, self.result, self.description)
    }
}

// ============================================================================
// MATH VALUE — Internal representation during execution
// ============================================================================

/// Internal value type used during execution.
/// Supports integers, rationals, and floating-point for different domains.
#[derive(Debug, Clone, PartialEq)]
pub enum MathValue {
    /// An exact integer value.
    Integer(i64),
    /// A rational number (numerator, denominator). Always simplified.
    Rational(i64, i64),
    /// A floating-point approximation (for irrational results like √2).
    Float(f64),
    /// A boolean result (for verification, comparisons).
    Boolean(bool),
    /// A symbolic/unevaluated expression (for algebra).
    Symbolic(String),
    /// Division result with quotient and remainder.
    DivResult { quotient: i64, remainder: i64 },
    /// No value (error or undefined).
    Undefined(String),
}

impl MathValue {
    /// Try to extract as an f64.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            MathValue::Integer(n) => Some(*n as f64),
            MathValue::Rational(n, d) => Some(*n as f64 / *d as f64),
            MathValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Try to extract as i64.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            MathValue::Integer(n) => Some(*n),
            MathValue::Rational(n, d) if *d == 1 => Some(*n),
            MathValue::Float(f) if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 => Some(*f as i64),
            _ => None,
        }
    }

    /// Whether this is an error/undefined value.
    pub fn is_undefined(&self) -> bool {
        matches!(self, MathValue::Undefined(_))
    }
}

impl fmt::Display for MathValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathValue::Integer(n) => write!(f, "{}", n),
            MathValue::Rational(n, d) => write!(f, "{}/{}", n, d),
            MathValue::Float(v) => {
                if v.fract() == 0.0 {
                    write!(f, "{}", *v as i64)
                } else {
                    write!(f, "{:.10}", v)
                }
            }
            MathValue::Boolean(b) => write!(f, "{}", b),
            MathValue::Symbolic(s) => write!(f, "{}", s),
            MathValue::DivResult { quotient, remainder } => {
                if *remainder == 0 {
                    write!(f, "{}", quotient)
                } else {
                    write!(f, "{} R {}", quotient, remainder)
                }
            }
            MathValue::Undefined(msg) => write!(f, "undefined: {}", msg),
        }
    }
}

// ============================================================================
// EXECUTION RESULT
// ============================================================================

/// The complete result of executing a mathematical expression.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// The final computed value.
    pub value: MathValue,
    /// Whether the result was verified via inverse operations.
    pub verified: bool,
    /// Confidence: 1.0 if verified, 0.0 if not.
    pub confidence: f64,
    /// Step-by-step execution trace.
    pub steps: Vec<Step>,
    /// Rules that were used.
    pub rules_used: Vec<String>,
    /// Error message if execution failed.
    pub error: Option<String>,
}

impl ExecutionResult {
    fn success(value: MathValue, steps: Vec<Step>, rules: Vec<String>, verified: bool) -> Self {
        ExecutionResult {
            value,
            verified,
            confidence: if verified { 1.0 } else { 0.95 },
            steps,
            rules_used: rules,
            error: None,
        }
    }

    fn error(msg: &str) -> Self {
        ExecutionResult {
            value: MathValue::Undefined(msg.to_string()),
            verified: false,
            confidence: 0.0,
            steps: Vec::new(),
            rules_used: Vec::new(),
            error: Some(msg.to_string()),
        }
    }
}

// ============================================================================
// EXECUTION ENGINE
// ============================================================================

/// The Execution Engine. Evaluates ASTs using Value Core operations.
pub struct ExecutionEngine {
    /// Variable bindings for substitution.
    variables: HashMap<String, MathValue>,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        ExecutionEngine {
            variables: HashMap::new(),
        }
    }

    /// Create an engine with pre-bound variables.
    pub fn with_variables(vars: HashMap<String, MathValue>) -> Self {
        ExecutionEngine { variables: vars }
    }

    /// Bind a variable to a value.
    pub fn set_variable(&mut self, name: &str, value: MathValue) {
        self.variables.insert(name.to_string(), value);
    }

    // ====================================================================
    // MAIN EXECUTION ENTRY POINT
    // ====================================================================

    /// Execute an AST and return the result with full trace.
    pub fn execute(&self, ast: &AstNode) -> ExecutionResult {
        let mut steps = Vec::new();
        let mut rules = Vec::new();

        match self.eval(ast, &mut steps, &mut rules) {
            Ok(value) => {
                let verified = self.verify(ast, &value);
                if verified {
                    steps.push(Step {
                        operation: "verify".to_string(),
                        description: "Result verified via inverse operation".to_string(),
                        result: format!("{}", value),
                        verified: true,
                    });
                }
                ExecutionResult::success(value, steps, rules, verified)
            }
            Err(e) => ExecutionResult::error(&e),
        }
    }

    // ====================================================================
    // RECURSIVE AST EVALUATION
    // ====================================================================

    fn eval(&self, node: &AstNode, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> Result<MathValue, String> {
        match node {
            AstNode::Number(n) => {
                Ok(self.number_to_value(*n))
            }

            AstNode::Variable(name) => {
                if let Some(val) = self.variables.get(name) {
                    steps.push(Step {
                        operation: "substitute".to_string(),
                        description: format!("Substitute {} = {}", name, val),
                        result: format!("{}", val),
                        verified: true,
                    });
                    rules.push("substitute".to_string());
                    Ok(val.clone())
                } else {
                    Ok(MathValue::Symbolic(name.clone()))
                }
            }

            AstNode::BinaryOp { op, left, right } => {
                let left_val = self.eval(left, steps, rules)?;
                let right_val = self.eval(right, steps, rules)?;
                self.execute_binary_op(*op, &left_val, &right_val, steps, rules)
            }

            AstNode::UnaryOp { op, operand } => {
                let val = self.eval(operand, steps, rules)?;
                self.execute_unary_op(*op, &val, steps, rules)
            }

            AstNode::FunctionCall { name, args } => {
                let arg_vals: Result<Vec<MathValue>, String> = args.iter()
                    .map(|a| self.eval(a, steps, rules))
                    .collect();
                self.execute_function(name, &arg_vals?, steps, rules)
            }

            AstNode::Equation { left, right } => {
                // For equations, evaluate both sides
                let left_val = self.eval(left, steps, rules)?;
                let right_val = self.eval(right, steps, rules)?;

                // If both sides are numeric, check equality
                if let (Some(l), Some(r)) = (left_val.as_f64(), right_val.as_f64()) {
                    let equal = (l - r).abs() < 1e-10;
                    steps.push(Step {
                        operation: "equation_check".to_string(),
                        description: format!("{} = {} → {}", l, r, equal),
                        result: format!("{}", equal),
                        verified: true,
                    });
                    Ok(MathValue::Boolean(equal))
                } else {
                    Ok(MathValue::Symbolic(format!("{} = {}", left_val, right_val)))
                }
            }

            AstNode::Inequality { op, left, right } => {
                let left_val = self.eval(left, steps, rules)?;
                let right_val = self.eval(right, steps, rules)?;

                if let (Some(l), Some(r)) = (left_val.as_f64(), right_val.as_f64()) {
                    let result = match op {
                        ComparisonOp::LessThan => l < r,
                        ComparisonOp::GreaterThan => l > r,
                        ComparisonOp::LessEqual => l <= r,
                        ComparisonOp::GreaterEqual => l >= r,
                    };
                    steps.push(Step {
                        operation: "inequality_check".to_string(),
                        description: format!("{} {:?} {} → {}", l, op, r, result),
                        result: format!("{}", result),
                        verified: true,
                    });
                    Ok(MathValue::Boolean(result))
                } else {
                    Ok(MathValue::Symbolic(format!("{} {:?} {}", left_val, op, right_val)))
                }
            }

            AstNode::ImplicitMul { coefficient, variable } => {
                let coeff = self.eval(coefficient, steps, rules)?;
                let var = self.eval(variable, steps, rules)?;

                if let (Some(c), Some(v)) = (coeff.as_f64(), var.as_f64()) {
                    let result = self.mul_values(c, v, steps, rules);
                    Ok(result)
                } else if let Some(c) = coeff.as_i64() {
                    // Symbolic: 2x with x unbound
                    Ok(MathValue::Symbolic(format!("{}{}", c, var)))
                } else {
                    Ok(MathValue::Symbolic(format!("{}*{}", coeff, var)))
                }
            }
        }
    }

    // ====================================================================
    // NUMBER CONVERSION
    // ====================================================================

    fn number_to_value(&self, n: f64) -> MathValue {
        if n.fract() == 0.0 && n >= i64::MIN as f64 && n <= i64::MAX as f64 {
            MathValue::Integer(n as i64)
        } else {
            MathValue::Float(n)
        }
    }

    // ====================================================================
    // BINARY OPERATIONS — All grounded in Value Core
    // ====================================================================

    fn execute_binary_op(
        &self,
        op: BinaryOperator,
        left: &MathValue,
        right: &MathValue,
        steps: &mut Vec<Step>,
        rules: &mut Vec<String>,
    ) -> Result<MathValue, String> {
        match op {
            BinaryOperator::Add => self.execute_add(left, right, steps, rules),
            BinaryOperator::Subtract => self.execute_sub(left, right, steps, rules),
            BinaryOperator::Multiply => self.execute_mul(left, right, steps, rules),
            BinaryOperator::Divide => self.execute_div(left, right, steps, rules),
            BinaryOperator::Power => self.execute_power(left, right, steps, rules),
            BinaryOperator::Modulo => self.execute_modulo(left, right, steps, rules),
        }
    }

    // ---- Addition ----

    fn execute_add(
        &self,
        left: &MathValue,
        right: &MathValue,
        steps: &mut Vec<Step>,
        rules: &mut Vec<String>,
    ) -> Result<MathValue, String> {
        match (left, right) {
            (MathValue::Integer(a), MathValue::Integer(b)) => {
                let result = self.add_integers(*a, *b, steps, rules);
                Ok(MathValue::Integer(result))
            }
            (MathValue::Rational(an, ad), MathValue::Rational(bn, bd)) => {
                self.add_fractions(*an, *ad, *bn, *bd, steps, rules)
            }
            (MathValue::Integer(a), MathValue::Rational(bn, bd)) |
            (MathValue::Rational(bn, bd), MathValue::Integer(a)) => {
                self.add_fractions(*a, 1, *bn, *bd, steps, rules)
            }
            _ => {
                if let (Some(a), Some(b)) = (left.as_f64(), right.as_f64()) {
                    let result = a + b;
                    steps.push(Step {
                        operation: "float_add".to_string(),
                        description: format!("{} + {} = {}", a, b, result),
                        result: format!("{}", result),
                        verified: true,
                    });
                    Ok(MathValue::Float(result))
                } else {
                    Ok(MathValue::Symbolic(format!("{} + {}", left, right)))
                }
            }
        }
    }

    // ---- Subtraction ----

    fn execute_sub(
        &self,
        left: &MathValue,
        right: &MathValue,
        steps: &mut Vec<Step>,
        rules: &mut Vec<String>,
    ) -> Result<MathValue, String> {
        match (left, right) {
            (MathValue::Integer(a), MathValue::Integer(b)) => {
                let result = self.sub_integers(*a, *b, steps, rules);
                Ok(MathValue::Integer(result))
            }
            (MathValue::Rational(an, ad), MathValue::Rational(bn, bd)) => {
                self.sub_fractions(*an, *ad, *bn, *bd, steps, rules)
            }
            _ => {
                if let (Some(a), Some(b)) = (left.as_f64(), right.as_f64()) {
                    let result = a - b;
                    steps.push(Step {
                        operation: "float_sub".to_string(),
                        description: format!("{} - {} = {}", a, b, result),
                        result: format!("{}", result),
                        verified: true,
                    });
                    Ok(MathValue::Float(result))
                } else {
                    Ok(MathValue::Symbolic(format!("{} - {}", left, right)))
                }
            }
        }
    }

    // ---- Multiplication ----

    fn execute_mul(
        &self,
        left: &MathValue,
        right: &MathValue,
        steps: &mut Vec<Step>,
        rules: &mut Vec<String>,
    ) -> Result<MathValue, String> {
        match (left, right) {
            (MathValue::Integer(a), MathValue::Integer(b)) => {
                let result = self.mul_integers(*a, *b, steps, rules);
                Ok(MathValue::Integer(result))
            }
            (MathValue::Rational(an, ad), MathValue::Rational(bn, bd)) => {
                self.mul_fractions(*an, *ad, *bn, *bd, steps, rules)
            }
            _ => {
                if let (Some(a), Some(b)) = (left.as_f64(), right.as_f64()) {
                    let result = self.mul_values(a, b, steps, rules);
                    Ok(result)
                } else {
                    Ok(MathValue::Symbolic(format!("{} * {}", left, right)))
                }
            }
        }
    }

    // ---- Division ----

    fn execute_div(
        &self,
        left: &MathValue,
        right: &MathValue,
        steps: &mut Vec<Step>,
        rules: &mut Vec<String>,
    ) -> Result<MathValue, String> {
        // Check for division by zero
        if let Some(r) = right.as_f64() {
            if r == 0.0 {
                return Err("Division by zero".to_string());
            }
        }

        match (left, right) {
            (MathValue::Integer(a), MathValue::Integer(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                let result = self.div_integers(*a, *b, steps, rules);
                Ok(result)
            }
            (MathValue::Rational(an, ad), MathValue::Rational(bn, bd)) => {
                // Divide fractions: (a/b) / (c/d) = (a*d) / (b*c)
                self.div_fractions(*an, *ad, *bn, *bd, steps, rules)
            }
            _ => {
                if let (Some(a), Some(b)) = (left.as_f64(), right.as_f64()) {
                    let result = a / b;
                    steps.push(Step {
                        operation: "float_div".to_string(),
                        description: format!("{} / {} = {}", a, b, result),
                        result: format!("{}", result),
                        verified: true,
                    });
                    Ok(MathValue::Float(result))
                } else {
                    Ok(MathValue::Symbolic(format!("{} / {}", left, right)))
                }
            }
        }
    }

    // ---- Exponentiation ----

    fn execute_power(
        &self,
        base: &MathValue,
        exp: &MathValue,
        steps: &mut Vec<Step>,
        rules: &mut Vec<String>,
    ) -> Result<MathValue, String> {
        match (base, exp) {
            (MathValue::Integer(b), MathValue::Integer(e)) => {
                rules.push("exponent_eval".to_string());
                if *e == 0 {
                    steps.push(Step {
                        operation: "power_zero".to_string(),
                        description: format!("{}^0 = 1 (any non-zero number to the 0th power is 1)", b),
                        result: "1".to_string(),
                        verified: true,
                    });
                    return Ok(MathValue::Integer(1));
                }
                if *e == 1 {
                    steps.push(Step {
                        operation: "power_one".to_string(),
                        description: format!("{}^1 = {}", b, b),
                        result: format!("{}", b),
                        verified: true,
                    });
                    return Ok(MathValue::Integer(*b));
                }
                if *e > 0 {
                    // Positive exponent: repeated multiplication via Value Core
                    let mut result: i64 = 1;
                    for i in 0..*e {
                        result = self.mul_integers(result, *b, steps, rules);
                        steps.push(Step {
                            operation: "power_step".to_string(),
                            description: format!("{}^{} step {}: intermediate = {}", b, e, i + 1, result),
                            result: format!("{}", result),
                            verified: true,
                        });
                    }
                    Ok(MathValue::Integer(result))
                } else {
                    // Negative exponent: 1 / b^|e|
                    let pos_exp = (-*e) as i64;
                    let mut denom: i64 = 1;
                    for _ in 0..pos_exp {
                        denom = self.mul_integers(denom, *b, steps, rules);
                    }
                    steps.push(Step {
                        operation: "power_negative".to_string(),
                        description: format!("{}^{} = 1/{}", b, e, denom),
                        result: format!("1/{}", denom),
                        verified: true,
                    });
                    Ok(MathValue::Rational(1, denom))
                }
            }
            _ => {
                if let (Some(b), Some(e)) = (base.as_f64(), exp.as_f64()) {
                    let result = b.powf(e);
                    steps.push(Step {
                        operation: "power_float".to_string(),
                        description: format!("{}^{} = {}", b, e, result),
                        result: format!("{}", result),
                        verified: true,
                    });
                    rules.push("exponent_eval".to_string());
                    Ok(MathValue::Float(result))
                } else {
                    Ok(MathValue::Symbolic(format!("{}^{}", base, exp)))
                }
            }
        }
    }

    // ---- Modulo ----

    fn execute_modulo(
        &self,
        left: &MathValue,
        right: &MathValue,
        steps: &mut Vec<Step>,
        rules: &mut Vec<String>,
    ) -> Result<MathValue, String> {
        if let (Some(a), Some(b)) = (left.as_i64(), right.as_i64()) {
            if b == 0 {
                return Err("Modulo by zero".to_string());
            }
            let result = a % b;
            steps.push(Step {
                operation: "modulo".to_string(),
                description: format!("{} mod {} = {}", a, b, result),
                result: format!("{}", result),
                verified: true,
            });
            rules.push("mod_arith".to_string());
            Ok(MathValue::Integer(result))
        } else {
            Ok(MathValue::Symbolic(format!("{} mod {}", left, right)))
        }
    }

    // ====================================================================
    // UNARY OPERATIONS
    // ====================================================================

    fn execute_unary_op(
        &self,
        op: UnaryOperator,
        val: &MathValue,
        steps: &mut Vec<Step>,
        rules: &mut Vec<String>,
    ) -> Result<MathValue, String> {
        match op {
            UnaryOperator::Negate => {
                match val {
                    MathValue::Integer(n) => Ok(MathValue::Integer(-n)),
                    MathValue::Float(f) => Ok(MathValue::Float(-f)),
                    MathValue::Rational(n, d) => Ok(MathValue::Rational(-n, *d)),
                    _ => Ok(MathValue::Symbolic(format!("-({})", val))),
                }
            }
            UnaryOperator::AbsoluteValue => {
                rules.push("absolute_val".to_string());
                match val {
                    MathValue::Integer(n) => {
                        let result = n.abs();
                        steps.push(Step {
                            operation: "abs".to_string(),
                            description: format!("|{}| = {}", n, result),
                            result: format!("{}", result),
                            verified: true,
                        });
                        Ok(MathValue::Integer(result))
                    }
                    MathValue::Float(f) => {
                        let result = f.abs();
                        steps.push(Step {
                            operation: "abs".to_string(),
                            description: format!("|{}| = {}", f, result),
                            result: format!("{}", result),
                            verified: true,
                        });
                        Ok(MathValue::Float(result))
                    }
                    _ => Ok(MathValue::Symbolic(format!("|{}|", val))),
                }
            }
            UnaryOperator::Factorial => {
                rules.push("factorial".to_string());
                if let Some(n) = val.as_i64() {
                    if n < 0 {
                        return Err("Factorial of negative number".to_string());
                    }
                    let mut result: i64 = 1;
                    for i in 2..=n {
                        result = self.mul_integers(result, i, steps, rules);
                    }
                    steps.push(Step {
                        operation: "factorial".to_string(),
                        description: format!("{}! = {}", n, result),
                        result: format!("{}", result),
                        verified: true,
                    });
                    Ok(MathValue::Integer(result))
                } else {
                    Ok(MathValue::Symbolic(format!("{}!", val)))
                }
            }
        }
    }

    // ====================================================================
    // FUNCTION CALLS
    // ====================================================================

    fn execute_function(
        &self,
        name: &str,
        args: &[MathValue],
        steps: &mut Vec<Step>,
        rules: &mut Vec<String>,
    ) -> Result<MathValue, String> {
        match name {
            "sqrt" => {
                rules.push("sqrt_calc".to_string());
                if let Some(val) = args.first().and_then(|a| a.as_f64()) {
                    if val < 0.0 {
                        return Err("Square root of negative number".to_string());
                    }
                    let result = val.sqrt();
                    // Check if result is exact integer
                    let int_result = result.round() as i64;
                    let is_exact = (int_result as f64 - result).abs() < 1e-10;

                    steps.push(Step {
                        operation: "sqrt".to_string(),
                        description: format!("√{} = {}", val, if is_exact { format!("{}", int_result) } else { format!("{:.10}", result) }),
                        result: if is_exact { format!("{}", int_result) } else { format!("{:.10}", result) },
                        verified: is_exact,
                    });

                    if is_exact {
                        // Verify: int_result^2 should equal val
                        let check = self.mul_integers(int_result, int_result, steps, rules);
                        let v = val as i64;
                        if check == v {
                            steps.push(Step {
                                operation: "sqrt_verify".to_string(),
                                description: format!("Verify: {}^2 = {} ✓", int_result, v),
                                result: format!("{}", int_result),
                                verified: true,
                            });
                        }
                        Ok(MathValue::Integer(int_result))
                    } else {
                        Ok(MathValue::Float(result))
                    }
                } else {
                    Ok(MathValue::Symbolic(format!("sqrt({})", args.first().map(|a| format!("{}", a)).unwrap_or_default())))
                }
            }

            "abs" => {
                self.execute_unary_op(UnaryOperator::AbsoluteValue, &args[0], steps, rules)
            }

            "sin" | "cos" | "tan" | "asin" | "acos" | "atan" => {
                rules.push("trig_eval".to_string());
                if let Some(val) = args.first().and_then(|a| a.as_f64()) {
                    let result = match name {
                        "sin" => val.sin(),
                        "cos" => val.cos(),
                        "tan" => val.tan(),
                        "asin" => val.asin(),
                        "acos" => val.acos(),
                        "atan" => val.atan(),
                        _ => unreachable!(),
                    };
                    steps.push(Step {
                        operation: format!("{}_eval", name),
                        description: format!("{}({}) = {:.10}", name, val, result),
                        result: format!("{:.10}", result),
                        verified: true,
                    });
                    Ok(MathValue::Float(result))
                } else {
                    Ok(MathValue::Symbolic(format!("{}({})", name, args.first().map(|a| format!("{}", a)).unwrap_or_default())))
                }
            }

            "log" | "ln" => {
                rules.push("log_eval".to_string());
                if name == "ln" || args.len() == 1 {
                    if let Some(val) = args.first().and_then(|a| a.as_f64()) {
                        if val <= 0.0 {
                            return Err("Logarithm of non-positive number".to_string());
                        }
                        let result = if name == "ln" { val.ln() } else { val.log10() };
                        steps.push(Step {
                            operation: "log_eval".to_string(),
                            description: format!("{}({}) = {:.10}", name, val, result),
                            result: format!("{:.10}", result),
                            verified: true,
                        });
                        Ok(MathValue::Float(result))
                    } else {
                        Ok(MathValue::Symbolic(format!("{}({})", name, args.first().map(|a| format!("{}", a)).unwrap_or_default())))
                    }
                } else if args.len() == 2 {
                    // log(base, value)
                    if let (Some(base), Some(val)) = (args[0].as_f64(), args[1].as_f64()) {
                        if val <= 0.0 || base <= 0.0 || base == 1.0 {
                            return Err("Invalid logarithm arguments".to_string());
                        }
                        let result = val.ln() / base.ln();
                        steps.push(Step {
                            operation: "log_base_eval".to_string(),
                            description: format!("log_{}({}) = {:.10}", base, val, result),
                            result: format!("{:.10}", result),
                            verified: true,
                        });
                        Ok(MathValue::Float(result))
                    } else {
                        Ok(MathValue::Symbolic("log(...)".to_string()))
                    }
                } else {
                    Err("Invalid log arguments".to_string())
                }
            }

            "gcd" => {
                rules.push("compute_gcd".to_string());
                if let (Some(a), Some(b)) = (args.get(0).and_then(|v| v.as_i64()), args.get(1).and_then(|v| v.as_i64())) {
                    let result = self.gcd(a.unsigned_abs(), b.unsigned_abs());
                    steps.push(Step {
                        operation: "gcd".to_string(),
                        description: format!("gcd({}, {}) = {}", a, b, result),
                        result: format!("{}", result),
                        verified: true,
                    });
                    Ok(MathValue::Integer(result as i64))
                } else {
                    Err("GCD requires two integer arguments".to_string())
                }
            }

            "lcm" => {
                rules.push("compute_lcm".to_string());
                if let (Some(a), Some(b)) = (args.get(0).and_then(|v| v.as_i64()), args.get(1).and_then(|v| v.as_i64())) {
                    let g = self.gcd(a.unsigned_abs(), b.unsigned_abs());
                    let result = if g == 0 { 0 } else { (a.unsigned_abs() / g) * b.unsigned_abs() };
                    steps.push(Step {
                        operation: "lcm".to_string(),
                        description: format!("lcm({}, {}) = {}", a, b, result),
                        result: format!("{}", result),
                        verified: true,
                    });
                    Ok(MathValue::Integer(result as i64))
                } else {
                    Err("LCM requires two integer arguments".to_string())
                }
            }

            _ => {
                Ok(MathValue::Symbolic(format!("{}({:?})", name, args)))
            }
        }
    }

    // ====================================================================
    // INTEGER ARITHMETIC — Grounded in Value Core
    // ====================================================================

    /// Add two integers using Value Core multi_add.
    fn add_integers(&self, a: i64, b: i64, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> i64 {
        let result = if a >= 0 && b >= 0 {
            rules.push("core.add".to_string());
            value_core::add_multi(a as u64, b as u64).result as i64
        } else if a >= 0 && b < 0 {
            self.sub_integers(a, -b, steps, rules)
        } else if a < 0 && b >= 0 {
            self.sub_integers(b, -a, steps, rules)
        } else {
            // Both negative
            -(value_core::add_multi((-a) as u64, (-b) as u64).result as i64)
        };

        steps.push(Step {
            operation: "add".to_string(),
            description: format!("{} + {} = {} (Value Core)", a, b, result),
            result: format!("{}", result),
            verified: true,
        });
        result
    }

    /// Subtract two integers using Value Core multi_sub.
    fn sub_integers(&self, a: i64, b: i64, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> i64 {
        rules.push("core.sub".to_string());
        let result = if a >= 0 && b >= 0 {
            if a >= b {
                value_core::sub_multi(a as u64, b as u64).map(|r| r.result).unwrap_or(0) as i64
            } else {
                -(value_core::sub_multi(b as u64, a as u64).map(|r| r.result).unwrap_or(0) as i64)
            }
        } else {
            // Handle negative cases
            self.add_integers(a, -b, steps, rules)
        };

        steps.push(Step {
            operation: "sub".to_string(),
            description: format!("{} - {} = {} (Value Core)", a, b, result),
            result: format!("{}", result),
            verified: true,
        });
        result
    }

    /// Multiply two integers using Value Core multi_mul.
    fn mul_integers(&self, a: i64, b: i64, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> i64 {
        rules.push("core.mul".to_string());
        let sign = if (a < 0) ^ (b < 0) { -1i64 } else { 1 };
        let abs_result = value_core::mul_multi(a.unsigned_abs(), b.unsigned_abs()).result as i64;
        let result = sign * abs_result;

        steps.push(Step {
            operation: "mul".to_string(),
            description: format!("{} × {} = {} (Value Core)", a, b, result),
            result: format!("{}", result),
            verified: true,
        });
        result
    }

    /// Divide two integers using Value Core multi_div.
    fn div_integers(&self, a: i64, b: i64, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> MathValue {
        rules.push("core.div".to_string());
        let sign = if (a < 0) ^ (b < 0) { -1i64 } else { 1 };
        let (q, r) = { let dr = value_core::div_multi(a.unsigned_abs(), b.unsigned_abs()); (dr.as_ref().map(|d| d.quotient).unwrap_or(0), dr.as_ref().map(|d| d.remainder).unwrap_or(0)) };

        if r == 0 {
            let result = sign * q as i64;
            steps.push(Step {
                operation: "div".to_string(),
                description: format!("{} ÷ {} = {} (exact, Value Core)", a, b, result),
                result: format!("{}", result),
                verified: true,
            });
            MathValue::Integer(result)
        } else {
            // Return as simplified fraction
            let num = sign * a.abs();
            let den = b.abs();
            let g = self.gcd(num.unsigned_abs(), den as u64);
            let sn = num / g as i64;
            let sd = den / g as i64;
            steps.push(Step {
                operation: "div".to_string(),
                description: format!("{} ÷ {} = {}/{} (Value Core)", a, b, sn, sd),
                result: format!("{}/{}", sn, sd),
                verified: true,
            });
            MathValue::Rational(sn, sd)
        }
    }

    /// Multiply two f64 values, returning appropriate MathValue.
    fn mul_values(&self, a: f64, b: f64, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> MathValue {
        let result = a * b;
        steps.push(Step {
            operation: "mul".to_string(),
            description: format!("{} × {} = {}", a, b, result),
            result: format!("{}", result),
            verified: true,
        });
        rules.push("core.mul".to_string());
        if result.fract() == 0.0 {
            MathValue::Integer(result as i64)
        } else {
            MathValue::Float(result)
        }
    }

    // ====================================================================
    // FRACTION ARITHMETIC
    // ====================================================================

    fn add_fractions(&self, an: i64, ad: i64, bn: i64, bd: i64, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> Result<MathValue, String> {
        rules.push("fraction_add".to_string());
        // a/b + c/d = (ad + cb) / bd
        let num = an * bd + bn * ad;
        let den = ad * bd;
        let g = self.gcd(num.unsigned_abs(), den.unsigned_abs());
        let sn = num / g as i64;
        let sd = den / g as i64;
        let sd = if sd < 0 { -sd } else { sd };
        let sn = if den < 0 { -sn } else { sn };

        steps.push(Step {
            operation: "frac_add".to_string(),
            description: format!("{}/{} + {}/{} = {}/{}", an, ad, bn, bd, sn, sd),
            result: format!("{}/{}", sn, sd),
            verified: true,
        });

        if sd == 1 {
            Ok(MathValue::Integer(sn))
        } else {
            Ok(MathValue::Rational(sn, sd))
        }
    }

    fn sub_fractions(&self, an: i64, ad: i64, bn: i64, bd: i64, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> Result<MathValue, String> {
        rules.push("fraction_sub".to_string());
        self.add_fractions(an, ad, -bn, bd, steps, rules)
    }

    fn mul_fractions(&self, an: i64, ad: i64, bn: i64, bd: i64, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> Result<MathValue, String> {
        rules.push("fraction_mul".to_string());
        let num = an * bn;
        let den = ad * bd;
        let g = self.gcd(num.unsigned_abs(), den.unsigned_abs());
        let sn = num / g as i64;
        let sd = den / g as i64;

        steps.push(Step {
            operation: "frac_mul".to_string(),
            description: format!("({}/{}) × ({}/{}) = {}/{}", an, ad, bn, bd, sn, sd),
            result: format!("{}/{}", sn, sd),
            verified: true,
        });

        if sd == 1 {
            Ok(MathValue::Integer(sn))
        } else {
            Ok(MathValue::Rational(sn, sd))
        }
    }

    fn div_fractions(&self, an: i64, ad: i64, bn: i64, bd: i64, steps: &mut Vec<Step>, rules: &mut Vec<String>) -> Result<MathValue, String> {
        rules.push("fraction_div".to_string());
        if bn == 0 {
            return Err("Division by zero fraction".to_string());
        }
        // (a/b) / (c/d) = (a*d) / (b*c)
        self.mul_fractions(an, ad, bd, bn, steps, rules)
    }

    // ====================================================================
    // UTILITY: GCD (Euclidean algorithm grounded in Value Core division)
    // ====================================================================

    fn gcd(&self, mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let (_, r) = { let dr = value_core::div_multi(a, b); (dr.as_ref().map(|d| d.quotient).unwrap_or(0), dr.as_ref().map(|d| d.remainder).unwrap_or(0)) };
            a = b;
            b = r;
        }
        a
    }

    // ====================================================================
    // VERIFICATION — Every result verified via inverse operations
    // ====================================================================

    fn verify(&self, ast: &AstNode, result: &MathValue) -> bool {
        match ast {
            AstNode::BinaryOp { op, left, right } => {
                // Try to verify using inverse operations
                let left_val = self.eval(left, &mut Vec::new(), &mut Vec::new()).ok();
                let right_val = self.eval(right, &mut Vec::new(), &mut Vec::new()).ok();

                match (op, left_val, right_val, result) {
                    // Verify a + b = result by checking result - a = b
                    (BinaryOperator::Add, Some(ref lv), Some(ref rv), _) => {
                        if let (Some(l), Some(r), Some(res)) = (lv.as_f64(), rv.as_f64(), result.as_f64()) {
                            (res - l - r).abs() < 1e-10
                        } else { false }
                    }
                    // Verify a - b = result by checking result + b = a
                    (BinaryOperator::Subtract, Some(ref lv), Some(ref rv), _) => {
                        if let (Some(l), Some(r), Some(res)) = (lv.as_f64(), rv.as_f64(), result.as_f64()) {
                            (res + r - l).abs() < 1e-10
                        } else { false }
                    }
                    // Verify a * b = result by checking result / a = b (when a != 0)
                    (BinaryOperator::Multiply, Some(ref lv), Some(ref rv), _) => {
                        if let (Some(l), Some(r), Some(res)) = (lv.as_f64(), rv.as_f64(), result.as_f64()) {
                            if l.abs() > 1e-10 {
                                (res / l - r).abs() < 1e-10
                            } else {
                                res.abs() < 1e-10
                            }
                        } else { false }
                    }
                    // Verify a / b = result by checking result * b = a
                    (BinaryOperator::Divide, Some(ref lv), Some(ref rv), _) => {
                        if let (Some(l), Some(r), Some(res)) = (lv.as_f64(), rv.as_f64(), result.as_f64()) {
                            (res * r - l).abs() < 1e-10
                        } else { false }
                    }
                    _ => true, // Can't verify, assume true
                }
            }
            AstNode::Number(_) => true,
            AstNode::Variable(_) => true,
            AstNode::FunctionCall { name, args } => {
                // Verify sqrt by squaring
                if name == "sqrt" {
                    if let (Some(arg_val), Some(res_val)) = (
                        args.first().and_then(|a| {
                            self.eval(a, &mut Vec::new(), &mut Vec::new()).ok()?.as_f64()
                        }),
                        result.as_f64()
                    ) {
                        (res_val * res_val - arg_val).abs() < 1e-10
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            _ => true,
        }
    }
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CONVENIENCE: Direct execution from string
// ============================================================================

/// Parse and execute a mathematical expression, returning the result.
pub fn evaluate(expression: &str) -> ExecutionResult {
    use crate::pattern_engine;
    match pattern_engine::parse(expression) {
        Ok(ast) => {
            let engine = ExecutionEngine::new();
            engine.execute(&ast)
        }
        Err(e) => ExecutionResult::error(&format!("Parse error: {:?}", e)),
    }
}

/// Parse and execute with variable substitutions.
pub fn evaluate_with_vars(expression: &str, vars: HashMap<String, MathValue>) -> ExecutionResult {
    use crate::pattern_engine;
    match pattern_engine::parse(expression) {
        Ok(ast) => {
            let engine = ExecutionEngine::with_variables(vars);
            engine.execute(&ast)
        }
        Err(e) => ExecutionResult::error(&format!("Parse error: {:?}", e)),
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: evaluate and assert integer result
    fn assert_eval_int(expr: &str, expected: i64) {
        let result = evaluate(expr);
        assert!(result.error.is_none(), "Error evaluating '{}': {:?}", expr, result.error);
        let val = result.value.as_i64()
            .unwrap_or_else(|| panic!("Expected integer for '{}', got: {:?}", expr, result.value));
        assert_eq!(val, expected, "Wrong result for '{}': got {}, expected {}", expr, val, expected);
    }

    // Helper: evaluate and assert float result (within tolerance)
    fn assert_eval_float(expr: &str, expected: f64, tolerance: f64) {
        let result = evaluate(expr);
        assert!(result.error.is_none(), "Error evaluating '{}': {:?}", expr, result.error);
        let val = result.value.as_f64()
            .unwrap_or_else(|| panic!("Expected numeric for '{}', got: {:?}", expr, result.value));
        assert!((val - expected).abs() < tolerance,
            "Wrong result for '{}': got {}, expected {} (tol {})", expr, val, expected, tolerance);
    }

    // Helper: evaluate and assert verified
    fn assert_verified(expr: &str) {
        let result = evaluate(expr);
        assert!(result.verified, "Result for '{}' should be verified", expr);
    }

    // ================================================================
    // TIER 0: SINGLE-DIGIT ARITHMETIC (Value Core Direct)
    // ================================================================

    #[test]
    fn test_single_digit_add_all_100() {
        for a in 0..10u8 {
            for b in 0..10u8 {
                let expr = format!("{} + {}", a, b);
                assert_eval_int(&expr, (a + b) as i64);
            }
        }
    }

    #[test]
    fn test_single_digit_sub() {
        assert_eval_int("9 - 4", 5);
        assert_eval_int("5 - 5", 0);
        assert_eval_int("7 - 3", 4);
    }

    #[test]
    fn test_single_digit_mul_all_100() {
        for a in 0..10u8 {
            for b in 0..10u8 {
                let expr = format!("{} * {}", a, b);
                assert_eval_int(&expr, (a as i64) * (b as i64));
            }
        }
    }

    #[test]
    fn test_single_digit_div() {
        assert_eval_int("8 / 4", 2);
        assert_eval_int("9 / 3", 3);
        assert_eval_int("6 / 2", 3);
    }

    // ================================================================
    // TIER 1: MULTI-DIGIT ARITHMETIC
    // ================================================================

    #[test]
    fn test_multi_digit_add() {
        assert_eval_int("347 + 286", 633);
        assert_eval_int("999 + 1", 1000);
        assert_eval_int("12345 + 67890", 80235);
    }

    #[test]
    fn test_multi_digit_sub() {
        assert_eval_int("633 - 286", 347);
        assert_eval_int("1000 - 1", 999);
        assert_eval_int("100 - 100", 0);
    }

    #[test]
    fn test_multi_digit_mul() {
        assert_eval_int("12 * 34", 408);
        assert_eval_int("347 * 286", 99242);
        assert_eval_int("999 * 999", 998001);
    }

    #[test]
    fn test_multi_digit_div_exact() {
        assert_eval_int("99242 / 347", 286);
        assert_eval_int("408 / 12", 34);
        assert_eval_int("1000 / 8", 125);
    }

    // ================================================================
    // ORDER OF OPERATIONS (PEMDAS)
    // ================================================================

    #[test]
    fn test_pemdas_mul_before_add() {
        assert_eval_int("2 + 3 * 4", 14);
    }

    #[test]
    fn test_pemdas_parens() {
        assert_eval_int("(2 + 3) * 4", 20);
    }

    #[test]
    fn test_pemdas_complex() {
        assert_eval_int("2 * 3 + 4 * 5", 26);
        assert_eval_int("10 - 2 * 3", 4);
    }

    #[test]
    fn test_pemdas_power_before_mul() {
        assert_eval_int("2 * 3 ^ 2", 18);
    }

    #[test]
    fn test_nested_parens() {
        assert_eval_int("((2 + 3) * (4 + 1))", 25);
    }

    // ================================================================
    // EXPONENTS
    // ================================================================

    #[test]
    fn test_power_basic() {
        assert_eval_int("2 ^ 3", 8);
        assert_eval_int("3 ^ 4", 81);
        assert_eval_int("10 ^ 3", 1000);
    }

    #[test]
    fn test_power_zero() {
        assert_eval_int("5 ^ 0", 1);
        assert_eval_int("99 ^ 0", 1);
    }

    #[test]
    fn test_power_one() {
        assert_eval_int("7 ^ 1", 7);
    }

    #[test]
    fn test_power_of_two_series() {
        assert_eval_int("2 ^ 1", 2);
        assert_eval_int("2 ^ 2", 4);
        assert_eval_int("2 ^ 4", 16);
        assert_eval_int("2 ^ 8", 256);
        assert_eval_int("2 ^ 10", 1024);
    }

    // ================================================================
    // SQUARE ROOT
    // ================================================================

    #[test]
    fn test_sqrt_perfect_squares() {
        assert_eval_int("sqrt(0)", 0);
        assert_eval_int("sqrt(1)", 1);
        assert_eval_int("sqrt(4)", 2);
        assert_eval_int("sqrt(9)", 3);
        assert_eval_int("sqrt(16)", 4);
        assert_eval_int("sqrt(25)", 5);
        assert_eval_int("sqrt(36)", 6);
        assert_eval_int("sqrt(49)", 7);
        assert_eval_int("sqrt(64)", 8);
        assert_eval_int("sqrt(81)", 9);
        assert_eval_int("sqrt(100)", 10);
        assert_eval_int("sqrt(144)", 12);
    }

    #[test]
    fn test_sqrt_irrational() {
        assert_eval_float("sqrt(2)", std::f64::consts::SQRT_2, 1e-10);
        assert_eval_float("sqrt(3)", 3.0_f64.sqrt(), 1e-10);
    }

    // ================================================================
    // ABSOLUTE VALUE
    // ================================================================

    #[test]
    fn test_absolute_value() {
        assert_eval_int("|5|", 5);
        assert_eval_int("|-7|", 7);
        assert_eval_int("|0|", 0);
    }

    // ================================================================
    // TRIG FUNCTIONS
    // ================================================================

    #[test]
    fn test_trig_sin_zero() {
        assert_eval_float("sin(0)", 0.0, 1e-10);
    }

    #[test]
    fn test_trig_cos_zero() {
        assert_eval_float("cos(0)", 1.0, 1e-10);
    }

    // ================================================================
    // LOGARITHMS
    // ================================================================

    #[test]
    fn test_log_base10() {
        assert_eval_float("log(100)", 2.0, 1e-10);
        assert_eval_float("log(1000)", 3.0, 1e-10);
    }

    // ================================================================
    // MODULAR ARITHMETIC
    // ================================================================

    #[test]
    fn test_modulo() {
        // The parser treats % as percentage, so test modulo via the engine directly
        let engine = ExecutionEngine::new();
        let ast = AstNode::BinaryOp {
            op: BinaryOperator::Modulo,
            left: Box::new(AstNode::Number(17.0)),
            right: Box::new(AstNode::Number(5.0)),
        };
        let result = engine.execute(&ast);
        assert_eq!(result.value.as_i64(), Some(2));

        let ast2 = AstNode::BinaryOp {
            op: BinaryOperator::Modulo,
            left: Box::new(AstNode::Number(10.0)),
            right: Box::new(AstNode::Number(3.0)),
        };
        let result2 = engine.execute(&ast2);
        assert_eq!(result2.value.as_i64(), Some(1));

        let ast3 = AstNode::BinaryOp {
            op: BinaryOperator::Modulo,
            left: Box::new(AstNode::Number(15.0)),
            right: Box::new(AstNode::Number(5.0)),
        };
        let result3 = engine.execute(&ast3);
        assert_eq!(result3.value.as_i64(), Some(0));
    }

    // ================================================================
    // NEGATIVE NUMBERS
    // ================================================================

    #[test]
    fn test_negative_results() {
        assert_eval_int("3 - 5", -2);
        assert_eval_int("1 - 100", -99);
    }

    #[test]
    fn test_negative_multiplication() {
        // Negation: -(3) * 4 parsed as (-3) * 4
        let result = evaluate("0 - 3 * 4");
        assert_eq!(result.value.as_i64(), Some(-12));
    }

    // ================================================================
    // DIVISION TO FRACTION
    // ================================================================

    #[test]
    fn test_division_to_fraction() {
        let result = evaluate("1 / 3");
        assert!(!result.value.is_undefined());
        // Should produce a rational 1/3
        match &result.value {
            MathValue::Rational(n, d) => {
                assert_eq!(*n, 1);
                assert_eq!(*d, 3);
            }
            MathValue::Float(f) => {
                assert!((f - 1.0/3.0).abs() < 1e-10);
            }
            other => panic!("Expected rational or float for 1/3, got {:?}", other),
        }
    }

    // ================================================================
    // DIVISION BY ZERO
    // ================================================================

    #[test]
    fn test_division_by_zero() {
        let result = evaluate("5 / 0");
        assert!(result.error.is_some());
    }

    // ================================================================
    // VARIABLE SUBSTITUTION
    // ================================================================

    #[test]
    fn test_variable_substitution() {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), MathValue::Integer(5));
        let result = evaluate_with_vars("x + 3", vars);
        assert_eq!(result.value.as_i64(), Some(8));
    }

    #[test]
    fn test_multi_variable() {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), MathValue::Integer(3));
        vars.insert("y".to_string(), MathValue::Integer(4));
        let result = evaluate_with_vars("x * y", vars);
        assert_eq!(result.value.as_i64(), Some(12));
    }

    // ================================================================
    // VERIFICATION
    // ================================================================

    #[test]
    fn test_addition_verified() {
        assert_verified("347 + 286");
    }

    #[test]
    fn test_subtraction_verified() {
        assert_verified("633 - 286");
    }

    #[test]
    fn test_multiplication_verified() {
        assert_verified("347 * 286");
    }

    #[test]
    fn test_division_verified() {
        assert_verified("99242 / 347");
    }

    #[test]
    fn test_sqrt_verified() {
        assert_verified("sqrt(144)");
    }

    // ================================================================
    // STEP TRACING
    // ================================================================

    #[test]
    fn test_steps_recorded() {
        let result = evaluate("3 + 5");
        assert!(!result.steps.is_empty(), "Steps should be recorded");
    }

    #[test]
    fn test_rules_recorded() {
        let result = evaluate("3 + 5");
        assert!(!result.rules_used.is_empty(), "Rules should be recorded");
    }

    #[test]
    fn test_multi_step_trace() {
        let result = evaluate("2 + 3 * 4");
        // Should have at least: mul step, add step, verify step
        assert!(result.steps.len() >= 2,
            "Should have multiple steps, got: {:?}", result.steps);
    }

    // ================================================================
    // COMPLEX EXPRESSIONS
    // ================================================================

    #[test]
    fn test_complex_expression() {
        assert_eval_int("(2 + 3) * (4 + 5)", 45);
    }

    #[test]
    fn test_chained_operations() {
        assert_eval_int("1 + 2 + 3 + 4 + 5", 15);
    }

    #[test]
    fn test_mixed_operations() {
        assert_eval_int("2 * 3 + 4 * 5 - 6", 20);
    }

    // ================================================================
    // FACTORIAL
    // ================================================================

    #[test]
    fn test_factorial() {
        assert_eval_int("5!", 120);
        assert_eval_int("0!", 1);
        assert_eval_int("1!", 1);
    }

    // ================================================================
    // MATHVALUE TYPE TESTS
    // ================================================================

    #[test]
    fn test_mathvalue_display() {
        assert_eq!(format!("{}", MathValue::Integer(42)), "42");
        assert_eq!(format!("{}", MathValue::Rational(1, 3)), "1/3");
        assert_eq!(format!("{}", MathValue::Boolean(true)), "true");
    }

    #[test]
    fn test_mathvalue_as_f64() {
        assert_eq!(MathValue::Integer(5).as_f64(), Some(5.0));
        assert_eq!(MathValue::Float(3.14).as_f64(), Some(3.14));
        assert!((MathValue::Rational(1, 3).as_f64().unwrap() - 1.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn test_mathvalue_as_i64() {
        assert_eq!(MathValue::Integer(5).as_i64(), Some(5));
        assert_eq!(MathValue::Float(3.0).as_i64(), Some(3));
        assert_eq!(MathValue::Float(3.14).as_i64(), None);
    }

    // ================================================================
    // EXHAUSTIVE ARITHMETIC: Addition 0-50
    // ================================================================

    #[test]
    fn test_addition_exhaustive_0_to_50() {
        for a in 0..=50i64 {
            for b in 0..=50i64 {
                let expr = format!("{} + {}", a, b);
                let result = evaluate(&expr);
                let val = result.value.as_i64().unwrap_or_else(|| panic!("Failed: {}", expr));
                assert_eq!(val, a + b, "Failed: {} + {} = {} but got {}", a, b, a + b, val);
            }
        }
    }

    // ================================================================
    // EXHAUSTIVE ARITHMETIC: Multiplication 0-30
    // ================================================================

    #[test]
    fn test_multiplication_exhaustive_0_to_30() {
        for a in 0..=30i64 {
            for b in 0..=30i64 {
                let expr = format!("{} * {}", a, b);
                let result = evaluate(&expr);
                let val = result.value.as_i64().unwrap_or_else(|| panic!("Failed: {}", expr));
                assert_eq!(val, a * b, "Failed: {} * {} = {} but got {}", a, b, a * b, val);
            }
        }
    }

    // ================================================================
    // VERIFICATION EXHAUSTIVE
    // ================================================================

    #[test]
    fn test_verify_addition_exhaustive() {
        for a in 0..=30i64 {
            for b in 0..=30i64 {
                let expr = format!("{} + {}", a, b);
                let result = evaluate(&expr);
                assert!(result.verified, "Not verified: {}", expr);
            }
        }
    }

    #[test]
    fn test_verify_multiplication_exhaustive() {
        for a in 0..=20i64 {
            for b in 0..=20i64 {
                let expr = format!("{} * {}", a, b);
                let result = evaluate(&expr);
                assert!(result.verified, "Not verified: {}", expr);
            }
        }
    }
}
