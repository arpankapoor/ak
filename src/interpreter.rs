use std::collections::VecDeque;
use std::ops::Deref;

use crate::environ::{define_variable, get_variable};
use crate::error::{RuntimeError, RuntimeErrorCode};
use crate::k::{Verb, K, K0};
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
                        None => K0::Nil.into(),
                    })
                }
                value.apply(kargs)
            }
            ASTNode::ExprList(Spanned(s, _, _)) => Err(RuntimeError::new(s, RuntimeErrorCode::Nyi)),
        }
    }

    fn apply(self, mut args: VecDeque<K>) -> Result<K, RuntimeError> {
        let start = self.start();
        let k = self.interpret()?;
        match k.deref() {
            K0::Verb(Verb::Plus) => match args.len() {
                0 => Ok(k),
                1 => todo!("flip"),
                2 => (&args[0] + &args[1]).map_err(|e| RuntimeError::new(start, e)),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K0::Verb(Verb::Minus) => match args.len() {
                0 => Ok(k),
                1 => (-&args[0]).map_err(|e| RuntimeError::new(start, e)),
                2 => (&args[0] - &args[1]).map_err(|e| RuntimeError::new(start, e)),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K0::Verb(Verb::Star) => match args.len() {
                0 => Ok(k),
                1 => todo!("first"),
                2 => (&args[0] * &args[1]).map_err(|e| RuntimeError::new(start, e)),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K0::Verb(Verb::Percent) => match args.len() {
                0 => Ok(k),
                1 => todo!("first"),
                2 => (&args[0] / &args[1]).map_err(|e| RuntimeError::new(start, e)),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K0::Verb(Verb::Comma) => match args.len() {
                0 => Ok(k),
                _ => Ok(Vec::from(args).into()), // todo: specialize cases
            },
            K0::Verb(Verb::Colon) => match args.len() {
                0 => Ok(k),
                2 => match args[0].deref() {
                    K0::Name(lhs) => match args[1].deref() {
                        K0::Name(rhs) => match get_variable(*rhs) {
                            Some(value) => {
                                define_variable(*lhs, &value);
                                Ok(value)
                            }
                            None => Err(RuntimeError::new(
                                start,
                                RuntimeErrorCode::UndefinedVariable,
                            )),
                        },
                        _ => {
                            define_variable(*lhs, &args[1]);
                            Ok(args.pop_back().unwrap())
                        }
                    },
                    _ => Err(RuntimeError::new(start, RuntimeErrorCode::NameExpected)),
                },
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K0::Verb(Verb::Bang) => match args.len() {
                0 => Ok(k),
                1 => match args[0].deref() {
                    K0::Int(x) => Ok(K0::IntList((0..*x).collect()).into()),
                    _ => Err(RuntimeError::new(start, RuntimeErrorCode::Type)),
                },
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Nyi)),
            },
            K0::Verb(Verb::At) => match args.len() {
                0 => Ok(k),
                1 => Ok(K0::Sym(Sym::new(match args[0].deref() {
                    K0::Nil => b"nil",
                    K0::Char(_) => b"c",
                    K0::Int(_) => b"i",
                    K0::Float(_) => b"f",
                    K0::Sym(_) => b"n",
                    K0::Name(_) => b"n", // todo: lookup variable

                    K0::Verb(_) => b"v",
                    K0::Adverb(_) => b"a",

                    K0::CharList(_) => b"C",
                    K0::IntList(_) => b"I",
                    K0::FloatList(_) => b"F",
                    K0::SymList(_) => b"N",
                    K0::GenList(_) => b"l",
                }))
                .into()),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Nyi)),
            },
            _ => Err(RuntimeError::new(start, RuntimeErrorCode::Nyi)),
        }
    }
}
