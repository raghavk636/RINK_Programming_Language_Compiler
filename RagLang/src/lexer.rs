use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    chars: Vec<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            chars: input.chars().collect(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.position += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.advance() {
            Some('=') => {
                if self.peek() == Some('=') {
                    self.advance(); // consume second '='
                    Token::Eq
                } else {
                    Token::Equals
                }
            }
            Some('!') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::NotEq
                } else {
                    Token::Illegal('!')
                }
            }
            Some('<') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::LessEq
                } else {
                    Token::Less
                }
            }
            Some('>') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::GreaterEq
                } else {
                    Token::Greater
                }
            }            
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Star,
            Some('/') => Token::Slash,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(';') => Token::Semicolon,
            Some(ch) if ch.is_ascii_digit() => {
                let mut number = ch.to_string();
                while let Some(next) = self.peek() {
                    if next.is_ascii_digit() {
                        number.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Number(number.parse().unwrap())
            }
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                let mut ident = ch.to_string();
                while let Some(next) = self.peek() {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        ident.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "let" => Token::Let,
                    "print" => Token::Print,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    _ => Token::Identifier(ident),
                }
            }
            None => Token::EOF,
            Some(ch) => Token::Illegal(ch),
        }
    }
}
