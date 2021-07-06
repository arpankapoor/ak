use crate::k::{Verb, K};
use crate::parser::ASTNode;
use crate::span::Spanned;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Error {
    location: usize,
    code: ErrorCode,
}

#[derive(Debug)]
pub enum ErrorCode {
    Nyi,
}

impl ASTNode {
    pub fn interpret(self) -> Result<K, Error> {
        match self {
            ASTNode::Expr(Spanned(_, _, k)) => Ok(k),
            ASTNode::Apply(Spanned(_, _, (value, args))) => {
                let mut kargs = VecDeque::with_capacity(args.len());
                for item in args.into_iter().rev() {
                    kargs.push_front(match item {
                        Some(ast) => ast.interpret()?,
                        None => K::Nil,
                    })
                }
                value.apply(kargs)
            }
            ASTNode::ExprList(Spanned(s, _, _)) => Err(Error {
                location: s,
                code: ErrorCode::Nyi,
            }),
        }
    }

    fn apply(self, args: VecDeque<K>) -> Result<K, Error> {
        let k = self.interpret()?;
        match k {
            K::Verb(Verb::Plus) => {
                if let K::Int(x) = args[0] {
                    if let K::Int(y) = args[1] {
                        return Ok(K::Int(x + y));
                    }
                }
                Ok(K::Int(0))
            }
            K::Verb(Verb::Star) => {
                if let K::Int(x) = args[0] {
                    if let K::Int(y) = args[1] {
                        return Ok(K::Int(x * y));
                    }
                }
                Ok(K::Int(0))
            }
            _ => Err(Error {
                location: 0,
                code: ErrorCode::Nyi,
            }),
        }
    }
}
