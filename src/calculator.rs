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
    fn from_char(ch: char) -> Option<Operation> {
        match ch {
            '+' => Some(Operation::Add),
            '-' => Some(Operation::Subtract),
            '*' => Some(Operation::Multiply),
            '/' => Some(Operation::Divide),
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
        // Ignore whitespace so "2+3", "2 + 3", and "2  +  3" all parse the same.
        let compact: String = expression.chars().filter(|c| !c.is_whitespace()).collect();

        if compact.is_empty() {
            return Err(CalculatorError::InvalidExpression);
        }

        let chars: Vec<char> = compact.chars().collect();
        let mut i = 0;
        let has_operator = chars.iter().skip(1).any(|ch| matches!(ch, '+' | '-' | '*' | '/'));

        let mut result = match Self::parse_number(&chars, &mut i) {
            Ok(n) => n,
            // No operator at all (e.g. "invalid") → InvalidExpression; otherwise InvalidNumber
            Err(CalculatorError::InvalidNumber(_)) if !has_operator => {
                return Err(CalculatorError::InvalidExpression);
            }
            Err(e) => return Err(e),
        };

        // Expect "number operator number [operator number]..."
        if i >= chars.len() {
            return Err(CalculatorError::InvalidExpression);
        }

        while i < chars.len() {
            let operation = Operation::from_char(chars[i]).ok_or(CalculatorError::InvalidExpression)?;
            i += 1;

            if i >= chars.len() {
                return Err(CalculatorError::InvalidExpression);
            }

            let right = Self::parse_number(&chars, &mut i)?;
            result = self.calculate(result, right, operation)?;
        }

        Ok(result)
    }

    /// Parse a number starting at `i`, advancing `i` past the number.
    /// Allows an optional leading `-` (unary minus / negative number).
    fn parse_number(chars: &[char], i: &mut usize) -> Result<f64, CalculatorError> {
        let start = *i;

        if *i < chars.len() && chars[*i] == '-' {
            *i += 1;
        }

        let num_start = *i;
        while *i < chars.len() && (chars[*i].is_ascii_digit() || chars[*i] == '.') {
            *i += 1;
        }

        if *i == num_start {
            // No digits after optional sign — not a valid number
            let bad: String = chars[start..chars.len().min(start + 1)].iter().collect();
            if Operation::from_char(chars.get(start).copied().unwrap_or('\0')).is_some() {
                return Err(CalculatorError::InvalidExpression);
            }
            return Err(CalculatorError::InvalidNumber(bad));
        }

        let number_str: String = chars[start..*i].iter().collect();
        number_str
            .parse::<f64>()
            .map_err(|_| CalculatorError::InvalidNumber(number_str))
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
