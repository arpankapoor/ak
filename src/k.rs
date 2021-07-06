use std::fmt;

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
        fn print_list<T: fmt::Display>(
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

        match self {
            Self::Nil => write!(f, "nil"),
            Self::Char(x) => write!(f, "{:?}", *x as char),
            Self::Int(x) => write!(f, "{}", x),
            Self::Float(x) => write!(f, "{}", x),
            Self::Sym(x) => write!(f, "{}", x),
            Self::Name(x) => write!(f, "{}", x),
            Self::Verb(x) => write!(f, "{:?}", x),
            Self::Adverb(x) => write!(f, "{:?}", x),
            Self::CharList(x) => write!(f, "{:?}", String::from_utf8_lossy(x)),
            Self::IntList(x) => print_list(f, x, false, " "),
            Self::FloatList(x) => print_list(f, x, false, " "),
            Self::SymList(x) => print_list(f, x, false, ""),
            Self::GenList(x) => print_list(f, x, true, ";"),
        }
    }
}
