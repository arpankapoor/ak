use crate::k::K;
use crate::span::Spanned;
use crate::tok::Token;
use std::iter::Peekable;
use std::slice::Iter;

pub struct ASTNode(Spanned<K>);

pub struct Error {}

pub struct Parser {
    //tokens_iter: Peekable<Iter<'_, Spanned<Token>>>,
}

impl Parser {
    //pub fn new(tokens: &[Spanned<Token>]) -> Self {
    //    Parser {
    //        tokens_iter: tokens.iter().peekable(),
    //    }
    //}

    //pub fn parse(&mut self) -> Result<ASTNode, Error> {
    //    self.program()
    //}

    //fn program(&mut self) -> Result<ASTNode, Error> {
    //    self.expression()
    //}

    //fn expression(&mut self) -> Result<ASTNode, Error> {}
}
