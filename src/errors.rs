use super::parser::MDValue;
use super::tokenizer::Token;
use std::rc::Rc;

/// Representation of all the possible errors originated at 
pub enum ParserError<'a> {
    ExpectedButGot((usize, usize), &'a[&'a str], Token),
    Unexpected((usize, usize), &'a Token),
}