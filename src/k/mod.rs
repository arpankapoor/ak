use std::fmt;
use std::hint::unreachable_unchecked;
use std::mem;
use std::num::FpCategory;

use crate::error::RuntimeErrorCode;
use crate::sym::Sym;

mod arith;

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

type KResult = Result<K, RuntimeErrorCode>;

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

macro_rules! impl_from {
    ($type: ty, $kvariant: path) => {
        impl From<$type> for K {
            fn from(v: $type) -> K {
                $kvariant(v)
            }
        }
    };
}

impl_from!(u8, K::Char);
impl_from!(i64, K::Int);
impl_from!(f64, K::Float);
impl_from!(Sym, K::Sym);
impl_from!(Vec<u8>, K::CharList);
impl_from!(Vec<i64>, K::IntList);
impl_from!(Vec<f64>, K::FloatList);
impl_from!(Vec<Sym>, K::SymList);

impl From<Vec<K>> for K {
    fn from(v: Vec<K>) -> Self {
        if let Some((first, rest)) = v.split_first() {
            if matches!(first, K::Char(_) | K::Int(_) | K::Float(_) | K::Sym(_))
                && rest
                    .iter()
                    .all(|x| mem::discriminant(first) == mem::discriminant(x))
            {
                macro_rules! to_simple_list {
                    ($list: ident, $variant: path) => {
                        $list
                            .into_iter()
                            .map(|k| match k {
                                $variant(x) => x,
                                _ => unsafe { unreachable_unchecked() },
                            })
                            .collect::<Vec<_>>()
                            .into()
                    };
                }
                return match first {
                    K::Char(_) => to_simple_list!(v, K::Char),
                    K::Int(_) => to_simple_list!(v, K::Int),
                    K::Float(_) => to_simple_list!(v, K::Float),
                    K::Sym(_) => to_simple_list!(v, K::Sym),
                    _ => unsafe { unreachable_unchecked() },
                };
            }
        }
        K::GenList(v)
    }
}
