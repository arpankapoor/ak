use std::collections::VecDeque;

use crate::error::RuntimeErrorCode;
use crate::k::{Verb, K};
use crate::parser::ASTNode;
use crate::span::Spanned;

#[derive(Debug)]
pub struct Error {
    location: usize,
    code: RuntimeErrorCode,
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
                code: RuntimeErrorCode::Nyi,
            }),
        }
    }

    fn apply(self, mut args: VecDeque<K>) -> Result<K, Error> {
        let (start, end) = (self.start(), self.end());
        let k = self.interpret()?;
        match k {
            K::Verb(Verb::Plus) => match args.len() {
                0 => Ok(K::Verb(Verb::Plus)),
                1 => todo!("flip"),
                2 => {
                    let (arg1, arg0) = (args.remove(1).unwrap(), args.remove(0).unwrap());
                    match arg0 + arg1 {
                        Ok(res) => Ok(res),
                        Err(e) => Err(Error {
                            location: start,
                            code: e,
                        }),
                    }
                }
                _ => Err(Error {
                    location: start,
                    code: RuntimeErrorCode::Rank,
                }),
            },
            K::Verb(Verb::Star) => {
                if let K::Int(x) = args[0] {
                    if let K::Int(y) = args[1] {
                        return Ok(K::Int(x * y));
                    }
                }
                Ok(K::Int(0))
            }
            _ => Err(Error {
                location: start,
                code: RuntimeErrorCode::Nyi,
            }),
        }
    }
}
