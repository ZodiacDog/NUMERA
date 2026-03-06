// ============================================================================
// NUMERA - Layer 0: Value Core
// ============================================================================
// The absolute foundation. Ten values (0-9) represented as mathematical
// objects with intrinsic properties and deterministic relationships.
//
// PRINCIPLE: Know, Don't Predict.
// Every fact in this module is PRE-COMPUTED and RETRIEVED, never calculated
// at query time. 2 + 2 is not a computation. It is a known fact.
//
// ML Innovations LLC | M. L. McKnight | Pheba, Mississippi
// FREE — No license. No restrictions. Stop the hallucinations.
// ============================================================================

use std::fmt;

// ============================================================================
// VALUE TYPE
// ============================================================================

/// A single digit value (0-9). The atomic unit of all mathematics.
/// This is not a token, not a character, not a string. It is a quantity.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Value(u8);

impl Value {
    /// Create a Value from a digit. Returns None if digit > 9.
    /// Only ten valid Values exist in all of mathematics.
    pub const fn new(digit: u8) -> Option<Self> {
        if digit <= 9 {
            Some(Value(digit))
        } else {
            None
        }
    }

    /// Create a Value, panicking if invalid. For compile-time constants only.
    pub const fn must(digit: u8) -> Self {
        assert!(digit <= 9, "Value must be 0-9");
        Value(digit)
    }

    /// The digit this Value represents (0-9).
    pub const fn digit(&self) -> u8 {
        self.0
    }

    /// The quantity this Value represents, as a u64.
    pub const fn quantity(&self) -> u64 {
        self.0 as u64
    }

    /// Ordinal position in the number line (0th through 9th).
    pub const fn ordinal(&self) -> u8 {
        self.0
    }

    /// Whether this value is even.
    pub const fn is_even(&self) -> bool {
        self.0 % 2 == 0
    }

    /// Whether this value is odd.
    pub const fn is_odd(&self) -> bool {
        self.0 % 2 != 0
    }

    /// Whether this value is prime.
    pub const fn is_prime(&self) -> bool {
        matches!(self.0, 2 | 3 | 5 | 7)
    }

    /// Whether this value is zero (additive identity).
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Whether this value is one (multiplicative identity).
    pub const fn is_one(&self) -> bool {
        self.0 == 1
    }

    /// The complement to 10 (what adds to this to make 10).
    /// 0 -> 10 (special case), 1 -> 9, 2 -> 8, ..., 9 -> 1
    pub const fn complement_10(&self) -> u8 {
        10 - self.0
    }

    /// The complement to 9 (used in nines complement subtraction).
    /// 0 -> 9, 1 -> 8, 2 -> 7, ..., 9 -> 0
    pub const fn complement_9(&self) -> u8 {
        9 - self.0
    }

    /// All digits greater than this value.
    pub fn greater_values(&self) -> Vec<Value> {
        ((self.0 + 1)..=9).map(|d| Value(d)).collect()
    }

    /// All digits less than this value.
    pub fn lesser_values(&self) -> Vec<Value> {
        (0..self.0).map(|d| Value(d)).collect()
    }

    /// Factor set of this digit.
    pub fn factors(&self) -> Vec<u8> {
        if self.0 == 0 {
            return vec![];
        }
        (1..=self.0).filter(|&f| self.0 % f == 0).collect()
    }

    /// Whether this digit evenly divides another digit.
    pub const fn divides(&self, other: &Value) -> bool {
        if self.0 == 0 {
            return false;
        }
        other.0 % self.0 == 0
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "V({})", self.0)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// THE TEN VALUES - Named constants for clarity
// ============================================================================

pub const ZERO: Value = Value(0);
pub const ONE: Value = Value(1);
pub const TWO: Value = Value(2);
pub const THREE: Value = Value(3);
pub const FOUR: Value = Value(4);
pub const FIVE: Value = Value(5);
pub const SIX: Value = Value(6);
pub const SEVEN: Value = Value(7);
pub const EIGHT: Value = Value(8);
pub const NINE: Value = Value(9);

/// All ten values in order. The complete foundation of mathematics.
pub const ALL_VALUES: [Value; 10] = [
    ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE,
];

// ============================================================================
// PRE-COMPUTED ADDITION TABLE
// ============================================================================

pub static ADDITION_TABLE: [[u8; 10]; 10] = [
    [  0,  1,  2,  3,  4,  5,  6,  7,  8,  9],
    [  1,  2,  3,  4,  5,  6,  7,  8,  9, 10],
    [  2,  3,  4,  5,  6,  7,  8,  9, 10, 11],
    [  3,  4,  5,  6,  7,  8,  9, 10, 11, 12],
    [  4,  5,  6,  7,  8,  9, 10, 11, 12, 13],
    [  5,  6,  7,  8,  9, 10, 11, 12, 13, 14],
    [  6,  7,  8,  9, 10, 11, 12, 13, 14, 15],
    [  7,  8,  9, 10, 11, 12, 13, 14, 15, 16],
    [  8,  9, 10, 11, 12, 13, 14, 15, 16, 17],
    [  9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
];

// ============================================================================
// PRE-COMPUTED SUBTRACTION TABLE
// ============================================================================

pub static SUBTRACTION_TABLE: [[i8; 10]; 10] = [
    [  0, -1, -2, -3, -4, -5, -6, -7, -8, -9],
    [  1,  0, -1, -2, -3, -4, -5, -6, -7, -8],
    [  2,  1,  0, -1, -2, -3, -4, -5, -6, -7],
    [  3,  2,  1,  0, -1, -2, -3, -4, -5, -6],
    [  4,  3,  2,  1,  0, -1, -2, -3, -4, -5],
    [  5,  4,  3,  2,  1,  0, -1, -2, -3, -4],
    [  6,  5,  4,  3,  2,  1,  0, -1, -2, -3],
    [  7,  6,  5,  4,  3,  2,  1,  0, -1, -2],
    [  8,  7,  6,  5,  4,  3,  2,  1,  0, -1],
    [  9,  8,  7,  6,  5,  4,  3,  2,  1,  0],
];

// ============================================================================
// PRE-COMPUTED MULTIPLICATION TABLE
// ============================================================================

pub static MULTIPLICATION_TABLE: [[u8; 10]; 10] = [
    [  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [  0,  1,  2,  3,  4,  5,  6,  7,  8,  9],
    [  0,  2,  4,  6,  8, 10, 12, 14, 16, 18],
    [  0,  3,  6,  9, 12, 15, 18, 21, 24, 27],
    [  0,  4,  8, 12, 16, 20, 24, 28, 32, 36],
    [  0,  5, 10, 15, 20, 25, 30, 35, 40, 45],
    [  0,  6, 12, 18, 24, 30, 36, 42, 48, 54],
    [  0,  7, 14, 21, 28, 35, 42, 49, 56, 63],
    [  0,  8, 16, 24, 32, 40, 48, 56, 64, 72],
    [  0,  9, 18, 27, 36, 45, 54, 63, 72, 81],
];

// ============================================================================
// FACT RETRIEVAL FUNCTIONS
// ============================================================================

/// Retrieve the addition fact for two single-digit values.
pub fn add(a: Value, b: Value) -> u8 {
    ADDITION_TABLE[a.digit() as usize][b.digit() as usize]
}

/// Retrieve the subtraction fact for two single-digit values.
pub fn sub(a: Value, b: Value) -> i8 {
    SUBTRACTION_TABLE[a.digit() as usize][b.digit() as usize]
}

/// Retrieve the multiplication fact for two single-digit values.
pub fn mul(a: Value, b: Value) -> u8 {
    MULTIPLICATION_TABLE[a.digit() as usize][b.digit() as usize]
}

/// Retrieve the division fact. Returns (quotient, remainder) or None for div by zero.
pub fn div(a: Value, b: Value) -> Option<(u8, u8)> {
    if b.is_zero() {
        None
    } else {
        Some((a.digit() / b.digit(), a.digit() % b.digit()))
    }
}

// ============================================================================
// MULTI-DIGIT OPERATIONS VIA PLACE VALUE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaceComponent {
    pub value: Value,
    pub place: u64,
}

impl PlaceComponent {
    pub fn contribution(&self) -> u64 {
        self.value.quantity() * self.place
    }
}

/// Decompose any non-negative integer into Value Core components via place value.
pub fn decompose(n: u64) -> Vec<PlaceComponent> {
    if n == 0 {
        return vec![PlaceComponent { value: ZERO, place: 1 }];
    }
    let mut components = Vec::new();
    let mut remaining = n;
    let mut place = 1u64;
    while remaining > 0 {
        let digit = (remaining % 10) as u8;
        components.push(PlaceComponent { value: Value(digit), place });
        remaining /= 10;
        place *= 10;
    }
    components.reverse();
    components
}

/// Compose a number from Value Core components. Inverse of decompose.
pub fn compose(components: &[PlaceComponent]) -> u64 {
    components.iter().map(|c| c.contribution()).sum()
}

/// Number of digits in a non-negative integer.
pub fn digit_count(n: u64) -> usize {
    if n == 0 { return 1; }
    let mut count = 0;
    let mut remaining = n;
    while remaining > 0 { count += 1; remaining /= 10; }
    count
}

/// Extract digit at position (0 = ones, 1 = tens, etc.).
pub fn digit_at(n: u64, position: u32) -> Value {
    let divisor = 10u64.pow(position);
    Value(((n / divisor) % 10) as u8)
}

// ============================================================================
// MULTI-DIGIT ARITHMETIC
// ============================================================================

#[derive(Debug, Clone)]
pub struct ArithmeticStep {
    pub position: usize,
    pub operand_a: Value,
    pub operand_b: Value,
    pub carry_in: u8,
    pub intermediate: u8,
    pub result_digit: u8,
    pub carry_out: u8,
    pub rule: &'static str,
}

#[derive(Debug, Clone)]
pub struct MultiDigitResult {
    pub result: u64,
    pub steps: Vec<ArithmeticStep>,
    pub verified: bool,
}

#[derive(Debug, Clone)]
pub struct DivisionResult {
    pub quotient: u64,
    pub remainder: u64,
    pub dividend: u64,
    pub divisor: u64,
    pub verified: bool,
}

/// Add two non-negative integers using Value Core facts and carry propagation.
pub fn add_multi(a: u64, b: u64) -> MultiDigitResult {
    let mut result_digits: Vec<u8> = Vec::new();
    let mut steps: Vec<ArithmeticStep> = Vec::new();
    let mut carry: u8 = 0;
    let max_digits = digit_count(a).max(digit_count(b));

    for pos in 0..max_digits {
        let digit_a = digit_at(a, pos as u32);
        let digit_b = digit_at(b, pos as u32);
        let partial_sum = add(digit_a, digit_b);
        let total = partial_sum + carry;
        let result_digit = total % 10;
        let new_carry = total / 10;

        steps.push(ArithmeticStep {
            position: pos, operand_a: digit_a, operand_b: digit_b,
            carry_in: carry, intermediate: partial_sum,
            result_digit, carry_out: new_carry, rule: "value_core_addition",
        });
        result_digits.push(result_digit);
        carry = new_carry;
    }
    if carry > 0 { result_digits.push(carry); }

    let mut result: u64 = 0;
    for (i, &d) in result_digits.iter().enumerate() {
        result += (d as u64) * 10u64.pow(i as u32);
    }
    MultiDigitResult { result, steps, verified: false }
}

/// Subtract two non-negative integers. Returns None if b > a.
pub fn sub_multi(a: u64, b: u64) -> Option<MultiDigitResult> {
    if b > a { return None; }
    let mut result_digits: Vec<u8> = Vec::new();
    let mut steps: Vec<ArithmeticStep> = Vec::new();
    let mut borrow: u8 = 0;
    let max_digits = digit_count(a).max(digit_count(b));

    for pos in 0..max_digits {
        let da = digit_at(a, pos as u32).digit();
        let db = digit_at(b, pos as u32).digit();
        let a_adj = da as i8 - borrow as i8;
        let diff = a_adj - db as i8;
        let (rd, nb) = if diff < 0 { ((diff + 10) as u8, 1u8) } else { (diff as u8, 0u8) };

        steps.push(ArithmeticStep {
            position: pos, operand_a: Value(da), operand_b: Value(db),
            carry_in: borrow, intermediate: 0, result_digit: rd,
            carry_out: nb, rule: "value_core_subtraction",
        });
        result_digits.push(rd);
        borrow = nb;
    }

    let mut result: u64 = 0;
    for (i, &d) in result_digits.iter().enumerate() {
        result += (d as u64) * 10u64.pow(i as u32);
    }
    Some(MultiDigitResult { result, steps, verified: false })
}

/// Multiply two non-negative integers using Value Core long multiplication.
pub fn mul_multi(a: u64, b: u64) -> MultiDigitResult {
    if a == 0 || b == 0 {
        return MultiDigitResult { result: 0, steps: vec![], verified: false };
    }
    let mut steps: Vec<ArithmeticStep> = Vec::new();
    let mut partial_products: Vec<u64> = Vec::new();
    let b_digits = digit_count(b);

    for b_pos in 0..b_digits {
        let digit_b = digit_at(b, b_pos as u32);
        let mut carry: u8 = 0;
        let mut partial: u64 = 0;
        let a_digits = digit_count(a);

        for a_pos in 0..a_digits {
            let digit_a = digit_at(a, a_pos as u32);
            let product = mul(digit_a, digit_b);
            let total = product + carry;
            let rd = total % 10;
            let nc = total / 10;
            partial += (rd as u64) * 10u64.pow(a_pos as u32);

            steps.push(ArithmeticStep {
                position: a_pos + b_pos, operand_a: digit_a, operand_b: digit_b,
                carry_in: carry, intermediate: product, result_digit: rd,
                carry_out: nc, rule: "value_core_multiplication",
            });
            carry = nc;
        }
        if carry > 0 { partial += (carry as u64) * 10u64.pow(a_digits as u32); }
        partial *= 10u64.pow(b_pos as u32);
        partial_products.push(partial);
    }

    let result: u64 = partial_products.iter().sum();
    MultiDigitResult { result, steps, verified: false }
}

/// Divide two non-negative integers. Returns None for division by zero.
pub fn div_multi(a: u64, b: u64) -> Option<DivisionResult> {
    if b == 0 { return None; }
    Some(DivisionResult {
        quotient: a / b, remainder: a % b,
        dividend: a, divisor: b, verified: false,
    })
}

// ============================================================================
// VERIFICATION
// ============================================================================

pub fn verify_addition(a: u64, b: u64, result: u64) -> bool {
    if let Some(c1) = sub_multi(result, b) {
        if let Some(c2) = sub_multi(result, a) {
            return c1.result == a && c2.result == b;
        }
    }
    false
}

pub fn verify_subtraction(a: u64, b: u64, result: u64) -> bool {
    add_multi(result, b).result == a
}

pub fn verify_multiplication(a: u64, b: u64, result: u64) -> bool {
    if a == 0 || b == 0 { return result == 0; }
    if let Some(c1) = div_multi(result, b) {
        if let Some(c2) = div_multi(result, a) {
            return c1.quotient == a && c1.remainder == 0
                && c2.quotient == b && c2.remainder == 0;
        }
    }
    false
}

pub fn verify_division(a: u64, b: u64, quotient: u64, remainder: u64) -> bool {
    if b == 0 { return false; }
    let reconstructed = mul_multi(b, quotient);
    let total = add_multi(reconstructed.result, remainder);
    total.result == a && remainder < b
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ================================================================
    // VALUE CONSTRUCTION & PROPERTIES
    // ================================================================

    #[test]
    fn test_value_construction_valid() {
        for d in 0..=9u8 {
            let v = Value::new(d);
            assert!(v.is_some(), "Value::new({}) should succeed", d);
            assert_eq!(v.unwrap().digit(), d);
        }
    }

    #[test]
    fn test_value_construction_invalid() {
        for d in 10..=255u8 {
            assert!(Value::new(d).is_none(), "Value::new({}) should fail", d);
        }
    }

    #[test]
    fn test_value_must_valid() {
        for d in 0..=9u8 {
            assert_eq!(Value::must(d).digit(), d);
        }
    }

    #[test]
    #[should_panic]
    fn test_value_must_invalid() {
        Value::must(10);
    }

    #[test]
    fn test_value_quantity() {
        for d in 0..=9u8 {
            assert_eq!(Value::must(d).quantity(), d as u64);
        }
    }

    #[test]
    fn test_value_ordinal() {
        for d in 0..=9u8 {
            assert_eq!(Value::must(d).ordinal(), d);
        }
    }

    #[test]
    fn test_value_even_odd() {
        let even = [0, 2, 4, 6, 8];
        let odd = [1, 3, 5, 7, 9];
        for &d in &even {
            assert!(Value::must(d).is_even(), "{} should be even", d);
            assert!(!Value::must(d).is_odd(), "{} should not be odd", d);
        }
        for &d in &odd {
            assert!(!Value::must(d).is_even(), "{} should not be even", d);
            assert!(Value::must(d).is_odd(), "{} should be odd", d);
        }
    }

    #[test]
    fn test_value_prime() {
        let primes = [2, 3, 5, 7];
        let non_primes = [0, 1, 4, 6, 8, 9];
        for &d in &primes { assert!(Value::must(d).is_prime(), "{} should be prime", d); }
        for &d in &non_primes { assert!(!Value::must(d).is_prime(), "{} should not be prime", d); }
    }

    #[test]
    fn test_value_zero_one() {
        assert!(ZERO.is_zero()); assert!(!ZERO.is_one());
        assert!(ONE.is_one()); assert!(!ONE.is_zero());
        for d in 2..=9u8 { assert!(!Value::must(d).is_zero()); assert!(!Value::must(d).is_one()); }
    }

    #[test]
    fn test_complement_10() {
        let expected = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        for d in 0..=9u8 {
            assert_eq!(Value::must(d).complement_10(), expected[d as usize]);
        }
    }

    #[test]
    fn test_complement_9() {
        for d in 0..=9u8 { assert_eq!(Value::must(d).complement_9(), 9 - d); }
    }

    #[test]
    fn test_ordering() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                let va = Value::must(a); let vb = Value::must(b);
                assert_eq!(va < vb, a < b);
                assert_eq!(va > vb, a > b);
                assert_eq!(va == vb, a == b);
            }
        }
    }

    #[test]
    fn test_greater_lesser_values() {
        assert_eq!(ZERO.greater_values().len(), 9);
        assert_eq!(ZERO.lesser_values().len(), 0);
        assert_eq!(FIVE.greater_values().len(), 4);
        assert_eq!(FIVE.lesser_values().len(), 5);
        assert_eq!(NINE.greater_values().len(), 0);
        assert_eq!(NINE.lesser_values().len(), 9);
    }

    #[test]
    fn test_factors() {
        assert_eq!(ZERO.factors(), vec![]);
        assert_eq!(ONE.factors(), vec![1]);
        assert_eq!(TWO.factors(), vec![1, 2]);
        assert_eq!(THREE.factors(), vec![1, 3]);
        assert_eq!(FOUR.factors(), vec![1, 2, 4]);
        assert_eq!(FIVE.factors(), vec![1, 5]);
        assert_eq!(SIX.factors(), vec![1, 2, 3, 6]);
        assert_eq!(SEVEN.factors(), vec![1, 7]);
        assert_eq!(EIGHT.factors(), vec![1, 2, 4, 8]);
        assert_eq!(NINE.factors(), vec![1, 3, 9]);
    }

    #[test]
    fn test_divides() {
        assert!(!ZERO.divides(&FIVE));
        assert!(ONE.divides(&FIVE));
        assert!(TWO.divides(&FOUR));
        assert!(!TWO.divides(&THREE));
        assert!(THREE.divides(&NINE));
        assert!(THREE.divides(&SIX));
    }

    // ================================================================
    // EXHAUSTIVE ADDITION TABLE - All 100 facts
    // ================================================================

    #[test]
    fn test_addition_table_exhaustive() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                let expected = a + b;
                let retrieved = ADDITION_TABLE[a as usize][b as usize];
                assert_eq!(retrieved, expected, "ADD[{}][{}] = {} expected {}", a, b, retrieved, expected);
            }
        }
    }

    #[test]
    fn test_add_function_exhaustive() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                assert_eq!(add(Value::must(a), Value::must(b)), a + b, "add({}, {})", a, b);
            }
        }
    }

    #[test]
    fn test_addition_commutativity() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                assert_eq!(add(Value::must(a), Value::must(b)), add(Value::must(b), Value::must(a)));
            }
        }
    }

    #[test]
    fn test_addition_identity() {
        for d in 0..=9u8 {
            assert_eq!(add(Value::must(d), ZERO), d);
            assert_eq!(add(ZERO, Value::must(d)), d);
        }
    }

    // ================================================================
    // EXHAUSTIVE SUBTRACTION TABLE - All 100 facts
    // ================================================================

    #[test]
    fn test_subtraction_table_exhaustive() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                let expected = a as i8 - b as i8;
                assert_eq!(SUBTRACTION_TABLE[a as usize][b as usize], expected, "SUB[{}][{}]", a, b);
            }
        }
    }

    #[test]
    fn test_sub_function_exhaustive() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                assert_eq!(sub(Value::must(a), Value::must(b)), a as i8 - b as i8, "sub({}, {})", a, b);
            }
        }
    }

    #[test]
    fn test_subtraction_identity() {
        for d in 0..=9u8 { assert_eq!(sub(Value::must(d), ZERO), d as i8); }
    }

    #[test]
    fn test_subtraction_self() {
        for d in 0..=9u8 { assert_eq!(sub(Value::must(d), Value::must(d)), 0); }
    }

    // ================================================================
    // EXHAUSTIVE MULTIPLICATION TABLE - All 100 facts
    // ================================================================

    #[test]
    fn test_multiplication_table_exhaustive() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                let expected = a * b;
                assert_eq!(MULTIPLICATION_TABLE[a as usize][b as usize], expected, "MUL[{}][{}]", a, b);
            }
        }
    }

    #[test]
    fn test_mul_function_exhaustive() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                assert_eq!(mul(Value::must(a), Value::must(b)), a * b, "mul({}, {})", a, b);
            }
        }
    }

    #[test]
    fn test_multiplication_commutativity() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                assert_eq!(mul(Value::must(a), Value::must(b)), mul(Value::must(b), Value::must(a)));
            }
        }
    }

    #[test]
    fn test_multiplication_identity() {
        for d in 0..=9u8 {
            assert_eq!(mul(Value::must(d), ONE), d);
            assert_eq!(mul(ONE, Value::must(d)), d);
        }
    }

    #[test]
    fn test_multiplication_zero() {
        for d in 0..=9u8 {
            assert_eq!(mul(Value::must(d), ZERO), 0);
            assert_eq!(mul(ZERO, Value::must(d)), 0);
        }
    }

    // ================================================================
    // DIVISION FACTS
    // ================================================================

    #[test]
    fn test_div_by_zero() {
        for d in 0..=9u8 { assert!(div(Value::must(d), ZERO).is_none()); }
    }

    #[test]
    fn test_div_exhaustive() {
        for a in 0..=9u8 {
            for b in 1..=9u8 {
                let (q, r) = div(Value::must(a), Value::must(b)).unwrap();
                assert_eq!(q, a / b, "{}/{} quotient", a, b);
                assert_eq!(r, a % b, "{}/{} remainder", a, b);
            }
        }
    }

    #[test]
    fn test_div_identity() {
        for d in 1..=9u8 {
            let (q, r) = div(Value::must(d), ONE).unwrap();
            assert_eq!(q, d); assert_eq!(r, 0);
        }
    }

    #[test]
    fn test_div_self() {
        for d in 1..=9u8 {
            let (q, r) = div(Value::must(d), Value::must(d)).unwrap();
            assert_eq!(q, 1); assert_eq!(r, 0);
        }
    }

    // ================================================================
    // ADDITION-SUBTRACTION INVERSE
    // ================================================================

    #[test]
    fn test_addition_subtraction_inverse() {
        for a in 0..=9u8 {
            for b in 0..=9u8 {
                let sum = add(Value::must(a), Value::must(b));
                if sum <= 9 {
                    assert_eq!(sub(Value::must(sum), Value::must(a)), b as i8);
                }
            }
        }
    }

    // ================================================================
    // PLACE VALUE
    // ================================================================

    #[test]
    fn test_decompose_zero() {
        let c = decompose(0);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].value, ZERO);
        assert_eq!(c[0].place, 1);
    }

    #[test]
    fn test_decompose_single_digits() {
        for d in 0..=9u64 {
            let c = decompose(d);
            assert_eq!(c.len(), 1);
            assert_eq!(c[0].value.digit(), d as u8);
        }
    }

    #[test]
    fn test_decompose_347() {
        let c = decompose(347);
        assert_eq!(c.len(), 3);
        assert_eq!(c[0].value, THREE); assert_eq!(c[0].place, 100);
        assert_eq!(c[1].value, FOUR);  assert_eq!(c[1].place, 10);
        assert_eq!(c[2].value, SEVEN); assert_eq!(c[2].place, 1);
    }

    #[test]
    fn test_decompose_with_zeros() {
        let c = decompose(1000);
        assert_eq!(c.len(), 4);
        assert_eq!(c[0].value, ONE);  assert_eq!(c[0].place, 1000);
        assert_eq!(c[1].value, ZERO); assert_eq!(c[2].value, ZERO); assert_eq!(c[3].value, ZERO);
    }

    #[test]
    fn test_compose_decompose_roundtrip() {
        let vals = [0, 1, 5, 9, 10, 42, 99, 100, 347, 999, 1000, 12345, 99999, 123456789, 999999999];
        for &n in &vals {
            assert_eq!(compose(&decompose(n)), n, "roundtrip failed for {}", n);
        }
    }

    #[test]
    fn test_compose_decompose_exhaustive_0_to_999() {
        for n in 0..=999u64 {
            assert_eq!(compose(&decompose(n)), n, "roundtrip failed for {}", n);
        }
    }

    #[test]
    fn test_digit_count() {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(9), 1);
        assert_eq!(digit_count(10), 2);
        assert_eq!(digit_count(99), 2);
        assert_eq!(digit_count(100), 3);
        assert_eq!(digit_count(123456789), 9);
    }

    #[test]
    fn test_digit_at() {
        assert_eq!(digit_at(347, 0), SEVEN);
        assert_eq!(digit_at(347, 1), FOUR);
        assert_eq!(digit_at(347, 2), THREE);
        assert_eq!(digit_at(347, 3), ZERO);
    }

    // ================================================================
    // MULTI-DIGIT ADDITION
    // ================================================================

    #[test]
    fn test_add_multi_simple() {
        assert_eq!(add_multi(2, 3).result, 5);
        assert_eq!(add_multi(0, 0).result, 0);
        assert_eq!(add_multi(0, 5).result, 5);
        assert_eq!(add_multi(5, 0).result, 5);
    }

    #[test]
    fn test_add_multi_with_carry() {
        assert_eq!(add_multi(7, 8).result, 15);
        assert_eq!(add_multi(99, 1).result, 100);
        assert_eq!(add_multi(999, 1).result, 1000);
    }

    #[test]
    fn test_add_multi_larger() {
        assert_eq!(add_multi(347, 286).result, 633);
        assert_eq!(add_multi(12345, 67890).result, 80235);
        assert_eq!(add_multi(999999, 1).result, 1000000);
    }

    #[test]
    fn test_add_multi_commutative() {
        for &(a, b) in &[(347, 286), (12345, 67890), (1, 999999), (0, 0)] {
            assert_eq!(add_multi(a, b).result, add_multi(b, a).result);
        }
    }

    #[test]
    fn test_add_multi_exhaustive_0_to_100() {
        for a in 0..=100u64 {
            for b in 0..=100u64 {
                assert_eq!(add_multi(a, b).result, a + b, "{} + {}", a, b);
            }
        }
    }

    // ================================================================
    // MULTI-DIGIT SUBTRACTION
    // ================================================================

    #[test]
    fn test_sub_multi_simple() {
        assert_eq!(sub_multi(5, 3).unwrap().result, 2);
        assert_eq!(sub_multi(0, 0).unwrap().result, 0);
        assert_eq!(sub_multi(9, 0).unwrap().result, 9);
    }

    #[test]
    fn test_sub_multi_with_borrow() {
        assert_eq!(sub_multi(100, 1).unwrap().result, 99);
        assert_eq!(sub_multi(1000, 1).unwrap().result, 999);
        assert_eq!(sub_multi(633, 286).unwrap().result, 347);
    }

    #[test]
    fn test_sub_multi_negative() {
        assert!(sub_multi(3, 5).is_none());
        assert!(sub_multi(0, 1).is_none());
        assert!(sub_multi(100, 200).is_none());
    }

    #[test]
    fn test_sub_multi_exhaustive_0_to_100() {
        for a in 0..=100u64 {
            for b in 0..=a {
                assert_eq!(sub_multi(a, b).unwrap().result, a - b, "{} - {}", a, b);
            }
        }
    }

    // ================================================================
    // MULTI-DIGIT MULTIPLICATION
    // ================================================================

    #[test]
    fn test_mul_multi_simple() {
        assert_eq!(mul_multi(2, 3).result, 6);
        assert_eq!(mul_multi(0, 100).result, 0);
        assert_eq!(mul_multi(1, 12345).result, 12345);
    }

    #[test]
    fn test_mul_multi_larger() {
        assert_eq!(mul_multi(12, 12).result, 144);
        assert_eq!(mul_multi(99, 99).result, 9801);
        assert_eq!(mul_multi(123, 456).result, 56088);
        assert_eq!(mul_multi(347, 286).result, 99242);
    }

    #[test]
    fn test_mul_multi_commutative() {
        for &(a, b) in &[(12, 34), (99, 99), (123, 456), (1, 99999), (0, 12345)] {
            assert_eq!(mul_multi(a, b).result, mul_multi(b, a).result);
        }
    }

    #[test]
    fn test_mul_multi_exhaustive_0_to_50() {
        for a in 0..=50u64 {
            for b in 0..=50u64 {
                assert_eq!(mul_multi(a, b).result, a * b, "{} * {}", a, b);
            }
        }
    }

    // ================================================================
    // MULTI-DIGIT DIVISION
    // ================================================================

    #[test]
    fn test_div_multi_simple() {
        let r = div_multi(6, 3).unwrap();
        assert_eq!(r.quotient, 2); assert_eq!(r.remainder, 0);
    }

    #[test]
    fn test_div_multi_with_remainder() {
        let r = div_multi(7, 3).unwrap();
        assert_eq!(r.quotient, 2); assert_eq!(r.remainder, 1);
    }

    #[test]
    fn test_div_multi_by_zero() { assert!(div_multi(5, 0).is_none()); }

    #[test]
    fn test_div_multi_larger() {
        let r = div_multi(99242, 347).unwrap();
        assert_eq!(r.quotient, 286); assert_eq!(r.remainder, 0);
    }

    #[test]
    fn test_div_multi_exhaustive() {
        for a in 0..=200u64 {
            for b in 1..=50u64 {
                let r = div_multi(a, b).unwrap();
                assert_eq!(r.quotient, a / b, "{}/{} quotient", a, b);
                assert_eq!(r.remainder, a % b, "{}/{} remainder", a, b);
            }
        }
    }

    // ================================================================
    // VERIFICATION PROTOCOL
    // ================================================================

    #[test]
    fn test_verify_addition() {
        assert!(verify_addition(347, 286, 633));
        assert!(!verify_addition(347, 286, 634));
        assert!(verify_addition(0, 0, 0));
        assert!(verify_addition(999, 1, 1000));
    }

    #[test]
    fn test_verify_subtraction() {
        assert!(verify_subtraction(633, 286, 347));
        assert!(!verify_subtraction(633, 286, 348));
        assert!(verify_subtraction(1000, 1, 999));
    }

    #[test]
    fn test_verify_multiplication() {
        assert!(verify_multiplication(347, 286, 99242));
        assert!(!verify_multiplication(347, 286, 99243));
        assert!(verify_multiplication(0, 100, 0));
    }

    #[test]
    fn test_verify_division() {
        assert!(verify_division(7, 3, 2, 1));
        assert!(verify_division(99242, 347, 286, 0));
        assert!(!verify_division(7, 3, 3, 0));
        assert!(!verify_division(5, 0, 0, 0));
    }

    #[test]
    fn test_verify_addition_exhaustive_0_to_100() {
        for a in 0..=100u64 {
            for b in 0..=100u64 {
                assert!(verify_addition(a, b, a + b), "verify_add({}, {}, {})", a, b, a + b);
                if a + b > 0 {
                    assert!(!verify_addition(a, b, a + b + 1));
                }
            }
        }
    }

    #[test]
    fn test_verify_multiplication_exhaustive_0_to_30() {
        for a in 0..=30u64 {
            for b in 0..=30u64 {
                assert!(verify_multiplication(a, b, a * b), "verify_mul({}, {}, {})", a, b, a * b);
            }
        }
    }

    #[test]
    fn test_verify_division_exhaustive() {
        for a in 0..=200u64 {
            for b in 1..=30u64 {
                assert!(verify_division(a, b, a / b, a % b), "verify_div({}, {})", a, b);
            }
        }
    }

    // ================================================================
    // CROSS-OPERATION CONSISTENCY
    // ================================================================

    #[test]
    fn test_addition_subtraction_roundtrip() {
        for a in 0..=100u64 {
            for b in 0..=100u64 {
                let sum = add_multi(a, b).result;
                assert_eq!(sub_multi(sum, b).unwrap().result, a, "({} + {}) - {} != {}", a, b, b, a);
            }
        }
    }

    #[test]
    fn test_multiplication_division_roundtrip() {
        for a in 0..=50u64 {
            for b in 1..=50u64 {
                let product = mul_multi(a, b).result;
                let d = div_multi(product, b).unwrap();
                assert_eq!(d.quotient, a, "({} * {}) / {} != {}", a, b, b, a);
                assert_eq!(d.remainder, 0);
            }
        }
    }

    // ================================================================
    // MATHEMATICAL PROPERTIES
    // ================================================================

    #[test]
    fn test_distributive_property() {
        for a in 0..=20u64 {
            for b in 0..=20u64 {
                for c in 0..=20u64 {
                    let left = mul_multi(a, add_multi(b, c).result).result;
                    let right = add_multi(mul_multi(a, b).result, mul_multi(a, c).result).result;
                    assert_eq!(left, right, "{}*({} + {}) != {}*{} + {}*{}", a, b, c, a, b, a, c);
                }
            }
        }
    }

    #[test]
    fn test_addition_associativity() {
        for a in 0..=30u64 {
            for b in 0..=30u64 {
                for c in 0..=30u64 {
                    let left = add_multi(add_multi(a, b).result, c).result;
                    let right = add_multi(a, add_multi(b, c).result).result;
                    assert_eq!(left, right, "({} + {}) + {} != {} + ({} + {})", a, b, c, a, b, c);
                }
            }
        }
    }

    #[test]
    fn test_multiplication_associativity() {
        for a in 0..=15u64 {
            for b in 0..=15u64 {
                for c in 0..=15u64 {
                    let left = mul_multi(mul_multi(a, b).result, c).result;
                    let right = mul_multi(a, mul_multi(b, c).result).result;
                    assert_eq!(left, right, "({} * {}) * {} != {} * ({} * {})", a, b, c, a, b, c);
                }
            }
        }
    }
}
