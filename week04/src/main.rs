use std::collections::HashMap;

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

enum Expr {
    Number(f64),
    Variable(String),
    Unary(UnaryOp,Box<Expr>),
    Binary(BinaryOp,Box<Expr>,Box<Expr>)
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
                left = Expr::Binary(op, Box::new(left), Box::new(right));
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
                left = Expr::Binary(op, Box::new(left), Box::new(right));
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
            Some(op) => Ok(Expr::Unary(op, Box::new(operand))),
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



trait ExprVisitor {
    type Output;

    fn visit_number(&self, value: f64) -> Self::Output;
    fn visit_variable(&self, name: &str) -> Self::Output;
    fn visit_unary(&self, operation: &UnaryOp, right: &Expr) -> Self::Output;
    fn visit_binary(&self, operation: &BinaryOp, left: &Expr, right: &Expr) -> Self::Output;
}

impl Expr {
    fn accept<V>(&self, visitor: &V) -> V::Output
    where
        V: ExprVisitor + ?Sized
    {
        match self {
            Expr::Number(value) =>
                visitor.visit_number(*value),
            Expr::Variable(name) =>
                visitor.visit_variable(name),
            Expr::Unary(o, r) =>
                visitor.visit_unary(o, r),
            Expr::Binary(o, l, r) =>
                visitor.visit_binary(o, l, r)
        }
    }
}

struct Evaluator {
    env: HashMap<String,f64>
}

impl Evaluator {
    fn new() -> Self {
        Self { env: HashMap::new() }
    }
}

impl ExprVisitor for Evaluator {
    type Output = Result<f64, String>;

    fn visit_number(&self, value: f64) -> Self::Output {
        Ok(value)
    }

    fn visit_variable(&self, name: &str) -> Self::Output {
        self.env.get(name).copied().ok_or_else(|| format!("Unknown variable: {name}"))
    }

    fn visit_unary(&self, operation: &UnaryOp, right: &Expr) -> Self::Output {
        let rv = right.accept(self)?;
        match operation {
            UnaryOp::Plus => Ok(rv),
            UnaryOp::Minus => Ok(-rv),
        }
    }

    fn visit_binary(&self, operation: &BinaryOp, left: &Expr, right: &Expr) -> Self::Output {
        let lv = left.accept(self)?;
        let rv = right.accept(self)?;
        Ok(match operation {
            BinaryOp::Add => lv + rv,
            BinaryOp::Sub => lv - rv,
            BinaryOp::Mul => lv * rv,
            BinaryOp::Div => lv / rv,
        })
    }
}

struct PrettyPrinter;

impl ExprVisitor for PrettyPrinter {
    type Output = String;

    fn visit_number(&self, value: f64) -> Self::Output {
        format!("{value}")
    }

    fn visit_variable(&self, name: &str) -> Self::Output {
        name.to_string()
    }

    fn visit_unary(&self, operation: &UnaryOp, right: &Expr) -> Self::Output {
        let rs = right.accept(self);
        let ops = match operation {
            UnaryOp::Plus => "+",
            UnaryOp::Minus => "-",
        };
        format!("({ops} {rs})")
    }

    fn visit_binary(&self, operation: &BinaryOp, left: &Expr, right: &Expr) -> Self::Output {
        let ls = left.accept(self);
        let rs = right.accept(self);
        let ops = match operation {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
        };
        format!("({ls} {ops} {rs})")
    }
}

fn main() {
    let text0 = "x + 3 * (y - 2)";
    let scanner0 = Scanner { source: text0, position: 0 };
    let mut parser0 = Parser { scanner: scanner0, lookahead: None };
    match parser0.parse() {
        Ok(expr) => {
            let mut ev = Evaluator::new();
            ev.env.insert("x".into(), 888f64);
            ev.env.insert("y".into(), 777f64);
            let result = expr.accept(&ev).unwrap();
            let es = expr.accept(&PrettyPrinter{});
            println!("{es} = {result}")
        },
        Err(e) =>
            println!("Error parsing expression: {}", e),
    }
}
