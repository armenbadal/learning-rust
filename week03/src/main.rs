#[derive(Debug,PartialEq, Eq, Clone)]
enum Token {
    Number,
    Identifier,
    Add,
    Sub,
    Mul,
    Div,
    LeftPar,
    RightPar,
    Eos,
}

#[derive(Debug, Clone)]
struct Lexeme<'a> {
    token: Token,
    value: &'a str,
}

struct Scanner<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Scanner<'a> {
    fn scan(&mut self) -> Option<Lexeme<'a>> {
        self.skip_whitespace();

        if !self.ok() {
            return None;
        }

        let begin = self.position;
        let c = self.peek();
        let token = if Self::is_digit(c) {
            self.scan_while(Self::is_digit);
            Token::Number
        } else if Self::is_letter(c) {
            self.scan_while(Self::is_letter);
            Token::Identifier
        } else if c == '+' {
            self.advance();
            Token::Add
        } else if c == '-' {
            self.advance();
            Token::Sub
        } else if c == '*' {
            self.advance();
            Token::Mul
        } else if c == '/' {
            self.advance();
            Token::Div
        } else if c == '(' {
            self.advance();
            Token::LeftPar
        } else if c == ')' {
            self.advance();
            Token::RightPar
        } else {
            self.advance();
            Token::Eos
        };

        Some(Lexeme {
            token,
            value: &self.source[begin..self.position],
        })
    }

    fn skip_whitespace(&mut self) {
        while self.ok() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn scan_while(&mut self, predicate: fn(char) -> bool) -> usize {
        let begin = self.position;
        while self.ok() && predicate(self.peek()) {
            self.advance();
        }
        self.position - begin
    }

    fn is_letter(c: char) -> bool {
        c.is_alphabetic()
    }
    fn is_digit(c: char) -> bool {
        c.is_numeric()
    }
    fn peek(&self) -> char {
        self.source[self.position..]
            .chars()
            .next()
            .expect("scanner position must be on a character")
    }
    fn advance(&mut self) {
        self.position += self.peek().len_utf8();
    }
    fn ok(&self) -> bool {
        self.position < self.source.len()
    }
}

#[derive(Debug)]
enum UnaryOp {
    Plus,
    Minus,
}

#[derive(Debug)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expr {
    Number(f64),

    Variable(String),

    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },

    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Variable(name) => write!(f, "{}", name),
            Expr::Unary { op, operand } => {
                let op_str = match op {
                    UnaryOp::Plus => "+",
                    UnaryOp::Minus => "-",
                };
                write!(f, "({}{})", op_str, operand)
            }
            Expr::Binary { op, left, right } => {
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                };
                write!(f, "({} {} {})", left, op_str, right)
            }
        }
    }
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    lookahead: Option<Lexeme<'a>>,
}

impl<'a> Parser<'a> {
    fn parse(&mut self)  -> Result<Expr, String> {
        self.lookahead = self.scanner.scan();
        let expr = self.parse_expression()?;
        if let Some(lk) = &self.lookahead {
            Err(format!("Unexpected token: {:?}", lk.token))
        } else {
            Ok(expr)
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplication()?;
        while let Some(lk) = &self.lookahead {
            if lk.token == Token::Add || lk.token == Token::Sub {
                let op = if lk.token == Token::Add { BinaryOp::Add } else { BinaryOp::Sub };
                self.check(lk.token.clone())?;
                let right = self.parse_multiplication()?;
                left = Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while let Some(lk) = &self.lookahead {
            if lk.token == Token::Mul || lk.token == Token::Div {
                let op = if lk.token == Token::Mul { BinaryOp::Mul } else { BinaryOp::Div };
                self.check(lk.token.clone())?;
                let right = self.parse_unary()?;
                left = Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        let un_op = if let Some(lk) = &self.lookahead {
            if lk.token == Token::Add || lk.token == Token::Sub {
                let op = if lk.token == Token::Add { UnaryOp::Plus } else { UnaryOp::Minus };
                self.check(lk.token.clone())?;
                Some(op)
            } else {
                None
            }
        } else {
            None
        };
        let operand = self.parse_primary()?;
        match un_op {
            Some(op) => Ok(Expr::Unary {
                op,
                operand: Box::new(operand),
            }),
            None => Ok(operand),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        if let Some(lk) = &self.lookahead {
            match lk.token {
                Token::Number => {
                    let value: f64 = lk.value.parse().map_err(|_| format!("Invalid number: {}", lk.value))?;
                    self.check(Token::Number)?;
                    Ok(Expr::Number(value))
                }
                Token::Identifier => {
                    let name = lk.value.to_string();
                    self.check(Token::Identifier)?;
                    Ok(Expr::Variable(name))
                }
                Token::LeftPar => {
                    self.check(Token::LeftPar)?;
                    let expr = self.parse_expression()?;
                    self.check(Token::RightPar)?;
                    Ok(expr)
                }
                _ => Err(format!("Unexpected token: {:?}", lk.token)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn check(&mut self, expected: Token) -> Result<(), String> {
        if let Some(lk) = &self.lookahead {
            if lk.token == expected {
                self.lookahead = self.scanner.scan();
                Ok(())
            } else {
                Err(format!("Expected token {:?}, but found {:?}", expected, lk.token))
            }
        } else {
            Err(format!("Expected token {:?}, but found end of input", expected))
        }
    }
}

fn main() {
    let text0 = "x + 3 * (y - 2)";
    let scanner0 = Scanner { source: text0, position: 0 };
    let mut parser0 = Parser { scanner: scanner0, lookahead: None };
    match parser0.parse() {
        Ok(expr) => println!("Parsed expression: {expr}"),
        Err(e) => println!("Error parsing expression: {}", e),
    }
}
