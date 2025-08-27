use crate::token::Token;
use crate::ast::{Expr, BinOp, Stmt};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.position)
            .cloned()
            .unwrap_or(Token::EOF)
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek();
        self.position += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) {
        let actual = self.advance();
        if &actual != expected {
            panic!("Expected {:?}, got {:?}", expected, actual);
        }
    }

    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek() {
            Token::Let => {
                self.advance();
                if let Token::Identifier(name) = self.advance() {
                    self.expect(&Token::Equals);
                    let expr = self.parse_expr();
                    self.expect(&Token::Semicolon);
                    Some(Stmt::Let { name, expr })
                } else {
                    panic!("Expected identifier after 'let'");
                }
            }
    
            Token::Print => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(&Token::Semicolon);
                Some(Stmt::Print { expr })
            }
    
            Token::If => {
                self.advance(); // consume `if`
                let condition = self.parse_expr();
                let then_branch = self.parse_block();
                let else_branch = if self.peek() == Token::Else {
                    self.advance(); // consume `else`
                    self.parse_block()
                } else {
                    vec![]
                };
    
                Some(Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                })
            }

            Token::While => {
                self.advance(); // consume `while`
                let condition = self.parse_expr();
                let body = self.parse_block(); // reuse same block logic
                Some(Stmt::While { condition, body })
            }
    
            Token::EOF => None,
    
            other => panic!("Unexpected token in statement: {:?}", other),
        }
    }
    
    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
    
        while let Some(stmt) = self.parse_stmt() {
            statements.push(stmt);
        }
    
        statements
    }
    

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.expect(&Token::LBrace);
    
        let mut stmts = vec![];
        while self.peek() != Token::RBrace && self.peek() != Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
        }
    
        self.expect(&Token::RBrace);
        stmts
    }
    

    fn parse_expr(&mut self) -> Expr {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_add_sub();
    
        loop {
            match self.peek() {
                Token::Eq => {
                    self.advance();
                    let rhs = self.parse_add_sub();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Eq,
                        right: Box::new(rhs),
                    };
                }
                Token::NotEq => {
                    self.advance();
                    let rhs = self.parse_add_sub();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Ne,
                        right: Box::new(rhs),
                    };
                }
                Token::Less => {
                    self.advance();
                    let rhs = self.parse_add_sub();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Lt,
                        right: Box::new(rhs),
                    };
                }
                Token::LessEq => {
                    self.advance();
                    let rhs = self.parse_add_sub();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Le,
                        right: Box::new(rhs),
                    };
                }
                Token::Greater => {
                    self.advance();
                    let rhs = self.parse_add_sub();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Gt,
                        right: Box::new(rhs),
                    };
                }
                Token::GreaterEq => {
                    self.advance();
                    let rhs = self.parse_add_sub();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Ge,
                        right: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }
    
        expr
    }
    

    fn parse_add_sub(&mut self) -> Expr {
        let mut expr = self.parse_mul_div();

        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let rhs = self.parse_mul_div();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Add,
                        right: Box::new(rhs),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let rhs = self.parse_mul_div();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Sub,
                        right: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_mul_div(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let rhs = self.parse_primary();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Mul,
                        right: Box::new(rhs),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let rhs = self.parse_primary();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: BinOp::Div,
                        right: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Token::Number(n) => Expr::Number(n),
            Token::Identifier(name) => Expr::Identifier(name.clone()),
            Token::LParen => {
                let expr = self.parse_expr();
                self.expect(&Token::RParen);
                expr
            }
            other => panic!("Unexpected token in expression: {:?}", other),
        }
    }
}
