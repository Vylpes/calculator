use std::fmt;

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum CalculatorError {
    DivisionByZero,
    InvalidExpression,
    InvalidNumber(String),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Error: Division by zero"),
            CalculatorError::InvalidExpression => write!(f, "Error: Invalid expression"),
            CalculatorError::InvalidNumber(s) => write!(f, "Error: Invalid number '{}'", s),
        }
    }
}

pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn evaluate(&self, expression: &str) -> Result<f64, CalculatorError> {
        // Ignore whitespace so "2+3", "2 + 3", and "2  +  3" all parse the same.
        let compact: String = expression.chars().filter(|c| !c.is_whitespace()).collect();

        if compact.is_empty() {
            return Err(CalculatorError::InvalidExpression);
        }

        // Find the binary operator. Skip index 0 so a leading '-' stays part of the left number.
        let op_pos = compact
            .char_indices()
            .skip(1)
            .find_map(|(i, ch)| match ch {
                '+' | '-' | '*' | '/' => Some(i),
                _ => None,
            });

        let Some(op_pos) = op_pos else {
            return Err(CalculatorError::InvalidExpression);
        };

        let left_str = &compact[..op_pos];
        let operator = &compact[op_pos..op_pos + 1];
        let right_str = &compact[op_pos + 1..];

        if left_str.is_empty() || right_str.is_empty() {
            return Err(CalculatorError::InvalidExpression);
        }

        let left = left_str
            .parse::<f64>()
            .map_err(|_| CalculatorError::InvalidNumber(left_str.to_string()))?;

        let right = right_str
            .parse::<f64>()
            .map_err(|_| CalculatorError::InvalidNumber(right_str.to_string()))?;

        let operation = match operator {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => return Err(CalculatorError::InvalidExpression),
        };

        self.calculate(left, right, operation)
    }

    pub fn calculate(&self, left: f64, right: f64, operation: Operation) -> Result<f64, CalculatorError> {
        match operation {
            Operation::Add => Ok(left + right),
            Operation::Subtract => Ok(left - right),
            Operation::Multiply => Ok(left * right),
            Operation::Divide => {
                if right == 0.0 {
                    Err(CalculatorError::DivisionByZero)
                } else {
                    Ok(left / right)
                }
            }
        }
    }
}

