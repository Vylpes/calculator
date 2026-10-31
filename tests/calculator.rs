use vyxcalc::calculator::{Calculator, CalculatorError};
use vyxcalc::session_history::SessionHistory;

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

#[test]
fn test_session_history_cycles_backward_and_forward() {
    let mut history = SessionHistory::new();
    history.save("2 + 3");
    history.save("10 - 4");

    assert_eq!(history.previous(), Some("10 - 4"));
    assert_eq!(history.previous(), Some("2 + 3"));
    assert_eq!(history.next(), Some("10 - 4"));
    assert_eq!(history.next(), Some(""));
}

#[test]
fn test_session_history_skips_empty_submissions() {
    let mut history = SessionHistory::new();
    history.save("2 + 3");
    assert!(!history.save(""));
    assert!(!history.save("   "));
    history.save("4 + 5");

    assert_eq!(history.previous(), Some("4 + 5"));
    assert_eq!(history.previous(), Some("2 + 3"));
}

#[test]
fn test_session_history_is_session_scoped() {
    let mut first = SessionHistory::new();
    first.save("7 - 1");

    let mut second = SessionHistory::new();
    assert!(second.is_empty());
    assert_eq!(second.previous(), None);
}