
#[derive(Debug,Copy,Clone)]
enum Operation {
    ADD, SUB, MUL, DIV, OPEN, CLOSE
}

#[derive(Debug)]
enum Lexeme {
    Op(Operation),
    Number(f64),
    Eos
}

struct Scanner {
    source: String
}

impl Scanner {
    fn new(text: String) -> Self {
        Self { source: text }
    }

    fn scan(&mut self) -> Result<Lexeme, String> {
        let trimmed = self.source.trim_start();
        let whitespace_length = self.source.len() - trimmed.len();
        self.source.drain(..whitespace_length);

        let Some(first) = self.source.chars().next() else {
            return Ok(Lexeme::Eos);
        };

        if first.is_ascii_digit() || first == '.' {
            let mut end = 0;
            for character in self.source.chars() {
                if character.is_ascii_digit() || character == '.' {
                    end += character.len_utf8();
                } else {
                    break;
                }
            }

            let value = self.source[..end].to_string();
            self.source.drain(..end);

            let number = value
                .parse::<f64>()
                .map_err(|_| format!("invalid number: {value}"))?;
            return Ok(Lexeme::Number(number));
        }

        self.source.drain(..first.len_utf8());
        match first {
            '+' => Ok(Lexeme::Op(Operation::ADD)),
            '-' => Ok(Lexeme::Op(Operation::SUB)),
            '*' => Ok(Lexeme::Op(Operation::MUL)),
            '/' => Ok(Lexeme::Op(Operation::DIV)),
            '(' => Ok(Lexeme::Op(Operation::OPEN)),
            ')' => Ok(Lexeme::Op(Operation::CLOSE)),
            symbol => Err(format!("invalid symbol: {symbol}")),
        }
    }
}

struct Calculator {}

impl Calculator {
    fn evaluate(expr: String) -> Result<f64, String> {
        let mut scanner = Scanner::new(format!("({})", expr));

        let mut operands = Vec::new();
        let mut operators = Vec::new();

        loop {
            let lexeme = scanner.scan().map_err(|error| format!("Scanning error: {error}"))?;

            match lexeme {
                Lexeme::Number(value) => operands.push(value),
                Lexeme::Op(Operation::OPEN) => operators.push(Operation::OPEN),
                Lexeme::Op(Operation::CLOSE) => {
                    while let Some(operation) = operators.pop() {
                        if matches!(operation, Operation::OPEN) {
                            break;
                        }
                        let right = operands.pop().ok_or("empty value stack".to_string())?;
                        let left = operands.pop().ok_or("empty value stack".to_string())?;
                        operands.push(Self::calculate(operation, left, right));
                    }
                },
                Lexeme::Op(operation) => {
                    let pc = Self::priority(operation);
                    while operators.last().is_some_and(|op| Self::priority(*op) >= pc) {
                        let previous = operators.pop().unwrap();
                        let right = operands.pop().ok_or("empty value stack".to_string())?;
                        let left = operands.pop().ok_or("empty value stack".to_string())?;
                        let result = Self::calculate(previous, left, right);
                        operands.push(result);
                    }
                    operators.push(operation);
                },
                Lexeme::Eos => break
            }

        }

        match operands.last() {
            Some(value) => Ok(*value),
            None => Err(String::from("error"))
        }    
    }

    fn calculate(operation: Operation, left: f64, right: f64) -> f64 {
        match operation {
            Operation::ADD => left + right,
            Operation::SUB => left - right,
            Operation::MUL => left * right,
            Operation::DIV => left / right,
            _ => 0.0
        }
    }

    fn priority(operation: Operation) -> i8 {
        match operation {
            Operation::ADD | Operation::SUB => 1,
            Operation::MUL | Operation::DIV => 2,
            _ => 0
        }
    }
}


fn main() {
    let result = Calculator::evaluate(String::from("1+2"));
    println!("Result = {}", result.unwrap())
}
