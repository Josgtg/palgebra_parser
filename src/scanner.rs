use crate::token::Token;
use crate::errors;


pub fn scan(proposition: &str) -> (Vec<Token>, bool) {
    let mut scanner = Scanner::new();
    scanner.scan(proposition)
}


fn ignore(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\0' || c == '\n' || c == '\r'
}


struct Scanner {
    tokens: Vec<Token>,
    error: bool,
    idx: u32
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            tokens: Vec::new(),
            error: false,
            idx: 0
        }
    }

    fn scan(&mut self, proposition: &str) -> (Vec<Token>, bool) {
        for c in proposition.chars() {
            if ignore(c) {
                continue;
            }
            self.match_token(c);
            self.idx += 1;
        }

        (self.tokens.clone(), self.error)
    }

    fn match_token(&mut self, c: char) {
        match c {
            '&' => self.tokens.push(Token::And),
            '|' => self.tokens.push(Token::Or),
            '!' => self.tokens.push(Token::Not),
            '~' => self.tokens.push(Token::IfOnlyIf),
            '>' => self.tokens.push(Token::IfThen),
            '(' => self.tokens.push(Token::LeftParen),
            ')' => self.tokens.push(Token::RightParen),
            _ => {
                if !c.is_alphabetic() {
                    self.error = true;
                    errors::report(&format!("unexpected character \"{}\"", c), 0, self.idx);
                    self.tokens.push(Token::Invalid);
                    return
                }
                // self.sentences.insert(c);
                self.tokens.push(Token::Sentence(c));
            }
        }
    }
}
