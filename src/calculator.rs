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
        let expression = expression.trim();
        
        // Simple parser for basic arithmetic expressions like "2 + 3"
        let parts: Vec<&str> = expression.split_whitespace().collect();
        
        if parts.len() != 3 {
            return Err(CalculatorError::InvalidExpression);
        }

        let left = parts[0].parse::<f64>()
            .map_err(|_| CalculatorError::InvalidNumber(parts[0].to_string()))?;
        
        let operator = parts[1];
        
        let right = parts[2].parse::<f64>()
            .map_err(|_| CalculatorError::InvalidNumber(parts[2].to_string()))?;

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

