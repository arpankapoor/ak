use std::fmt;
use std::num::FpCategory;
use std::ops::{Add, Div, Mul, Sub};

use crate::error::RuntimeErrorCode;
use crate::sym::Sym;

#[derive(Debug)]
pub enum Verb {
    Colon = 0,
    Plus = 1,
    Minus = 2,
    Star = 3,
    Percent = 4,
    And = 5,
    Pipe = 6,
    Caret = 7,
    Eq = 8,
    Lt = 9,
    Gt = 10,
    Dollar = 11,
    Comma = 12,
    Hash = 13,
    Underscore = 14,
    Tilde = 15,
    Bang = 16,
    Question = 17,
    At = 18,
    Dot = 19,
    ZeroColon = 20,
    OneColon = 21,
    TwoColon = 22,
}

#[derive(Debug)]
pub enum Adverb {
    Quote = 0,
    Slash = 1,
    Backslash = 2,
    QuoteColon = 3,
    SlashColon = 4,
    BackslashColon = 5,
}

#[derive(Debug)]
pub enum K {
    Nil,
    Char(u8),
    Int(i64),
    Float(f64),
    Sym(Sym),
    Name(Sym),

    Verb(Verb),
    Adverb(Adverb),

    CharList(Vec<u8>),
    IntList(Vec<i64>),
    FloatList(Vec<f64>),
    SymList(Vec<Sym>),
    GenList(Vec<K>),
}

impl fmt::Display for K {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_list<T: fmt::Display>(
            f: &mut fmt::Formatter<'_>,
            list: &[T],
            parens: bool,
            separator: &str,
        ) -> fmt::Result {
            if parens {
                write!(f, "(")?;
            }
            if let Some((last, rest)) = list.split_last() {
                for k in rest {
                    write!(f, "{}{}", k, separator)?;
                }
                write!(f, "{}", last)?;
            }
            if parens {
                write!(f, ")")?;
            }
            Ok(())
        }

        fn fmt_float(f: &mut fmt::Formatter<'_>, x: f64) -> fmt::Result {
            match x.classify() {
                FpCategory::Nan => write!(f, "0n"),
                FpCategory::Infinite => {
                    write!(f, "{}0w", if x.is_sign_negative() { "-" } else { "" })
                }
                _ => write!(f, "{}", x),
            }
        }

        match self {
            Self::Nil => write!(f, "nil"),
            Self::Char(x) => write!(f, "{:?}", *x as char),
            Self::Int(x) => write!(f, "{}", x),
            Self::Float(x) => fmt_float(f, *x),
            Self::Sym(x) => write!(f, "{}", x),
            Self::Name(x) => write!(f, "{}", x),
            Self::Verb(x) => write!(f, "{:?}", x),
            Self::Adverb(x) => write!(f, "{:?}", x),
            Self::CharList(x) => write!(f, "{:?}", String::from_utf8_lossy(x)),
            Self::IntList(x) => fmt_list(f, x, false, " "),
            Self::FloatList(x) => {
                if let Some((last, rest)) = x.split_last() {
                    for k in rest {
                        fmt_float(f, *k)?;
                        write!(f, " ")?;
                    }
                    fmt_float(f, *last)?;
                }
                Ok(())
            }
            Self::SymList(x) => fmt_list(f, x, false, ""),
            Self::GenList(x) => fmt_list(f, x, true, ";"),
        }
    }
}

macro_rules! arithmetic_operation {
    ($self: expr, $rhs: expr, $op: tt) => {
        match ($self, $rhs) {
            (Self::Int(x), Self::Int(y)) => Ok(K::Int(x $op y)),
            (Self::Int(x), Self::Float(y)) => Ok(K::Float(x as f64 $op y)),
            (Self::Int(x), Self::IntList(y)) => Ok(K::IntList(y.iter().map(|e| x $op e).collect())),
            (Self::Int(x), Self::FloatList(y)) => {
                let x = x as f64;
                Ok(K::FloatList(y.iter().map(|e| x $op e).collect()))
            }

            (Self::Float(x), Self::Int(y)) => Ok(K::Float(x $op y as f64)),
            (Self::Float(x), Self::Float(y)) => Ok(K::Float(x $op y)),
            (Self::Float(x), Self::IntList(y)) => {
                Ok(K::FloatList(y.iter().map(|&e| x $op e as f64).collect()))
            }
            (Self::Float(x), Self::FloatList(y)) => {
                Ok(K::FloatList(y.iter().map(|e| x $op e).collect()))
            }

            (Self::IntList(x), Self::Int(y)) => Ok(K::IntList(x.iter().map(|e| e $op y).collect())),
            (Self::IntList(x), Self::Float(y)) => {
                Ok(K::FloatList(x.iter().map(|&e| e as f64 $op y).collect()))
            }
            (Self::IntList(x), Self::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::IntList(x.iter().zip(y).map(|(x, y)| x $op y).collect()))
                }
            }
            (Self::IntList(x), Self::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(&x, y)| x as f64 $op y).collect(),
                    ))
                }
            }

            (Self::FloatList(x), Self::Int(y)) => {
                let y = y as f64;
                Ok(K::FloatList(x.iter().map(|e| e $op y).collect()))
            }
            (Self::FloatList(x), Self::Float(y)) => {
                Ok(K::FloatList(x.iter().map(|e| e $op y).collect()))
            }
            (Self::FloatList(x), Self::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(x, y)| x $op y as f64).collect(),
                    ))
                }
            }
            (Self::FloatList(x), Self::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(x.iter().zip(y).map(|(&x, y)| x $op y).collect()))
                }
            }

            (_, _) => Err(RuntimeErrorCode::Type),
        }
    }
}

type KResult = Result<K, RuntimeErrorCode>;

impl Add for K {
    type Output = KResult;

    fn add(self, rhs: Self) -> Self::Output {
        arithmetic_operation!(self, rhs, +)
    }
}

impl Sub for K {
    type Output = KResult;

    fn sub(self, rhs: Self) -> Self::Output {
        arithmetic_operation!(self, rhs, -)
    }
}

impl Mul for K {
    type Output = KResult;

    fn mul(self, rhs: Self) -> Self::Output {
        arithmetic_operation!(self, rhs, *)
    }
}

impl Div for K {
    type Output = KResult;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(x), Self::Int(y)) => Ok(K::Float(x as f64 / y as f64)),
            (Self::Int(x), Self::Float(y)) => Ok(K::Float(x as f64 / y)),
            (Self::Int(x), Self::IntList(y)) => {
                let x = x as f64;
                Ok(K::FloatList(y.iter().map(|&e| x / e as f64).collect()))
            }
            (Self::Int(x), Self::FloatList(y)) => {
                let x = x as f64;
                Ok(K::FloatList(y.iter().map(|e| x / e).collect()))
            }

            (Self::Float(x), Self::Int(y)) => Ok(K::Float(x / y as f64)),
            (Self::Float(x), Self::Float(y)) => Ok(K::Float(x / y)),
            (Self::Float(x), Self::IntList(y)) => {
                Ok(K::FloatList(y.iter().map(|&e| x / e as f64).collect()))
            }
            (Self::Float(x), Self::FloatList(y)) => {
                Ok(K::FloatList(y.iter().map(|e| x / e).collect()))
            }

            (Self::IntList(x), Self::Int(y)) => {
                let y = y as f64;
                Ok(K::FloatList(x.iter().map(|&e| e as f64 / y).collect()))
            }
            (Self::IntList(x), Self::Float(y)) => {
                Ok(K::FloatList(x.iter().map(|&e| e as f64 / y).collect()))
            }
            (Self::IntList(x), Self::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(&x, y)| x as f64 / y as f64).collect(),
                    ))
                }
            }
            (Self::IntList(x), Self::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(&x, y)| x as f64 / y).collect(),
                    ))
                }
            }

            (Self::FloatList(x), Self::Int(y)) => {
                let y = y as f64;
                Ok(K::FloatList(x.iter().map(|e| e / y).collect()))
            }
            (Self::FloatList(x), Self::Float(y)) => {
                Ok(K::FloatList(x.iter().map(|e| e / y).collect()))
            }
            (Self::FloatList(x), Self::IntList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(
                        x.iter().zip(y).map(|(x, y)| x / y as f64).collect(),
                    ))
                }
            }
            (Self::FloatList(x), Self::FloatList(y)) => {
                if x.len() != y.len() {
                    Err(RuntimeErrorCode::Length)
                } else {
                    Ok(K::FloatList(x.iter().zip(y).map(|(&x, y)| x / y).collect()))
                }
            }

            (_, _) => Err(RuntimeErrorCode::Type),
        }
    }
}
