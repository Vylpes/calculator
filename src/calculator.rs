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

impl Operation {
    fn from_str(token: &str) -> Option<Operation> {
        match token {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            _ => None,
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
        let parts: Vec<&str> = expression.split_whitespace().collect();

        // Expect "number operator number [operator number]..." (odd length, >= 3)
        if parts.len() < 3 || parts.len() % 2 == 0 {
            return Err(CalculatorError::InvalidExpression);
        }

        let mut result = parts[0]
            .parse::<f64>()
            .map_err(|_| CalculatorError::InvalidNumber(parts[0].to_string()))?;

        let mut i = 1;
        while i < parts.len() {
            let operation = Operation::from_str(parts[i])
                .ok_or(CalculatorError::InvalidExpression)?;

            let right = parts[i + 1]
                .parse::<f64>()
                .map_err(|_| CalculatorError::InvalidNumber(parts[i + 1].to_string()))?;

            result = self.calculate(result, right, operation)?;
            i += 2;
        }

        Ok(result)
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
