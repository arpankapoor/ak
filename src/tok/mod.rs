use std::num::{ParseFloatError, ParseIntError};
use std::str;

use crate::k::{Adverb, Verb};
use crate::span::Spanned;
use crate::sym::Sym;
use crate::tok::stream::ByteStream;

mod stream;

#[derive(Debug)]
pub enum Token {
    LtParen,   // (
    RtParen,   // )
    LtBrace,   // {
    RtBrace,   // }
    LtBracket, // [
    RtBracket, // ]
    Semi,      // ;

    Verb(Verb),
    Adverb(Adverb),

    Char(u8),
    Int(i64),
    Float(f64),
    Sym(Sym),

    CharList(Vec<u8>),
    IntList(Vec<i64>),
    FloatList(Vec<f64>),
    SymList(Vec<Sym>),

    Identifier(Sym),
}

impl From<Vec<u8>> for Token {
    fn from(mut v: Vec<u8>) -> Self {
        if v.len() == 1 {
            Token::Char(v.remove(0))
        } else {
            Token::CharList(v)
        }
    }
}

impl From<Vec<i64>> for Token {
    fn from(mut v: Vec<i64>) -> Self {
        if v.len() == 1 {
            Token::Int(v.remove(0))
        } else {
            Token::IntList(v)
        }
    }
}

impl From<Vec<f64>> for Token {
    fn from(mut v: Vec<f64>) -> Self {
        if v.len() == 1 {
            Token::Float(v.remove(0))
        } else {
            Token::FloatList(v)
        }
    }
}

impl From<Vec<Sym>> for Token {
    fn from(mut v: Vec<Sym>) -> Self {
        if v.len() == 1 {
            Token::Sym(v.remove(0))
        } else {
            Token::SymList(v)
        }
    }
}

#[derive(Debug)]
pub struct Error {
    location: usize,
    code: ErrorCode,
}

#[derive(Debug)]
pub enum ErrorCode {
    UnterminatedString,
    UnterminatedEscape,
    UnterminatedFloatExponent,
    UnrecognizedEscape,
    UnrecognizedToken,
    InvalidNumber,
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
}

impl From<ParseFloatError> for ErrorCode {
    fn from(e: ParseFloatError) -> Self {
        ErrorCode::ParseFloatError(e)
    }
}

impl From<ParseIntError> for ErrorCode {
    fn from(e: ParseIntError) -> Self {
        ErrorCode::ParseIntError(e)
    }
}

pub struct Tokenizer<'a> {
    stream: ByteStream<'a>,
    start: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Self {
            stream: ByteStream::new(src),
            start: 0,
        }
    }

    fn bump(&mut self) {
        self.start = self.stream.next_index();
    }

    fn token(&self, token: Token) -> Option<<Self as Iterator>::Item> {
        Some(Ok(Spanned(self.start, token, self.stream.next_index())))
    }

    fn error(&self, error: ErrorCode) -> Option<<Self as Iterator>::Item> {
        Some(Err(Error {
            location: self.start,
            code: error,
        }))
    }

    fn skip_comment(&mut self) {
        self.stream.consume_while(|x| x != b'\n');
        self.bump();
    }

    // (`[a-zA-Z0-9.:]*)+
    fn symbol(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut syms = Vec::new();
        loop {
            let start = self.stream.next_index();
            self.stream
                .consume_while(|x| x.is_ascii_alphanumeric() || matches!(x, b'.' | b':'));
            syms.push(Sym::new(self.stream.slice(start)));
            if self.stream.next_if_eq(b'`').is_none() {
                break;
            }
        }
        self.token(syms.into())
    }

    // char/string
    fn string(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut string = Vec::new();
        while let Some(c) = self.stream.next_if(|x| x != b'"') {
            string.push(match c {
                b'\\' => match self.stream.next() {
                    Some(e @ (b'"' | b'\\')) => e,
                    Some(b'n') => b'\n',
                    Some(b'r') => b'\r',
                    Some(b't') => b'\t',
                    Some(_) => return self.error(ErrorCode::UnrecognizedEscape),
                    None => return self.error(ErrorCode::UnterminatedEscape),
                },
                _ => c,
            });
        }
        match self.stream.next_if_eq(b'"') {
            Some(_) => self.token(string.into()),
            None => self.error(ErrorCode::UnterminatedString),
        }
    }

    fn identifier(&mut self) -> Option<<Self as Iterator>::Item> {
        self.stream.consume_while(|x| x.is_ascii_alphanumeric());
        self.token(Token::Identifier(Sym::new(self.stream.slice(self.start))))
    }

    fn skip_whitespace(&mut self) {
        self.stream
            .consume_while(|x| matches!(x, b'\t' | b'\x0C' | b'\r' | b' '));
        self.bump();
    }

    fn newline(&mut self) -> Option<<Self as Iterator>::Item> {
        self.stream.consume_while(|x| x == b'\n');
        self.token(Token::Semi)
    }

    fn is_num_start(&self) -> bool {
        match self.stream.peek() {
            Some(b'0'..=b'9') => true,
            Some(b'.') => matches!(self.stream.peek_next(), Some(b'0'..=b'9')),
            _ => false,
        }
    }

    // ([^)}\]0-9a-zA-Z]-)?([0-9]+(\.[0-9]*)?|\.[0-9]+)(e[-+]?[0-9]+)?( -?([0-9]+(\.[0-9]*)?|\.[0-9]+)(e[-+]?[0-9]+)?)*
    fn number(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut is_float = false;
        let mut start = self.start;
        loop {
            if self.stream.curr() == Some(b'.') {
                is_float = true;
            } else {
                self.stream.consume_while(|x| x.is_ascii_digit());
                is_float |= self.stream.next_if_eq(b'.').is_some();
            }
            // digits before decimal point are consumed at this point
            self.stream.consume_while(|x| x.is_ascii_digit());
            if self.stream.next_if_eq(b'e').is_some() {
                is_float = true;
                self.stream.next_if(|x| matches!(x, b'+' | b'-'));
                if self.stream.consume_while(|x| x.is_ascii_digit()) == 0 {
                    self.start = start;
                    return self.error(ErrorCode::UnterminatedFloatExponent);
                }
            }
            let backtrack = self.stream.clone();
            match self.stream.peek() {
                Some(b' ') => {
                    self.stream.next(); // ' '
                    start = self.stream.next_index();
                    self.stream.next_if_eq(b'-');
                    if !self.is_num_start() {
                        self.stream = backtrack;
                        break;
                    }
                }
                Some(b'.' | b'a'..=b'z' | b'A'..=b'Z') => {
                    self.start = start;
                    return self.error(ErrorCode::InvalidNumber);
                }
                _ => break,
            }
        }
        macro_rules! parse_nums {
            ($ty: ty, $lexeme: ident) => {
                $lexeme
                    .split(|&x| x == b' ')
                    .map(|x| unsafe { str::from_utf8_unchecked(x) }.parse())
                    .collect::<Result<Vec<$ty>, _>>()
                    .map_or_else(|e| self.error(e.into()), |v| self.token(v.into()))
            };
        }
        let slice = self.stream.slice(self.start);
        if is_float {
            parse_nums!(f64, slice)
        } else {
            parse_nums!(i64, slice)
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Result<Spanned<Token>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let tok = match self.stream.next()? {
                b'(' => self.token(Token::LtParen),
                b')' => self.token(Token::RtParen),
                b'{' => self.token(Token::LtBrace),
                b'}' => self.token(Token::RtBrace),
                b'[' => self.token(Token::LtBracket),
                b']' => self.token(Token::RtBracket),
                b';' => self.token(Token::Semi),
                b':' => self.token(Token::Verb(Verb::Colon)),
                b'+' => self.token(Token::Verb(Verb::Plus)),
                b'-' => {
                    if (!matches!(
                        self.stream.prev(),
                        Some(b')' | b'}' | b']' | b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z')
                    )) && self.is_num_start()
                    {
                        self.number() // -.[0-9] or -[0-9]
                    } else {
                        self.token(Token::Verb(Verb::Minus))
                    }
                }
                b'*' => self.token(Token::Verb(Verb::Star)),
                b'%' => self.token(Token::Verb(Verb::Percent)),
                b'&' => self.token(Token::Verb(Verb::And)),
                b'|' => self.token(Token::Verb(Verb::Pipe)),
                b'^' => self.token(Token::Verb(Verb::Caret)),
                b'=' => self.token(Token::Verb(Verb::Eq)),
                b'<' => self.token(Token::Verb(Verb::Lt)),
                b'>' => self.token(Token::Verb(Verb::Gt)),
                b'$' => self.token(Token::Verb(Verb::Dollar)),
                b',' => self.token(Token::Verb(Verb::Comma)),
                b'#' => self.token(Token::Verb(Verb::Hash)),
                b'_' => self.token(Token::Verb(Verb::Underscore)),
                b'~' => self.token(Token::Verb(Verb::Tilde)),
                b'!' => self.token(Token::Verb(Verb::Bang)),
                b'?' => self.token(Token::Verb(Verb::Question)),
                b'@' => self.token(Token::Verb(Verb::At)),
                b'.' if self.stream.peek().filter(|x| x.is_ascii_digit()).is_some() => {
                    self.number() // .[0-9]
                }
                b'.' => self.token(Token::Verb(Verb::Dot)),
                b'0' if self.stream.next_if_eq(b':').is_some() => {
                    self.token(Token::Verb(Verb::ZeroColon))
                }
                b'1' if self.stream.next_if_eq(b':').is_some() => {
                    self.token(Token::Verb(Verb::OneColon))
                }
                b'2' if self.stream.next_if_eq(b':').is_some() => {
                    self.token(Token::Verb(Verb::TwoColon))
                }
                b'\'' if self.stream.next_if_eq(b':').is_some() => {
                    self.token(Token::Adverb(Adverb::QuoteColon))
                }
                b'\'' => self.token(Token::Adverb(Adverb::Quote)),
                b'/' if self
                    .stream
                    .prev()
                    .filter(|x| !x.is_ascii_whitespace())
                    .is_none() =>
                {
                    self.skip_comment();
                    continue;
                }
                b'/' if self.stream.next_if_eq(b':').is_some() => {
                    self.token(Token::Adverb(Adverb::SlashColon))
                }
                b'/' => self.token(Token::Adverb(Adverb::Slash)),
                b'\\' if self.stream.next_if_eq(b':').is_some() => {
                    self.token(Token::Adverb(Adverb::BackslashColon))
                }
                b'\\' => self.token(Token::Adverb(Adverb::Backslash)),
                b'`' => self.symbol(),
                b'"' => self.string(),
                b'a'..=b'z' | b'A'..=b'Z' => self.identifier(),
                b'0'..=b'9' => self.number(),
                b'\t' | b'\x0C' | b'\r' | b' ' => {
                    self.skip_whitespace();
                    continue;
                }
                b'\n' => self.newline(),
                _ => self.error(ErrorCode::UnrecognizedToken),
            };
            self.bump();
            break tok;
        }
    }
}
