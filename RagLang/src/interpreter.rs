use std::collections::HashMap;
use crate::ast::{Expr, Stmt, BinOp};

pub struct Interpreter {
    env: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    pub fn exec(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let { name, expr } => {
                let value = self.eval(expr);
                self.env.insert(name, value);
            }
            Stmt::Print { expr } => {
                let value = self.eval(expr);
                println!("{}", value);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.eval(condition);
                if cond_val != 0 {
                    for stmt in then_branch {
                        self.exec(stmt);
                    }
                } else {
                    for stmt in else_branch {
                        self.exec(stmt);
                    }
                }
            }
            Stmt::While { condition, body } => {
                while self.eval(condition.clone()) != 0 {
                    for stmt in &body {
                        self.exec(stmt.clone());
                    }
                }
            }
        }
    }

    pub fn exec_program(&mut self, program: Vec<Stmt>) {
        for stmt in program {
            self.exec(stmt);
        }
    }
    

    fn eval(&mut self, expr: Expr) -> i64 {
        match expr {
            Expr::Number(n) => n,
            Expr::Identifier(name) => {
                *self.env.get(&name).expect(&format!("Undefined variable '{}'", name))
            }
            Expr::BinaryOp { left, op, right } => {
                let lval = self.eval(*left);
                let rval = self.eval(*right);
                match op {
                    BinOp::Add => lval + rval,
                    BinOp::Sub => lval - rval,
                    BinOp::Mul => lval * rval,
                    BinOp::Div => {
                        if rval == 0 {
                            panic!("Division by zero");
                        }
                        lval / rval
                    }
                    BinOp::Gt => (lval > rval) as i64,
                    BinOp::Lt => (lval < rval) as i64,
                    BinOp::Eq => (lval == rval) as i64,
                    BinOp::Ne => (lval != rval) as i64,
                    BinOp::Ge => (lval >= rval) as i64,
                    BinOp::Le => (lval <= rval) as i64,
                }
            }
        }
    }
}
