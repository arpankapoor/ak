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
            ASTNode::Expr(Spanned(s, _, k)) => match k.deref() {
                K0::Name(name) => match get_variable(*name) {
                    Some(value) => Ok(value),
                    None => Err(RuntimeError::new(s, RuntimeErrorCode::UndefinedVariable)),
                },
                _ => Ok(k),
            },
            ASTNode::Apply(Spanned(s, _, (value, args))) => {
                if let ASTNode::Expr(Spanned(_, _, ref k)) = value.deref() {
                    match (k.deref(), args.len(), args.first()) {
                        (
                            K0::Verb(Verb::Dollar),
                            1,
                            Some(Some(ASTNode::ExprList(Spanned(_, _, elist)))),
                        ) if elist.len() > 2 => {
                            // don't interpret args if the verb is $ (conditional) and args is an exprlist with >2 elements
                            return Self::conditional(args);
                        }
                        (
                            K0::Verb(Verb::Colon),
                            2,
                            Some(Some(ASTNode::Expr(Spanned(_, _, name)))),
                        ) if matches!(name.deref(), K0::Name(_)) => {
                            // do not evaluate LHS in an assignment
                            return match args.last() {
                                Some(Some(rhs)) => {
                                    // todo: get rid of the clone somehow
                                    value.apply(&[name.clone(), rhs.clone().interpret()?])
                                }
                                _ => {
                                    Err(RuntimeError::new(s, RuntimeErrorCode::ExpressionExpected))
                                }
                            };
                        }
                        _ => (),
                    }
                }
                let mut kargs = VecDeque::with_capacity(args.len());
                for item in args.into_iter().rev() {
                    kargs.push_front(match item {
                        Some(ast) => ast.interpret()?,
                        None => K0::Nil.into(),
                    })
                }
                value.apply(kargs.make_contiguous())
            }
            ASTNode::ExprList(Spanned(_, _, mut elist)) => {
                let last = elist.pop();
                for ast in elist.into_iter().flatten() {
                    ast.interpret()?;
                }
                match last {
                    Some(Some(ast)) => ast.interpret(),
                    _ => Ok(K0::Nil.into()),
                }
            }
        }
    }

    fn conditional(_args: Vec<Option<ASTNode>>) -> Result<K, RuntimeError> {
        todo!("conditional expression")
    }

    fn apply(self, args: &[K]) -> Result<K, RuntimeError> {
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
                    K0::Name(lhs) => {
                        define_variable(*lhs, &args[1]);
                        Ok(args[1].clone())
                    }
                    _ => Err(RuntimeError::new(
                        start,
                        RuntimeErrorCode::NameExpectedOnLhs,
                    )),
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
