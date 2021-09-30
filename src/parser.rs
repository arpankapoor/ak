use std::fmt;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::error::{ParserError, ParserErrorCode};
use crate::k::{Verb, K, K0};
use crate::span::Spanned;
use crate::tok::Token;

#[derive(Debug)]
pub enum ASTNode {
    Expr(Spanned<K>),
    Apply(Spanned<(Box<ASTNode>, Vec<Option<ASTNode>>)>),
    ExprList(Spanned<Vec<Option<ASTNode>>>),
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_list(f: &mut fmt::Formatter<'_>, list: &[Option<ASTNode>]) -> fmt::Result {
            let mut write_option = |oast, sep| match oast {
                Some(ast) => write!(f, "{}{}", ast, sep),
                None => write!(f, "None{}", sep),
            };
            if let Some((last, rest)) = list.split_last() {
                for ast in rest {
                    write_option(ast.as_ref(), ", ")?;
                }
                write_option(last.as_ref(), "")?;
            }
            Ok(())
        }
        match self {
            Self::Expr(Spanned(_, _, k)) => write!(f, "{:?}", k),
            Self::Apply(Spanned(_, _, (value, args))) => {
                write!(f, "Apply[{}, ", value)?;
                write_list(f, args)?;
                write!(f, "]")
            }
            Self::ExprList(Spanned(_, _, list)) => {
                write!(f, "ExprList[")?;
                write_list(f, list)?;
                write!(f, "]")
            }
        }
    }
}

impl ASTNode {
    pub fn start(&self) -> usize {
        match self {
            Self::Expr(Spanned(s, _, _)) => *s,
            Self::Apply(Spanned(s, _, _)) => *s,
            Self::ExprList(Spanned(s, _, _)) => *s,
        }
    }

    pub fn end(&self) -> usize {
        match self {
            Self::Expr(Spanned(_, e, _)) => *e,
            Self::Apply(Spanned(_, e, _)) => *e,
            Self::ExprList(Spanned(_, e, _)) => *e,
        }
    }
}

pub struct Parser {
    tokens_iter: Peekable<IntoIter<Spanned<Token>>>,
}

macro_rules! extract_ast {
    ($e: expr) => {
        match $e {
            Ok(Some(ast)) => ast,
            x => return x,
        }
    };
}

type PResult = Result<Option<ASTNode>, ParserError>;

impl Parser {
    pub fn new(tokens: Vec<Spanned<Token>>) -> Self {
        Parser {
            tokens_iter: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> PResult {
        self.program()
    }

    fn program(&mut self) -> PResult {
        let Spanned(start, end, mut exprs) = self.expr_list(0)?;
        if let Some(Spanned(s, _, _)) = self.tokens_iter.next() {
            return Err(ParserError {
                location: s,
                code: ParserErrorCode::UnexpectedToken,
            });
        }
        match exprs.len() {
            1 => Ok(exprs.remove(0)),
            _ => Ok(Some(ASTNode::ExprList(Spanned(start, end, exprs)))),
        }
    }

    // infix verb or simple subexpression
    fn expr(&mut self) -> PResult {
        let e1 = extract_ast!(self.subexpr());
        let res = match self.tokens_iter.next_if(|x| matches!(x.2, Token::Verb(_))) {
            Some(Spanned(s, e, Token::Verb(v))) => {
                let verb = ASTNode::Expr(Spanned(s, e, K0::Verb(v).into()));
                match self.expr()? {
                    Some(e2) => ASTNode::Apply(Spanned(
                        e1.start(),
                        e2.end(),
                        (Box::new(verb), vec![Some(e1), Some(e2)]),
                    )),
                    None => ASTNode::Apply(Spanned(
                        e1.start(),
                        verb.end(),
                        (Box::new(verb), vec![Some(e1), None]),
                    )),
                }
            }
            _ => match self.expr()? {
                Some(e2) => ASTNode::Apply(Spanned(
                    e1.start(),
                    e2.end(),
                    (Box::new(e1), vec![Some(e2)]),
                )),
                None => e1,
            },
        };
        Ok(Some(res))
    }

    fn subexpr(&mut self) -> PResult {
        let Spanned(s, e, t) = match self
            .tokens_iter
            .next_if(|x| !matches!(x.2, Token::Semi | Token::RtParen | Token::RtBracket))
        {
            None => return Ok(None),
            Some(s) => s,
        };
        Ok(Some(match t {
            Token::LtParen => extract_ast!(self.paren(s)),
            //Token::LtBraces => extract_ast!(self.function(s)),
            Token::LtBracket => extract_ast!(self.bracket(s)),
            Token::Verb(v) => ASTNode::Expr(Spanned(s, e, K0::Verb(v).into())),
            Token::Adverb(a) => ASTNode::Expr(Spanned(s, e, K0::Adverb(a).into())),
            Token::Char(c) => ASTNode::Expr(Spanned(s, e, K0::Char(c).into())),
            Token::Int(i) => ASTNode::Expr(Spanned(s, e, K0::Int(i).into())),
            Token::Float(f) => ASTNode::Expr(Spanned(s, e, K0::Float(f).into())),
            Token::Sym(sym) => ASTNode::Expr(Spanned(s, e, K0::Sym(sym).into())),
            Token::CharList(c) => ASTNode::Expr(Spanned(s, e, K0::CharList(c).into())),
            Token::IntList(i) => ASTNode::Expr(Spanned(s, e, K0::IntList(i).into())),
            Token::FloatList(f) => ASTNode::Expr(Spanned(s, e, K0::FloatList(f).into())),
            Token::SymList(sym) => ASTNode::Expr(Spanned(s, e, K0::SymList(sym).into())),
            Token::Name(id) => ASTNode::Expr(Spanned(s, e, K0::Name(id).into())),
            _ => ASTNode::Expr(Spanned(0, 0, K0::GenList(vec![]).into())), // replace with error or unreachable..
        }))
    }

    // parenthesized expression or expression list
    fn paren(&mut self, start: usize) -> PResult {
        let Spanned(_, _, mut exprs) = self.expr_list(start)?;
        match self.tokens_iter.next_if(|x| matches!(x.2, Token::RtParen)) {
            Some(Spanned(_, end, _)) => match exprs.len() {
                // single expression within parens
                1 if matches!(exprs.first(), Some(Some(_))) => Ok(exprs.remove(0)),
                // empty parens ()
                1 => Ok(Some(ASTNode::Expr(Spanned(
                    start,
                    end,
                    K0::GenList(Vec::new()).into(),
                )))),
                // list of objects
                _ => Ok(Some(ASTNode::Apply(Spanned(
                    start,
                    end,
                    (
                        Box::new(ASTNode::Expr(Spanned(
                            start,
                            start,
                            K0::Verb(Verb::Comma).into(),
                        ))),
                        exprs,
                    ),
                )))),
            },
            None => Err(ParserError {
                location: start,
                code: ParserErrorCode::UnclosedParens,
            }),
        }
    }

    // bracketed expression list
    fn bracket(&mut self, start: usize) -> PResult {
        let Spanned(_, _, exprs) = self.expr_list(start)?;
        match self
            .tokens_iter
            .next_if(|x| matches!(x.2, Token::RtBracket))
        {
            Some(Spanned(_, end, _)) => Ok(Some(ASTNode::ExprList(Spanned(start, end, exprs)))),
            None => Err(ParserError {
                location: start,
                code: ParserErrorCode::UnclosedBrackets,
            }),
        }
    }

    // semicolon seperated expressions
    fn expr_list(&mut self, start: usize) -> Result<Spanned<Vec<Option<ASTNode>>>, ParserError> {
        let mut list = Vec::new();
        let mut end = start;
        loop {
            match self.expr()? {
                Some(ast) => {
                    end = ast.end();
                    list.push(Some(ast))
                }
                None => list.push(None),
            }
            match self.tokens_iter.next_if(|x| matches!(x.2, Token::Semi)) {
                Some(Spanned(_, e, _)) => end = e,
                None => break,
            }
        }
        Ok(Spanned(
            list.first()
                .map(|x| x.as_ref())
                .flatten()
                .map_or(start, |x| x.start()),
            list.last()
                .map(|x| x.as_ref())
                .flatten()
                .map_or(end, |x| x.end()),
            list,
        ))
    }
}
