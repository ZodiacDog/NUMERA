// NUMERA Layer 2: Pattern Engine
// Copyright (c) 2026 ML Innovations LLC — M. L. McKnight
// FREE — No license. No restrictions. Stop the hallucinations.
//
// The Pattern Engine receives raw mathematical input in standard notation
// and decomposes it into a structured Abstract Syntax Tree (AST). It then
// classifies the expression by domain and pattern type, driving the
// Retrieval Layer's rule matching.
//
// Phase 1: Standard notation only. LaTeX, natural language, and
// programmatic parsers will be added in Phase 2.
//
// Pipeline: Input String -> Tokenizer -> Parser -> AST -> Classifier -> PatternClassification

use crate::rule_library::{Domain, PatternType};
use std::fmt;

// ============================================================================
// TOKENS
// ============================================================================

/// A token produced by the tokenizer from raw input.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A numeric literal: integer or decimal
    Number(f64),
    /// A variable name (e.g., "x", "n", "theta")
    Variable(String),
    /// A named function (e.g., "sin", "cos", "log", "sqrt")
    Function(String),
    // Operators
    Plus,
    Minus,
    Star,        // *
    Slash,       // /
    Caret,       // ^ (exponentiation)
    Percent,     // %
    Bang,        // ! (factorial)
    // Grouping
    LeftParen,
    RightParen,
    // Separators
    Comma,
    Equals,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    // Special
    Pipe,        // | (absolute value)
    Underscore,  // _ (subscript, e.g., log_2)
    /// End of input
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Variable(s) => write!(f, "{}", s),
            Token::Function(s) => write!(f, "{}()", s),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Caret => write!(f, "^"),
            Token::Percent => write!(f, "%"),
            Token::Bang => write!(f, "!"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Equals => write!(f, "="),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::Pipe => write!(f, "|"),
            Token::Underscore => write!(f, "_"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

// ============================================================================
// TOKENIZER
// ============================================================================

/// Known function names that the tokenizer recognizes.
const KNOWN_FUNCTIONS: &[&str] = &[
    "sin", "cos", "tan", "asin", "acos", "atan",
    "sinh", "cosh", "tanh",
    "sqrt", "cbrt", "abs",
    "log", "ln", "exp",
    "floor", "ceil", "round",
    "gcd", "lcm",
    "det", "trace", "transpose",
    "mod",
];

/// Known constants.
const KNOWN_CONSTANTS: &[(&str, f64)] = &[
    ("pi", std::f64::consts::PI),
    ("e", std::f64::consts::E),
];

/// Tokenize a mathematical expression string into a sequence of tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // Skip whitespace
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // Numbers (integer or decimal)
        if c.is_ascii_digit() || (c == '.' && i + 1 < len && chars[i + 1].is_ascii_digit()) {
            let start = i;
            let mut has_dot = false;
            while i < len && (chars[i].is_ascii_digit() || (chars[i] == '.' && !has_dot)) {
                if chars[i] == '.' {
                    has_dot = true;
                }
                i += 1;
            }
            let num_str: String = chars[start..i].iter().collect();
            let num: f64 = num_str.parse().map_err(|_| ParseError::InvalidNumber(num_str))?;
            tokens.push(Token::Number(num));
            continue;
        }

        // Letters: variables, functions, or constants
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < len && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let lower = word.to_lowercase();

            // Check for known constants
            if let Some(&(_, val)) = KNOWN_CONSTANTS.iter().find(|&&(name, _)| name == lower) {
                tokens.push(Token::Number(val));
                continue;
            }

            // Check for known functions
            if KNOWN_FUNCTIONS.contains(&lower.as_str()) {
                tokens.push(Token::Function(lower));
                continue;
            }

            // Otherwise it's a variable
            tokens.push(Token::Variable(word));
            continue;
        }

        // Single-character tokens
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '^' => tokens.push(Token::Caret),
            '%' => tokens.push(Token::Percent),
            '!' => tokens.push(Token::Bang),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ',' => tokens.push(Token::Comma),
            '=' => tokens.push(Token::Equals),
            '|' => tokens.push(Token::Pipe),
            '<' => {
                if i + 1 < len && chars[i + 1] == '=' {
                    tokens.push(Token::LessEqual);
                    i += 1;
                } else {
                    tokens.push(Token::LessThan);
                }
            }
            '>' => {
                if i + 1 < len && chars[i + 1] == '=' {
                    tokens.push(Token::GreaterEqual);
                    i += 1;
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            _ => return Err(ParseError::UnexpectedCharacter(c, i)),
        }
        i += 1;
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

// ============================================================================
// ABSTRACT SYNTAX TREE
// ============================================================================

/// AST node representing a parsed mathematical expression.
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    /// A numeric literal.
    Number(f64),

    /// A variable reference.
    Variable(String),

    /// A binary operation: left op right.
    BinaryOp {
        op: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },

    /// A unary operation: op operand.
    UnaryOp {
        op: UnaryOperator,
        operand: Box<AstNode>,
    },

    /// A function call: name(args).
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },

    /// An equation: left = right.
    Equation {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },

    /// An inequality: left <op> right.
    Inequality {
        op: ComparisonOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },

    /// Implicit multiplication: coefficient * variable (e.g., 2x, 3n).
    ImplicitMul {
        coefficient: Box<AstNode>,
        variable: Box<AstNode>,
    },
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
    Factorial,
    AbsoluteValue,
}

/// Comparison operators for inequalities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOp {
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Power => write!(f, "^"),
            BinaryOperator::Modulo => write!(f, "mod"),
        }
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Number(n) => {
                if *n == n.floor() && n.abs() < 1e15 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            AstNode::Variable(v) => write!(f, "{}", v),
            AstNode::BinaryOp { op, left, right } => write!(f, "({} {} {})", left, op, right),
            AstNode::UnaryOp { op, operand } => match op {
                UnaryOperator::Negate => write!(f, "(-{})", operand),
                UnaryOperator::Factorial => write!(f, "{}!", operand),
                UnaryOperator::AbsoluteValue => write!(f, "|{}|", operand),
            },
            AstNode::FunctionCall { name, args } => {
                let arg_strs: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                write!(f, "{}({})", name, arg_strs.join(", "))
            }
            AstNode::Equation { left, right } => write!(f, "{} = {}", left, right),
            AstNode::Inequality { op, left, right } => {
                let sym = match op {
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::GreaterThan => ">",
                    ComparisonOp::LessEqual => "<=",
                    ComparisonOp::GreaterEqual => ">=",
                };
                write!(f, "{} {} {}", left, sym, right)
            }
            AstNode::ImplicitMul { coefficient, variable } => {
                write!(f, "{}{}", coefficient, variable)
            }
        }
    }
}

// ============================================================================
// PARSE ERRORS
// ============================================================================

/// Errors that can occur during tokenization or parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedCharacter(char, usize),
    InvalidNumber(String),
    UnexpectedToken(String),
    UnexpectedEof,
    MismatchedParentheses,
    EmptyExpression,
    InvalidFunctionArgs(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedCharacter(c, pos) => write!(f, "Unexpected character '{}' at position {}", c, pos),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::UnexpectedToken(s) => write!(f, "Unexpected token: {}", s),
            ParseError::UnexpectedEof => write!(f, "Unexpected end of input"),
            ParseError::MismatchedParentheses => write!(f, "Mismatched parentheses"),
            ParseError::EmptyExpression => write!(f, "Empty expression"),
            ParseError::InvalidFunctionArgs(s) => write!(f, "Invalid function arguments: {}", s),
        }
    }
}

// ============================================================================
// PARSER (Pratt Parser / Precedence Climbing)
// ============================================================================

/// A recursive descent parser with operator precedence for mathematical expressions.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
        let tok = self.advance();
        if std::mem::discriminant(&tok) == std::mem::discriminant(expected) {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(format!("expected {}, got {}", expected, tok)))
        }
    }

    /// Parse a complete expression, potentially an equation or inequality.
    pub fn parse_expression(&mut self) -> Result<AstNode, ParseError> {
        let left = self.parse_additive()?;

        match self.peek() {
            Token::Equals => {
                self.advance();
                let right = self.parse_additive()?;
                Ok(AstNode::Equation { left: Box::new(left), right: Box::new(right) })
            }
            Token::LessThan => {
                self.advance();
                let right = self.parse_additive()?;
                Ok(AstNode::Inequality { op: ComparisonOp::LessThan, left: Box::new(left), right: Box::new(right) })
            }
            Token::GreaterThan => {
                self.advance();
                let right = self.parse_additive()?;
                Ok(AstNode::Inequality { op: ComparisonOp::GreaterThan, left: Box::new(left), right: Box::new(right) })
            }
            Token::LessEqual => {
                self.advance();
                let right = self.parse_additive()?;
                Ok(AstNode::Inequality { op: ComparisonOp::LessEqual, left: Box::new(left), right: Box::new(right) })
            }
            Token::GreaterEqual => {
                self.advance();
                let right = self.parse_additive()?;
                Ok(AstNode::Inequality { op: ComparisonOp::GreaterEqual, left: Box::new(left), right: Box::new(right) })
            }
            _ => Ok(left),
        }
    }

    /// Parse addition and subtraction (lowest precedence arithmetic).
    fn parse_additive(&mut self) -> Result<AstNode, ParseError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = AstNode::BinaryOp { op: BinaryOperator::Add, left: Box::new(left), right: Box::new(right) };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = AstNode::BinaryOp { op: BinaryOperator::Subtract, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    /// Parse multiplication, division, modulo.
    fn parse_multiplicative(&mut self) -> Result<AstNode, ParseError> {
        let mut left = self.parse_unary()?;

        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = AstNode::BinaryOp { op: BinaryOperator::Multiply, left: Box::new(left), right: Box::new(right) };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = AstNode::BinaryOp { op: BinaryOperator::Divide, left: Box::new(left), right: Box::new(right) };
                }
                Token::Percent => {
                    self.advance();
                    // Check if this is "percent of" (e.g., 15% of 200) or modulo
                    // For now treat standalone % as percent (divide by 100)
                    left = AstNode::BinaryOp {
                        op: BinaryOperator::Divide,
                        left: Box::new(left),
                        right: Box::new(AstNode::Number(100.0)),
                    };
                }
                // Implicit multiplication: number followed by variable or paren
                // e.g., 2x, 3(x+1)
                Token::Variable(_) | Token::Function(_) | Token::LeftParen
                    if matches!(left, AstNode::Number(_) | AstNode::Variable(_) | AstNode::ImplicitMul { .. }) =>
                {
                    let right = self.parse_unary()?;
                    left = AstNode::ImplicitMul {
                        coefficient: Box::new(left),
                        variable: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    /// Parse unary operators: negation, postfix (factorial).
    fn parse_unary(&mut self) -> Result<AstNode, ParseError> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                let operand = self.parse_power()?;
                Ok(AstNode::UnaryOp { op: UnaryOperator::Negate, operand: Box::new(operand) })
            }
            Token::Pipe => {
                self.advance();
                let inner = self.parse_additive()?;
                self.expect(&Token::Pipe)?;
                Ok(AstNode::UnaryOp { op: UnaryOperator::AbsoluteValue, operand: Box::new(inner) })
            }
            _ => {
                let mut expr = self.parse_power()?;
                // Postfix factorial
                while matches!(self.peek(), Token::Bang) {
                    self.advance();
                    expr = AstNode::UnaryOp { op: UnaryOperator::Factorial, operand: Box::new(expr) };
                }
                Ok(expr)
            }
        }
    }

    /// Parse exponentiation (right-associative).
    fn parse_power(&mut self) -> Result<AstNode, ParseError> {
        let base = self.parse_primary()?;

        if matches!(self.peek(), Token::Caret) {
            self.advance();
            let exp = self.parse_unary()?; // Right-associative: recurse into unary
            Ok(AstNode::BinaryOp { op: BinaryOperator::Power, left: Box::new(base), right: Box::new(exp) })
        } else {
            Ok(base)
        }
    }

    /// Parse primary expressions: numbers, variables, functions, parenthesized groups.
    fn parse_primary(&mut self) -> Result<AstNode, ParseError> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(AstNode::Number(n))
            }
            Token::Variable(name) => {
                self.advance();
                Ok(AstNode::Variable(name))
            }
            Token::Function(name) => {
                self.advance();
                // Expect opening paren
                self.expect(&Token::LeftParen)?;
                let mut args = Vec::new();
                if !matches!(self.peek(), Token::RightParen) {
                    args.push(self.parse_additive()?);
                    while matches!(self.peek(), Token::Comma) {
                        self.advance();
                        args.push(self.parse_additive()?);
                    }
                }
                self.expect(&Token::RightParen)?;
                Ok(AstNode::FunctionCall { name, args })
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_additive()?;
                self.expect(&Token::RightParen)?;
                Ok(expr)
            }
            Token::Eof => Err(ParseError::UnexpectedEof),
            other => Err(ParseError::UnexpectedToken(format!("{}", other))),
        }
    }
}

// ============================================================================
// PUBLIC PARSE FUNCTION
// ============================================================================

/// Parse a mathematical expression string into an AST.
/// This is the main entry point for the Pattern Engine.
pub fn parse(input: &str) -> Result<AstNode, ParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ParseError::EmptyExpression);
    }
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expression()?;
    Ok(ast)
}

// ============================================================================
// PATTERN CLASSIFICATION
// ============================================================================

/// The result of classifying a parsed AST. Tells the Retrieval Layer
/// what kind of mathematical problem this is.
#[derive(Debug, Clone)]
pub struct PatternClassification {
    /// The primary mathematical domain.
    pub domain: Domain,
    /// The specific pattern type for rule matching.
    pub pattern_type: PatternType,
    /// Secondary patterns that may also apply.
    pub secondary_patterns: Vec<PatternType>,
    /// Whether the expression contains variables (algebraic vs arithmetic).
    pub has_variables: bool,
    /// Whether the expression is an equation (has = sign).
    pub is_equation: bool,
    /// Whether the expression is an inequality.
    pub is_inequality: bool,
    /// The set of unique variables found.
    pub variables: Vec<String>,
    /// The highest exponent found on any variable.
    pub max_variable_degree: u32,
    /// Names of functions used.
    pub functions_used: Vec<String>,
}

/// Classify a parsed AST into a pattern that the Retrieval Layer can match.
pub fn classify(ast: &AstNode) -> PatternClassification {
    let has_variables = contains_variables(ast);
    let is_equation = matches!(ast, AstNode::Equation { .. });
    let is_inequality = matches!(ast, AstNode::Inequality { .. });
    let variables = collect_variables(ast);
    let max_degree = max_variable_degree(ast);
    let functions = collect_functions(ast);

    let (domain, primary_pattern) = determine_pattern(ast, has_variables, is_equation, is_inequality, max_degree, &functions);
    let secondary = determine_secondary_patterns(ast, &primary_pattern);

    PatternClassification {
        domain,
        pattern_type: primary_pattern,
        secondary_patterns: secondary,
        has_variables,
        is_equation,
        is_inequality,
        variables,
        max_variable_degree: max_degree,
        functions_used: functions,
    }
}

// ============================================================================
// AST ANALYSIS HELPERS
// ============================================================================

/// Check if an AST contains any variables.
fn contains_variables(ast: &AstNode) -> bool {
    match ast {
        AstNode::Variable(_) => true,
        AstNode::Number(_) => false,
        AstNode::BinaryOp { left, right, .. } => contains_variables(left) || contains_variables(right),
        AstNode::UnaryOp { operand, .. } => contains_variables(operand),
        AstNode::FunctionCall { args, .. } => args.iter().any(contains_variables),
        AstNode::Equation { left, right } => contains_variables(left) || contains_variables(right),
        AstNode::Inequality { left, right, .. } => contains_variables(left) || contains_variables(right),
        AstNode::ImplicitMul { coefficient, variable } => contains_variables(coefficient) || contains_variables(variable),
    }
}

/// Collect all unique variable names from an AST.
fn collect_variables(ast: &AstNode) -> Vec<String> {
    let mut vars = Vec::new();
    collect_variables_inner(ast, &mut vars);
    vars.sort();
    vars.dedup();
    vars
}

fn collect_variables_inner(ast: &AstNode, vars: &mut Vec<String>) {
    match ast {
        AstNode::Variable(name) => {
            if !vars.contains(name) {
                vars.push(name.clone());
            }
        }
        AstNode::Number(_) => {}
        AstNode::BinaryOp { left, right, .. } => {
            collect_variables_inner(left, vars);
            collect_variables_inner(right, vars);
        }
        AstNode::UnaryOp { operand, .. } => collect_variables_inner(operand, vars),
        AstNode::FunctionCall { args, .. } => {
            for arg in args {
                collect_variables_inner(arg, vars);
            }
        }
        AstNode::Equation { left, right } => {
            collect_variables_inner(left, vars);
            collect_variables_inner(right, vars);
        }
        AstNode::Inequality { left, right, .. } => {
            collect_variables_inner(left, vars);
            collect_variables_inner(right, vars);
        }
        AstNode::ImplicitMul { coefficient, variable } => {
            collect_variables_inner(coefficient, vars);
            collect_variables_inner(variable, vars);
        }
    }
}

/// Collect all function names used in the AST.
fn collect_functions(ast: &AstNode) -> Vec<String> {
    let mut funcs = Vec::new();
    collect_functions_inner(ast, &mut funcs);
    funcs.sort();
    funcs.dedup();
    funcs
}

fn collect_functions_inner(ast: &AstNode, funcs: &mut Vec<String>) {
    match ast {
        AstNode::FunctionCall { name, args } => {
            if !funcs.contains(name) {
                funcs.push(name.clone());
            }
            for arg in args {
                collect_functions_inner(arg, funcs);
            }
        }
        AstNode::BinaryOp { left, right, .. } => {
            collect_functions_inner(left, funcs);
            collect_functions_inner(right, funcs);
        }
        AstNode::UnaryOp { operand, .. } => collect_functions_inner(operand, funcs),
        AstNode::Equation { left, right } => {
            collect_functions_inner(left, funcs);
            collect_functions_inner(right, funcs);
        }
        AstNode::Inequality { left, right, .. } => {
            collect_functions_inner(left, funcs);
            collect_functions_inner(right, funcs);
        }
        AstNode::ImplicitMul { coefficient, variable } => {
            collect_functions_inner(coefficient, funcs);
            collect_functions_inner(variable, funcs);
        }
        _ => {}
    }
}

/// Find the maximum degree of any variable in the expression.
/// e.g., x^3 + 2x has max degree 3.
fn max_variable_degree(ast: &AstNode) -> u32 {
    match ast {
        AstNode::Variable(_) => 1,
        AstNode::Number(_) => 0,
        AstNode::BinaryOp { op: BinaryOperator::Power, left, right } => {
            if contains_variables(left) {
                if let AstNode::Number(n) = right.as_ref() {
                    return *n as u32;
                }
            }
            max_variable_degree(left).max(max_variable_degree(right))
        }
        AstNode::BinaryOp { left, right, .. } => {
            max_variable_degree(left).max(max_variable_degree(right))
        }
        AstNode::UnaryOp { operand, .. } => max_variable_degree(operand),
        AstNode::FunctionCall { args, .. } => args.iter().map(max_variable_degree).max().unwrap_or(0),
        AstNode::Equation { left, right } => max_variable_degree(left).max(max_variable_degree(right)),
        AstNode::Inequality { left, right, .. } => max_variable_degree(left).max(max_variable_degree(right)),
        AstNode::ImplicitMul { coefficient, variable } => {
            // 2x^3 -> the variable part might be x^3
            max_variable_degree(coefficient).max(max_variable_degree(variable))
        }
    }
}

/// Check if an expression is purely numeric (no variables, no functions).
fn is_purely_numeric(ast: &AstNode) -> bool {
    match ast {
        AstNode::Number(_) => true,
        AstNode::Variable(_) => false,
        AstNode::BinaryOp { left, right, .. } => is_purely_numeric(left) && is_purely_numeric(right),
        AstNode::UnaryOp { operand, .. } => is_purely_numeric(operand),
        AstNode::FunctionCall { .. } => false,
        AstNode::Equation { left, right } => is_purely_numeric(left) && is_purely_numeric(right),
        AstNode::Inequality { left, right, .. } => is_purely_numeric(left) && is_purely_numeric(right),
        AstNode::ImplicitMul { .. } => false,
    }
}

/// Check if all numbers in the expression are single-digit (0-9).
fn all_single_digit(ast: &AstNode) -> bool {
    match ast {
        AstNode::Number(n) => *n >= 0.0 && *n <= 9.0 && *n == n.floor(),
        AstNode::Variable(_) => true,
        AstNode::BinaryOp { left, right, .. } => all_single_digit(left) && all_single_digit(right),
        AstNode::UnaryOp { operand, .. } => all_single_digit(operand),
        AstNode::FunctionCall { args, .. } => args.iter().all(all_single_digit),
        AstNode::Equation { left, right } => all_single_digit(left) && all_single_digit(right),
        AstNode::Inequality { left, right, .. } => all_single_digit(left) && all_single_digit(right),
        AstNode::ImplicitMul { coefficient, variable } => all_single_digit(coefficient) && all_single_digit(variable),
    }
}

/// Check if the expression involves fractions (division of integers).
fn involves_fractions(ast: &AstNode) -> bool {
    match ast {
        AstNode::BinaryOp { op: BinaryOperator::Divide, left, right } => {
            is_purely_numeric(left) && is_purely_numeric(right)
        }
        AstNode::BinaryOp { left, right, .. } => involves_fractions(left) || involves_fractions(right),
        AstNode::UnaryOp { operand, .. } => involves_fractions(operand),
        AstNode::Equation { left, right } => involves_fractions(left) || involves_fractions(right),
        _ => false,
    }
}

// ============================================================================
// PATTERN DETERMINATION
// ============================================================================

/// Determine the primary pattern type and domain for a given AST.
fn determine_pattern(
    ast: &AstNode,
    has_variables: bool,
    is_equation: bool,
    is_inequality: bool,
    max_degree: u32,
    functions: &[String],
) -> (Domain, PatternType) {

    // Check for trig functions -> Tier 6
    let trig_funcs = ["sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh"];
    if functions.iter().any(|f| trig_funcs.contains(&f.as_str())) {
        let pattern = if functions.contains(&"sin".to_string()) {
            PatternType::TrigEvaluation
        } else if functions.contains(&"cos".to_string()) {
            PatternType::TrigEvaluation
        } else if functions.contains(&"tan".to_string()) {
            PatternType::TrigEvaluation
        } else if functions.iter().any(|f| f.starts_with('a')) {
            PatternType::InverseTrig
        } else {
            PatternType::TrigEvaluation
        };
        return (Domain::Trigonometry, pattern);
    }

    // Check for calculus-related patterns (derivative/integral notation would be Phase 2)

    // Check for log/ln -> Tier 5
    if functions.contains(&"log".to_string()) || functions.contains(&"ln".to_string()) {
        return (Domain::AdvancedAlgebra, PatternType::Logarithm);
    }

    // Check for sqrt/cbrt -> Tier 3
    if functions.contains(&"sqrt".to_string()) {
        return (Domain::PreAlgebra, PatternType::SquareRoot);
    }
    if functions.contains(&"cbrt".to_string()) {
        return (Domain::PreAlgebra, PatternType::NthRoot);
    }

    // Check for abs -> Tier 3
    if functions.contains(&"abs".to_string()) || has_absolute_value(ast) {
        return (Domain::PreAlgebra, PatternType::AbsoluteValue);
    }

    // Check for gcd/lcm -> Tier 3
    if functions.contains(&"gcd".to_string()) {
        return (Domain::PreAlgebra, PatternType::GCD);
    }
    if functions.contains(&"lcm".to_string()) {
        return (Domain::PreAlgebra, PatternType::LCM);
    }

    // Check for mod -> Tier 9
    if functions.contains(&"mod".to_string()) || has_modulo(ast) {
        return (Domain::AbstractApplied, PatternType::ModularArithmetic);
    }

    // Inequality -> Tier 4
    if is_inequality {
        return (Domain::Algebra, PatternType::LinearInequalitySolve);
    }

    // Has variables?
    if has_variables {
        if is_equation {
            // Equation with variables -> solve
            if max_degree == 1 {
                return (Domain::Algebra, PatternType::LinearEquationSolve);
            } else if max_degree == 2 {
                return (Domain::Algebra, PatternType::QuadraticEquationSolve);
            }
            return (Domain::Algebra, PatternType::LinearEquationSolve);
        }

        // Expression with variables (not equation)
        if has_like_terms(ast) {
            return (Domain::Algebra, PatternType::LikeTermsCombination);
        }

        if has_power_on_variable(ast) {
            if max_degree == 2 && is_polynomial_form(ast) {
                return (Domain::Algebra, PatternType::PolynomialMultiplication);
            }
            return (Domain::PreAlgebra, PatternType::ExponentEvaluation);
        }

        // Simple variable expression -> could be substitution or evaluation
        return (Domain::Algebra, PatternType::VariableSubstitution);
    }

    // Pure numeric expression
    if is_purely_numeric(ast) {
        // Check for exponentiation -> Tier 3
        if has_exponentiation(ast) {
            return (Domain::PreAlgebra, PatternType::ExponentEvaluation);
        }

        // Check for fractions -> Tier 3
        if involves_fractions(ast) {
            return classify_fraction_operation(ast);
        }

        // Pure arithmetic
        if all_single_digit(ast) {
            return classify_single_digit_operation(ast);
        }

        // Multi-digit arithmetic
        return classify_multi_digit_operation(ast);
    }

    // Default fallback
    (Domain::CoreArithmetic, PatternType::SingleDigitAddition)
}

/// Classify a single-digit arithmetic operation.
fn classify_single_digit_operation(ast: &AstNode) -> (Domain, PatternType) {
    match ast {
        AstNode::BinaryOp { op, .. } => match op {
            BinaryOperator::Add => (Domain::CoreArithmetic, PatternType::SingleDigitAddition),
            BinaryOperator::Subtract => (Domain::CoreArithmetic, PatternType::SingleDigitSubtraction),
            BinaryOperator::Multiply => (Domain::CoreArithmetic, PatternType::SingleDigitMultiplication),
            BinaryOperator::Divide => (Domain::CoreArithmetic, PatternType::SingleDigitDivision),
            BinaryOperator::Power => (Domain::PreAlgebra, PatternType::ExponentEvaluation),
            BinaryOperator::Modulo => (Domain::AbstractApplied, PatternType::ModularArithmetic),
        },
        AstNode::UnaryOp { op, .. } => match op {
            UnaryOperator::Negate => (Domain::CoreArithmetic, PatternType::SingleDigitSubtraction),
            UnaryOperator::Factorial => (Domain::AbstractApplied, PatternType::Permutation),
            UnaryOperator::AbsoluteValue => (Domain::PreAlgebra, PatternType::AbsoluteValue),
        },
        _ => (Domain::CoreArithmetic, PatternType::SingleDigitAddition),
    }
}

/// Classify a multi-digit arithmetic operation.
fn classify_multi_digit_operation(ast: &AstNode) -> (Domain, PatternType) {
    match ast {
        AstNode::BinaryOp { op, .. } => match op {
            BinaryOperator::Add => (Domain::ExtendedArithmetic, PatternType::MultiDigitAddition),
            BinaryOperator::Subtract => (Domain::ExtendedArithmetic, PatternType::MultiDigitSubtraction),
            BinaryOperator::Multiply => (Domain::ExtendedArithmetic, PatternType::MultiDigitMultiplication),
            BinaryOperator::Divide => (Domain::ExtendedArithmetic, PatternType::MultiDigitDivision),
            BinaryOperator::Power => (Domain::PreAlgebra, PatternType::ExponentEvaluation),
            BinaryOperator::Modulo => (Domain::AbstractApplied, PatternType::ModularArithmetic),
        },
        _ => (Domain::ExtendedArithmetic, PatternType::MultiDigitAddition),
    }
}

/// Classify a fraction operation.
fn classify_fraction_operation(ast: &AstNode) -> (Domain, PatternType) {
    match ast {
        AstNode::BinaryOp { op, left, right } => {
            let left_is_frac = matches!(left.as_ref(), AstNode::BinaryOp { op: BinaryOperator::Divide, .. });
            let right_is_frac = matches!(right.as_ref(), AstNode::BinaryOp { op: BinaryOperator::Divide, .. });

            if left_is_frac || right_is_frac {
                match op {
                    BinaryOperator::Add => return (Domain::PreAlgebra, PatternType::FractionAddition),
                    BinaryOperator::Subtract => return (Domain::PreAlgebra, PatternType::FractionSubtraction),
                    BinaryOperator::Multiply => return (Domain::PreAlgebra, PatternType::FractionMultiplication),
                    BinaryOperator::Divide => return (Domain::PreAlgebra, PatternType::FractionDivision),
                    _ => {}
                }
            }

            if matches!(op, BinaryOperator::Divide) {
                return (Domain::PreAlgebra, PatternType::FractionSimplification);
            }

            (Domain::PreAlgebra, PatternType::FractionAddition)
        }
        _ => (Domain::PreAlgebra, PatternType::FractionSimplification),
    }
}

/// Determine secondary patterns that may also apply.
fn determine_secondary_patterns(ast: &AstNode, _primary: &PatternType) -> Vec<PatternType> {
    let mut secondary = Vec::new();

    if has_multiple_operations(ast) {
        secondary.push(PatternType::OrderOfOperations);
    }

    if has_commutable_operations(ast) {
        secondary.push(PatternType::CommutativeApplication);
    }

    if has_distributable_form(ast) {
        secondary.push(PatternType::DistributiveExpansion);
    }

    secondary
}

// ============================================================================
// AST STRUCTURAL ANALYSIS HELPERS
// ============================================================================

fn has_absolute_value(ast: &AstNode) -> bool {
    match ast {
        AstNode::UnaryOp { op: UnaryOperator::AbsoluteValue, .. } => true,
        AstNode::BinaryOp { left, right, .. } => has_absolute_value(left) || has_absolute_value(right),
        _ => false,
    }
}

fn has_modulo(ast: &AstNode) -> bool {
    match ast {
        AstNode::BinaryOp { op: BinaryOperator::Modulo, .. } => true,
        AstNode::BinaryOp { left, right, .. } => has_modulo(left) || has_modulo(right),
        _ => false,
    }
}

fn has_exponentiation(ast: &AstNode) -> bool {
    match ast {
        AstNode::BinaryOp { op: BinaryOperator::Power, .. } => true,
        AstNode::BinaryOp { left, right, .. } => has_exponentiation(left) || has_exponentiation(right),
        AstNode::UnaryOp { operand, .. } => has_exponentiation(operand),
        _ => false,
    }
}

fn has_power_on_variable(ast: &AstNode) -> bool {
    match ast {
        AstNode::BinaryOp { op: BinaryOperator::Power, left, .. } => contains_variables(left),
        AstNode::BinaryOp { left, right, .. } => has_power_on_variable(left) || has_power_on_variable(right),
        AstNode::UnaryOp { operand, .. } => has_power_on_variable(operand),
        AstNode::ImplicitMul { coefficient, variable } => has_power_on_variable(coefficient) || has_power_on_variable(variable),
        _ => false,
    }
}

fn has_like_terms(ast: &AstNode) -> bool {
    // Check if expression has addition/subtraction of terms with the same variable
    match ast {
        AstNode::BinaryOp { op: BinaryOperator::Add | BinaryOperator::Subtract, left, right } => {
            let left_vars = collect_variables(left);
            let right_vars = collect_variables(right);
            // Like terms if same variables appear on both sides
            !left_vars.is_empty() && left_vars == right_vars
        }
        _ => false,
    }
}

fn is_polynomial_form(_ast: &AstNode) -> bool {
    // Simplified check — a more thorough version would walk the tree
    true
}

fn has_multiple_operations(ast: &AstNode) -> bool {
    match ast {
        AstNode::BinaryOp { left, right, .. } => {
            matches!(left.as_ref(), AstNode::BinaryOp { .. }) ||
            matches!(right.as_ref(), AstNode::BinaryOp { .. })
        }
        _ => false,
    }
}

fn has_commutable_operations(ast: &AstNode) -> bool {
    match ast {
        AstNode::BinaryOp { op: BinaryOperator::Add | BinaryOperator::Multiply, .. } => true,
        AstNode::BinaryOp { left, right, .. } => has_commutable_operations(left) || has_commutable_operations(right),
        _ => false,
    }
}

fn has_distributable_form(ast: &AstNode) -> bool {
    // a * (b + c) form
    match ast {
        AstNode::BinaryOp { op: BinaryOperator::Multiply, right, .. } => {
            matches!(right.as_ref(), AstNode::BinaryOp { op: BinaryOperator::Add | BinaryOperator::Subtract, .. })
        }
        AstNode::ImplicitMul { variable, .. } => {
            matches!(variable.as_ref(), AstNode::BinaryOp { op: BinaryOperator::Add | BinaryOperator::Subtract, .. })
        }
        _ => false,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ================================================================
    // TOKENIZER TESTS
    // ================================================================

    #[test]
    fn test_tokenize_number() {
        let tokens = tokenize("42").unwrap();
        assert_eq!(tokens[0], Token::Number(42.0));
    }

    #[test]
    fn test_tokenize_decimal() {
        let tokens = tokenize("3.14").unwrap();
        assert_eq!(tokens[0], Token::Number(3.14));
    }

    #[test]
    fn test_tokenize_variable() {
        let tokens = tokenize("x").unwrap();
        assert_eq!(tokens[0], Token::Variable("x".to_string()));
    }

    #[test]
    fn test_tokenize_multichar_variable() {
        let tokens = tokenize("theta").unwrap();
        assert_eq!(tokens[0], Token::Variable("theta".to_string()));
    }

    #[test]
    fn test_tokenize_function() {
        let tokens = tokenize("sin").unwrap();
        assert_eq!(tokens[0], Token::Function("sin".to_string()));
    }

    #[test]
    fn test_tokenize_constant_pi() {
        let tokens = tokenize("pi").unwrap();
        assert_eq!(tokens[0], Token::Number(std::f64::consts::PI));
    }

    #[test]
    fn test_tokenize_operators() {
        let tokens = tokenize("+ - * / ^ % !").unwrap();
        assert_eq!(tokens[0], Token::Plus);
        assert_eq!(tokens[1], Token::Minus);
        assert_eq!(tokens[2], Token::Star);
        assert_eq!(tokens[3], Token::Slash);
        assert_eq!(tokens[4], Token::Caret);
        assert_eq!(tokens[5], Token::Percent);
        assert_eq!(tokens[6], Token::Bang);
    }

    #[test]
    fn test_tokenize_comparison() {
        let tokens = tokenize("< > <= >=").unwrap();
        assert_eq!(tokens[0], Token::LessThan);
        assert_eq!(tokens[1], Token::GreaterThan);
        assert_eq!(tokens[2], Token::LessEqual);
        assert_eq!(tokens[3], Token::GreaterEqual);
    }

    #[test]
    fn test_tokenize_parens() {
        let tokens = tokenize("(x)").unwrap();
        assert_eq!(tokens[0], Token::LeftParen);
        assert_eq!(tokens[1], Token::Variable("x".to_string()));
        assert_eq!(tokens[2], Token::RightParen);
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let tokens = tokenize("2 * x + 3").unwrap();
        assert_eq!(tokens[0], Token::Number(2.0));
        assert_eq!(tokens[1], Token::Star);
        assert_eq!(tokens[2], Token::Variable("x".to_string()));
        assert_eq!(tokens[3], Token::Plus);
        assert_eq!(tokens[4], Token::Number(3.0));
    }

    #[test]
    fn test_tokenize_equation() {
        let tokens = tokenize("2x + 3 = 7").unwrap();
        assert!(tokens.contains(&Token::Equals));
    }

    #[test]
    fn test_tokenize_function_call() {
        let tokens = tokenize("sin(x)").unwrap();
        assert_eq!(tokens[0], Token::Function("sin".to_string()));
        assert_eq!(tokens[1], Token::LeftParen);
        assert_eq!(tokens[2], Token::Variable("x".to_string()));
        assert_eq!(tokens[3], Token::RightParen);
    }

    #[test]
    fn test_tokenize_error_bad_char() {
        let result = tokenize("2 @ 3");
        assert!(result.is_err());
    }

    #[test]
    fn test_tokenize_whitespace_handling() {
        let t1 = tokenize("2+3").unwrap();
        let t2 = tokenize("  2  +  3  ").unwrap();
        assert_eq!(t1.len(), t2.len());
    }

    #[test]
    fn test_tokenize_all_known_functions() {
        for &func in KNOWN_FUNCTIONS {
            let tokens = tokenize(func).unwrap();
            assert_eq!(tokens[0], Token::Function(func.to_string()),
                "Function {} not recognized", func);
        }
    }

    #[test]
    fn test_tokenize_pipe() {
        let tokens = tokenize("|x|").unwrap();
        assert_eq!(tokens[0], Token::Pipe);
        assert_eq!(tokens[2], Token::Pipe);
    }

    // ================================================================
    // PARSER TESTS — BASIC ARITHMETIC
    // ================================================================

    #[test]
    fn test_parse_number() {
        let ast = parse("42").unwrap();
        assert_eq!(ast, AstNode::Number(42.0));
    }

    #[test]
    fn test_parse_decimal() {
        let ast = parse("3.14").unwrap();
        assert_eq!(ast, AstNode::Number(3.14));
    }

    #[test]
    fn test_parse_variable() {
        let ast = parse("x").unwrap();
        assert_eq!(ast, AstNode::Variable("x".to_string()));
    }

    #[test]
    fn test_parse_addition() {
        let ast = parse("2 + 3").unwrap();
        assert_eq!(ast, AstNode::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(AstNode::Number(2.0)),
            right: Box::new(AstNode::Number(3.0)),
        });
    }

    #[test]
    fn test_parse_subtraction() {
        let ast = parse("5 - 3").unwrap();
        assert_eq!(ast, AstNode::BinaryOp {
            op: BinaryOperator::Subtract,
            left: Box::new(AstNode::Number(5.0)),
            right: Box::new(AstNode::Number(3.0)),
        });
    }

    #[test]
    fn test_parse_multiplication() {
        let ast = parse("4 * 7").unwrap();
        assert_eq!(ast, AstNode::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(AstNode::Number(4.0)),
            right: Box::new(AstNode::Number(7.0)),
        });
    }

    #[test]
    fn test_parse_division() {
        let ast = parse("8 / 2").unwrap();
        assert_eq!(ast, AstNode::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(AstNode::Number(8.0)),
            right: Box::new(AstNode::Number(2.0)),
        });
    }

    #[test]
    fn test_parse_exponentiation() {
        let ast = parse("2 ^ 3").unwrap();
        assert_eq!(ast, AstNode::BinaryOp {
            op: BinaryOperator::Power,
            left: Box::new(AstNode::Number(2.0)),
            right: Box::new(AstNode::Number(3.0)),
        });
    }

    // ================================================================
    // PARSER TESTS — PRECEDENCE
    // ================================================================

    #[test]
    fn test_precedence_mul_before_add() {
        // 2 + 3 * 4 should be 2 + (3*4), not (2+3)*4
        let ast = parse("2 + 3 * 4").unwrap();
        match ast {
            AstNode::BinaryOp { op: BinaryOperator::Add, left, right } => {
                assert_eq!(*left, AstNode::Number(2.0));
                assert!(matches!(*right, AstNode::BinaryOp { op: BinaryOperator::Multiply, .. }));
            }
            _ => panic!("Expected addition at top level"),
        }
    }

    #[test]
    fn test_precedence_parens_override() {
        // (2 + 3) * 4 should group addition first
        let ast = parse("(2 + 3) * 4").unwrap();
        match ast {
            AstNode::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                assert!(matches!(*left, AstNode::BinaryOp { op: BinaryOperator::Add, .. }));
                assert_eq!(*right, AstNode::Number(4.0));
            }
            _ => panic!("Expected multiplication at top level"),
        }
    }

    #[test]
    fn test_precedence_power_before_mul() {
        // 2 * 3 ^ 2 should be 2 * (3^2)
        let ast = parse("2 * 3 ^ 2").unwrap();
        match ast {
            AstNode::BinaryOp { op: BinaryOperator::Multiply, right, .. } => {
                assert!(matches!(*right, AstNode::BinaryOp { op: BinaryOperator::Power, .. }));
            }
            _ => panic!("Expected multiply at top level"),
        }
    }

    #[test]
    fn test_left_associativity_add() {
        // 1 + 2 + 3 should be (1+2) + 3
        let ast = parse("1 + 2 + 3").unwrap();
        match ast {
            AstNode::BinaryOp { op: BinaryOperator::Add, left, right } => {
                assert!(matches!(*left, AstNode::BinaryOp { op: BinaryOperator::Add, .. }));
                assert_eq!(*right, AstNode::Number(3.0));
            }
            _ => panic!("Expected left-associative addition"),
        }
    }

    // ================================================================
    // PARSER TESTS — UNARY OPERATIONS
    // ================================================================

    #[test]
    fn test_parse_negation() {
        let ast = parse("-5").unwrap();
        assert_eq!(ast, AstNode::UnaryOp {
            op: UnaryOperator::Negate,
            operand: Box::new(AstNode::Number(5.0)),
        });
    }

    #[test]
    fn test_parse_absolute_value() {
        let ast = parse("|x|").unwrap();
        assert_eq!(ast, AstNode::UnaryOp {
            op: UnaryOperator::AbsoluteValue,
            operand: Box::new(AstNode::Variable("x".to_string())),
        });
    }

    #[test]
    fn test_parse_factorial() {
        let ast = parse("5!").unwrap();
        assert_eq!(ast, AstNode::UnaryOp {
            op: UnaryOperator::Factorial,
            operand: Box::new(AstNode::Number(5.0)),
        });
    }

    // ================================================================
    // PARSER TESTS — FUNCTIONS
    // ================================================================

    #[test]
    fn test_parse_function_single_arg() {
        let ast = parse("sin(x)").unwrap();
        assert_eq!(ast, AstNode::FunctionCall {
            name: "sin".to_string(),
            args: vec![AstNode::Variable("x".to_string())],
        });
    }

    #[test]
    fn test_parse_function_multi_arg() {
        let ast = parse("gcd(12, 8)").unwrap();
        assert_eq!(ast, AstNode::FunctionCall {
            name: "gcd".to_string(),
            args: vec![AstNode::Number(12.0), AstNode::Number(8.0)],
        });
    }

    #[test]
    fn test_parse_nested_functions() {
        let ast = parse("sin(cos(x))").unwrap();
        match ast {
            AstNode::FunctionCall { name, args } => {
                assert_eq!(name, "sin");
                assert!(matches!(&args[0], AstNode::FunctionCall { name, .. } if name == "cos"));
            }
            _ => panic!("Expected nested function call"),
        }
    }

    #[test]
    fn test_parse_sqrt() {
        let ast = parse("sqrt(9)").unwrap();
        assert_eq!(ast, AstNode::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![AstNode::Number(9.0)],
        });
    }

    #[test]
    fn test_parse_log_two_args() {
        let ast = parse("log(2, 8)").unwrap();
        match ast {
            AstNode::FunctionCall { name, args } => {
                assert_eq!(name, "log");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected log function call"),
        }
    }

    // ================================================================
    // PARSER TESTS — EQUATIONS & INEQUALITIES
    // ================================================================

    #[test]
    fn test_parse_equation() {
        let ast = parse("2 * x + 3 = 7").unwrap();
        assert!(matches!(ast, AstNode::Equation { .. }));
    }

    #[test]
    fn test_parse_inequality_less() {
        let ast = parse("x < 5").unwrap();
        assert!(matches!(ast, AstNode::Inequality { op: ComparisonOp::LessThan, .. }));
    }

    #[test]
    fn test_parse_inequality_greater_equal() {
        let ast = parse("x >= 10").unwrap();
        assert!(matches!(ast, AstNode::Inequality { op: ComparisonOp::GreaterEqual, .. }));
    }

    // ================================================================
    // PARSER TESTS — IMPLICIT MULTIPLICATION
    // ================================================================

    #[test]
    fn test_parse_implicit_mul_number_var() {
        // 2x should be ImplicitMul(2, x)
        let ast = parse("2x").unwrap();
        assert!(matches!(ast, AstNode::ImplicitMul { .. }));
    }

    #[test]
    fn test_parse_implicit_mul_in_expression() {
        // 2n + 2n
        let ast = parse("2n + 2n").unwrap();
        match ast {
            AstNode::BinaryOp { op: BinaryOperator::Add, left, right } => {
                assert!(matches!(*left, AstNode::ImplicitMul { .. }));
                assert!(matches!(*right, AstNode::ImplicitMul { .. }));
            }
            _ => panic!("Expected addition of implicit muls"),
        }
    }

    #[test]
    fn test_parse_implicit_mul_with_paren() {
        // 3(x + 1) should be ImplicitMul(3, (x+1))
        let ast = parse("3(x + 1)").unwrap();
        assert!(matches!(ast, AstNode::ImplicitMul { .. }));
    }

    // ================================================================
    // PARSER TESTS — COMPLEX EXPRESSIONS
    // ================================================================

    #[test]
    fn test_parse_quadratic() {
        // x^2 + 5x + 6
        let ast = parse("x^2 + 5x + 6").unwrap();
        // Should parse without error; top-level is addition
        assert!(matches!(ast, AstNode::BinaryOp { op: BinaryOperator::Add, .. }));
    }

    #[test]
    fn test_parse_nested_parens() {
        let ast = parse("((2 + 3) * (4 - 1))").unwrap();
        assert!(matches!(ast, AstNode::BinaryOp { op: BinaryOperator::Multiply, .. }));
    }

    #[test]
    fn test_parse_multi_term_polynomial() {
        let ast = parse("3x^2 + 2x + 1").unwrap();
        assert!(!format!("{:?}", ast).is_empty()); // Parses without error
    }

    // ================================================================
    // PARSER TESTS — ERROR HANDLING
    // ================================================================

    #[test]
    fn test_parse_empty_string() {
        assert_eq!(parse(""), Err(ParseError::EmptyExpression));
    }

    #[test]
    fn test_parse_whitespace_only() {
        assert_eq!(parse("   "), Err(ParseError::EmptyExpression));
    }

    #[test]
    fn test_parse_mismatched_parens() {
        assert!(parse("(2 + 3").is_err());
    }

    #[test]
    fn test_parse_invalid_char() {
        assert!(parse("2 @ 3").is_err());
    }

    // ================================================================
    // CLASSIFIER TESTS
    // ================================================================

    #[test]
    fn test_classify_single_digit_add() {
        let ast = parse("3 + 4").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::CoreArithmetic);
        assert_eq!(class.pattern_type, PatternType::SingleDigitAddition);
        assert!(!class.has_variables);
        assert!(!class.is_equation);
    }

    #[test]
    fn test_classify_single_digit_mul() {
        let ast = parse("6 * 7").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::CoreArithmetic);
        assert_eq!(class.pattern_type, PatternType::SingleDigitMultiplication);
    }

    #[test]
    fn test_classify_multi_digit_add() {
        let ast = parse("347 + 286").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::ExtendedArithmetic);
        assert_eq!(class.pattern_type, PatternType::MultiDigitAddition);
    }

    #[test]
    fn test_classify_multi_digit_mul() {
        let ast = parse("12 * 34").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::ExtendedArithmetic);
        assert_eq!(class.pattern_type, PatternType::MultiDigitMultiplication);
    }

    #[test]
    fn test_classify_exponent() {
        let ast = parse("2 ^ 10").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::PreAlgebra);
        assert_eq!(class.pattern_type, PatternType::ExponentEvaluation);
    }

    #[test]
    fn test_classify_fraction() {
        let ast = parse("3 / 4").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::PreAlgebra);
    }

    #[test]
    fn test_classify_sqrt() {
        let ast = parse("sqrt(16)").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::PreAlgebra);
        assert_eq!(class.pattern_type, PatternType::SquareRoot);
    }

    #[test]
    fn test_classify_gcd() {
        let ast = parse("gcd(12, 8)").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::PreAlgebra);
        assert_eq!(class.pattern_type, PatternType::GCD);
    }

    #[test]
    fn test_classify_like_terms() {
        let ast = parse("2n + 2n").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::Algebra);
        assert_eq!(class.pattern_type, PatternType::LikeTermsCombination);
        assert!(class.has_variables);
        assert_eq!(class.variables, vec!["n"]);
    }

    #[test]
    fn test_classify_linear_equation() {
        let ast = parse("2 * x + 3 = 7").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::Algebra);
        assert_eq!(class.pattern_type, PatternType::LinearEquationSolve);
        assert!(class.is_equation);
        assert!(class.has_variables);
    }

    #[test]
    fn test_classify_quadratic_equation() {
        let ast = parse("x^2 + 5 * x + 6 = 0").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::Algebra);
        assert_eq!(class.pattern_type, PatternType::QuadraticEquationSolve);
        assert_eq!(class.max_variable_degree, 2);
    }

    #[test]
    fn test_classify_inequality() {
        let ast = parse("2 * x + 1 < 7").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::Algebra);
        assert_eq!(class.pattern_type, PatternType::LinearInequalitySolve);
        assert!(class.is_inequality);
    }

    #[test]
    fn test_classify_sin() {
        let ast = parse("sin(x)").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::Trigonometry);
        assert_eq!(class.pattern_type, PatternType::TrigEvaluation);
        assert_eq!(class.functions_used, vec!["sin"]);
    }

    #[test]
    fn test_classify_log() {
        let ast = parse("log(2, 8)").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::AdvancedAlgebra);
        assert_eq!(class.pattern_type, PatternType::Logarithm);
    }

    #[test]
    fn test_classify_absolute_value() {
        let ast = parse("|x|").unwrap();
        let class = classify(&ast);
        assert_eq!(class.pattern_type, PatternType::AbsoluteValue);
    }

    #[test]
    fn test_classify_has_correct_variables() {
        let ast = parse("x^2 + y + z").unwrap();
        let class = classify(&ast);
        assert_eq!(class.variables, vec!["x", "y", "z"]);
    }

    #[test]
    fn test_classify_order_of_ops_secondary() {
        let ast = parse("2 + 3 * 4").unwrap();
        let class = classify(&ast);
        assert!(class.secondary_patterns.contains(&PatternType::OrderOfOperations));
    }

    #[test]
    fn test_classify_commutative_secondary() {
        let ast = parse("3 + 5").unwrap();
        let class = classify(&ast);
        assert!(class.secondary_patterns.contains(&PatternType::CommutativeApplication));
    }

    // ================================================================
    // AST DISPLAY TESTS
    // ================================================================

    #[test]
    fn test_display_number() {
        let ast = AstNode::Number(42.0);
        assert_eq!(format!("{}", ast), "42");
    }

    #[test]
    fn test_display_variable() {
        let ast = AstNode::Variable("x".to_string());
        assert_eq!(format!("{}", ast), "x");
    }

    #[test]
    fn test_display_binary_op() {
        let ast = parse("2 + 3").unwrap();
        assert_eq!(format!("{}", ast), "(2 + 3)");
    }

    #[test]
    fn test_display_equation() {
        let ast = parse("x = 5").unwrap();
        assert_eq!(format!("{}", ast), "x = 5");
    }

    // ================================================================
    // ROUNDTRIP TESTS — Parse then verify structure
    // ================================================================

    #[test]
    fn test_roundtrip_simple_add() {
        let ast = parse("7 + 8").unwrap();
        let class = classify(&ast);
        assert_eq!(class.pattern_type, PatternType::SingleDigitAddition);
        assert!(!class.has_variables);
    }

    #[test]
    fn test_roundtrip_algebraic() {
        let ast = parse("2n + 2n").unwrap();
        let class = classify(&ast);
        assert_eq!(class.pattern_type, PatternType::LikeTermsCombination);
        assert!(class.has_variables);
        assert_eq!(class.variables, vec!["n"]);
    }

    #[test]
    fn test_roundtrip_trig() {
        let ast = parse("sin(pi / 2)").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::Trigonometry);
        assert_eq!(class.functions_used, vec!["sin"]);
    }

    #[test]
    fn test_roundtrip_347_plus_286() {
        let ast = parse("347 + 286").unwrap();
        let class = classify(&ast);
        assert_eq!(class.domain, Domain::ExtendedArithmetic);
        assert_eq!(class.pattern_type, PatternType::MultiDigitAddition);
    }

    #[test]
    fn test_roundtrip_complex_expression() {
        // Should parse and classify without errors
        let expressions = [
            "2 + 3",
            "347 + 286",
            "2^10",
            "sqrt(16)",
            "sin(pi)",
            "2n + 2n",
            "x^2 + 5 * x + 6 = 0",
            "gcd(12, 8)",
            "log(2, 8)",
            "|x|",
            "3 * (x + 1)",
            "2 * x + 1 < 7",
            "5!",
            "-42",
        ];
        for expr in expressions {
            let ast = parse(expr);
            assert!(ast.is_ok(), "Failed to parse: {}", expr);
            let class = classify(&ast.unwrap());
            assert!(!format!("{:?}", class.pattern_type).is_empty(),
                "Failed to classify: {}", expr);
        }
    }
}
