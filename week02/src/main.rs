use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Debug)]
enum Token {
    Word,
    Number,
    Symbol,
}

#[derive(Debug)]
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
            Token::Word
        } else {
            self.advance();
            Token::Symbol
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

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("example01.txt")?);
    let mut output = BufWriter::new(File::create("result01.txt")?);

    for line in input.lines() {
        let line = line?;
        let mut scanner = Scanner {
            source: &line,
            position: 0,
        };

        while let Some(lexeme) = scanner.scan() {
            writeln!(output, "{:?}\t{:?}", lexeme.token, lexeme.value)?;
        }
    }

    output.flush()
}
