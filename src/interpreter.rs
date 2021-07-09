use std::collections::VecDeque;

use crate::environ::{define_variable, get_variable};
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
            ASTNode::ExprList(Spanned(s, _, _)) => Err(RuntimeError::new(s, RuntimeErrorCode::Nyi)),
        }
    }

    fn apply(self, args: VecDeque<K>) -> Result<K, RuntimeError> {
        let start = self.start();
        let k = self.interpret()?;
        match k {
            K::Verb(Verb::Plus) => match args.len() {
                0 => Ok(k),
                1 => todo!("flip"),
                2 => (&args[0] + &args[1]).map_err(|e| RuntimeError::new(start, e)),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K::Verb(Verb::Minus) => match args.len() {
                0 => Ok(k),
                1 => (-&args[0]).map_err(|e| RuntimeError::new(start, e)),
                2 => (&args[0] - &args[1]).map_err(|e| RuntimeError::new(start, e)),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K::Verb(Verb::Star) => match args.len() {
                0 => Ok(k),
                1 => todo!("first"),
                2 => (&args[0] * &args[1]).map_err(|e| RuntimeError::new(start, e)),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K::Verb(Verb::Percent) => match args.len() {
                0 => Ok(k),
                1 => todo!("first"),
                2 => (&args[0] / &args[1]).map_err(|e| RuntimeError::new(start, e)),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K::Verb(Verb::Comma) => match args.len() {
                0 => Ok(k),
                _ => Ok(Vec::from(args).into()), // todo: specialize cases
            },
            K::Verb(Verb::Colon) => match args.len() {
                0 => Ok(k),
                2 => match &args[0] {
                    K::Name(lhs) => match args[1].clone() {
                        K::Name(rhs) => match get_variable(rhs) {
                            Some(value) => {
                                define_variable(*lhs, value.clone());
                                Ok(value)
                            }
                            None => Err(RuntimeError::new(
                                start,
                                RuntimeErrorCode::UndefinedVariable,
                            )),
                        },
                        v=> {
                            define_variable(*lhs, v.clone());
                            Ok(v)
                        }
                        //None => Err(RuntimeError::new(
                        //    start,
                        //    RuntimeErrorCode::ExpressionExpected,
                        //)),
                    },
                    _ => Err(RuntimeError::new(start, RuntimeErrorCode::NameExpected)),
                },
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Rank)),
            },
            K::Verb(Verb::Bang) => match args.len() {
                0 => Ok(k),
                1 => match &args[0] {
                    K::Int(x) => Ok(K::IntList((0..*x).collect())),
                    _ => Err(RuntimeError::new(start, RuntimeErrorCode::Type)),
                },
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Nyi)),
            },
            K::Verb(Verb::At) => match args.len() {
                0 => Ok(k),
                1 => Ok(K::Sym(Sym::new(match &args[0] {
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
                }))),
                _ => Err(RuntimeError::new(start, RuntimeErrorCode::Nyi)),
            },
            _ => Err(RuntimeError::new(start, RuntimeErrorCode::Nyi)),
        }
    }
}
