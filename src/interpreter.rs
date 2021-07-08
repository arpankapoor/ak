use std::collections::VecDeque;

use crate::error::{RuntimeError, RuntimeErrorCode};
use crate::k::{Verb, K};
use crate::parser::ASTNode;
use crate::span::Spanned;
use crate::sym::Sym;

impl ASTNode {
    pub fn interpret(self) -> Result<K, RuntimeError> {
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
            ASTNode::ExprList(Spanned(s, _, _)) => Err(RuntimeError {
                location: s,
                code: RuntimeErrorCode::Nyi,
            }),
        }
    }

    fn apply(self, mut args: VecDeque<K>) -> Result<K, RuntimeError> {
        let (start, _) = (self.start(), self.end());
        let k = self.interpret()?;
        match k {
            v @ K::Verb(Verb::Plus) => match args.len() {
                0 => Ok(v),
                1 => todo!("flip"),
                2 => match &args[0] + &args[1] {
                    Ok(res) => Ok(res),
                    Err(e) => Err(RuntimeError {
                        location: start,
                        code: e,
                    }),
                },
                _ => Err(RuntimeError {
                    location: start,
                    code: RuntimeErrorCode::Rank,
                }),
            },
            v @ K::Verb(Verb::Minus) => match args.len() {
                0 => Ok(v),
                1 => todo!("neg"),
                2 => match &args[0] - &args[1] {
                    Ok(res) => Ok(res),
                    Err(e) => Err(RuntimeError {
                        location: start,
                        code: e,
                    }),
                },
                _ => Err(RuntimeError {
                    location: start,
                    code: RuntimeErrorCode::Rank,
                }),
            },
            v @ K::Verb(Verb::Star) => match args.len() {
                0 => Ok(v),
                1 => todo!("first"),
                2 => match &args[0] * &args[1] {
                    Ok(res) => Ok(res),
                    Err(e) => Err(RuntimeError {
                        location: start,
                        code: e,
                    }),
                },
                _ => Err(RuntimeError {
                    location: start,
                    code: RuntimeErrorCode::Rank,
                }),
            },
            v @ K::Verb(Verb::Percent) => match args.len() {
                0 => Ok(v),
                1 => todo!("first"),
                2 => match &args[0] / &args[1] {
                    Ok(res) => Ok(res),
                    Err(e) => Err(RuntimeError {
                        location: start,
                        code: e,
                    }),
                },
                _ => Err(RuntimeError {
                    location: start,
                    code: RuntimeErrorCode::Rank,
                }),
            },
            v @ K::Verb(Verb::Comma) => match args.len() {
                0 => Ok(v),
                _ => Ok(Vec::from(args).into()), // todo: specialize cases
            },
            v @ K::Verb(Verb::At) => match args.len() {
                0 => Ok(v),
                1 => {
                    Ok(K::Sym(Sym::new(match args.pop_front().unwrap() {
                        K::Nil => b"nil",
                        K::Char(_) => b"c",
                        K::Int(_) => b"i",
                        K::Float(_) => b"f",
                        K::Sym(_) => b"n",
                        K::Name(_) => b"n", // todo: lookup variable

                        K::Verb(_) => b"v",
                        K::Adverb(_) => b"a",

                        K::CharList(_) => b"C",
                        K::IntList(_) => b"I",
                        K::FloatList(_) => b"F",
                        K::SymList(_) => b"N",
                        K::GenList(_) => b"l",
                    })))
                }
                _ => Err(RuntimeError {
                    location: start,
                    code: RuntimeErrorCode::Nyi,
                }),
            },
            _ => Err(RuntimeError {
                location: start,
                code: RuntimeErrorCode::Nyi,
            }),
        }
    }
}
