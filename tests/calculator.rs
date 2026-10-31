use vyxcalc::calculator::{Calculator, CalculatorError};

#[test]
fn test_basic_arithmetic() {
    let calc = Calculator::new();
    
    assert_eq!(calc.evaluate("2 + 3").unwrap(), 5.0);
    assert_eq!(calc.evaluate("10 - 4").unwrap(), 6.0);
    assert_eq!(calc.evaluate("3 * 7").unwrap(), 21.0);
    assert_eq!(calc.evaluate("15 / 3").unwrap(), 5.0);
}

#[test]
fn test_multiple_operations_left_to_right() {
    let calc = Calculator::new();

    // Left-to-right: (5 + 2) * 6 = 42, not BIDMAS 17
    assert_eq!(calc.evaluate("5 + 2 * 6").unwrap(), 42.0);
    // (8 / 2) + 3 = 7
    assert_eq!(calc.evaluate("8 / 2 + 3").unwrap(), 7.0);
    // (20 - 10) / 2 = 5
    assert_eq!(calc.evaluate("20 - 10 / 2").unwrap(), 5.0);
    // (1 + 2 + 3) * 4 = 24
    assert_eq!(calc.evaluate("1 + 2 + 3 * 4").unwrap(), 24.0);
}

#[test]
fn test_multiple_operations_with_extra_whitespace() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("  5  +  2  *  6  ").unwrap(), 42.0);
}

#[test]
fn test_division_by_zero() {
    let calc = Calculator::new();
    assert!(matches!(calc.evaluate("5 / 0"), Err(CalculatorError::DivisionByZero)));
}

#[test]
fn test_division_by_zero_in_chain() {
    let calc = Calculator::new();
    assert!(matches!(
        calc.evaluate("5 / 0 * 6"),
        Err(CalculatorError::DivisionByZero)
    ));
}

#[test]
fn test_invalid_expression() {
    let calc = Calculator::new();
    assert!(matches!(calc.evaluate("2 +"), Err(CalculatorError::InvalidExpression)));
    assert!(matches!(calc.evaluate("invalid"), Err(CalculatorError::InvalidExpression)));
    assert!(matches!(calc.evaluate("5 + 2 *"), Err(CalculatorError::InvalidExpression)));
    assert!(matches!(calc.evaluate("5 + * 6"), Err(CalculatorError::InvalidExpression)));
}

#[test]
fn test_invalid_numbers() {
    let calc = Calculator::new();
    assert!(matches!(calc.evaluate("abc + 3"), Err(CalculatorError::InvalidNumber(_))));
}
