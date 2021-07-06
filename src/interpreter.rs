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
                    let (arg0, arg1) = (args.pop_front().unwrap(), args.pop_front().unwrap());
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
            K::Verb(Verb::Minus) => match args.len() {
                0 => Ok(K::Verb(Verb::Minus)),
                1 => todo!("neg"),
                2 => {
                    let (arg0, arg1) = (args.pop_front().unwrap(), args.pop_front().unwrap());
                    match arg0 - arg1 {
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
            K::Verb(Verb::Star) => match args.len() {
                0 => Ok(K::Verb(Verb::Star)),
                1 => todo!("first"),
                2 => {
                    let (arg0, arg1) = (args.pop_front().unwrap(), args.pop_front().unwrap());
                    match arg0 * arg1 {
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
            K::Verb(Verb::Percent) => match args.len() {
                0 => Ok(K::Verb(Verb::Percent)),
                1 => todo!("first"),
                2 => {
                    let (arg0, arg1) = (args.pop_front().unwrap(), args.pop_front().unwrap());
                    match arg0 / arg1 {
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
            _ => Err(Error {
                location: start,
                code: RuntimeErrorCode::Nyi,
            }),
        }
    }
}
