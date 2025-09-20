use calculator_app::calculator::{Calculator, CalculatorError};

#[test]
fn test_basic_arithmetic() {
    let calc = Calculator::new();
    
    assert_eq!(calc.evaluate("2 + 3").unwrap(), 5.0);
    assert_eq!(calc.evaluate("10 - 4").unwrap(), 6.0);
    assert_eq!(calc.evaluate("3 * 7").unwrap(), 21.0);
    assert_eq!(calc.evaluate("15 / 3").unwrap(), 5.0);
}

#[test]
fn test_division_by_zero() {
    let calc = Calculator::new();
    assert!(matches!(calc.evaluate("5 / 0"), Err(CalculatorError::DivisionByZero)));
}

#[test]
fn test_invalid_expression() {
    let calc = Calculator::new();
    assert!(matches!(calc.evaluate("2 +"), Err(CalculatorError::InvalidExpression)));
    assert!(matches!(calc.evaluate("invalid"), Err(CalculatorError::InvalidExpression)));
}

#[test]
fn test_invalid_numbers() {
    let calc = Calculator::new();
    assert!(matches!(calc.evaluate("abc + 3"), Err(CalculatorError::InvalidNumber(_))));
}