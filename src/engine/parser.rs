//! parses a regular expression into abstract syntax tree
use std::{
    error::Error,
    fmt::{self, Display},
    mem::take
};

// Type of AST
#[derive(Debug)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, BOX<AST>),
    Seq(Vec<AST>),
}

// Type of parse errors
#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char),  // invalid escape sequence
    InvalidRightParen(usize),    // missing left parenthesis
    NoPrev(usize),               // missing regex before +, |, * and ?
    NoRightParen(usize),         // missing right parenthesis
    Empty,                       // empty pattern
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(f, "ParseError: invalid escape: pos = {pos}, char = '{c}'")
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right parenthesis: pos = {pos}")
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: no previous expression: pos = {pos}")
            }
            ParseError::NoRightParen(pos) => {
                write!(f, "ParseError: no right parenthesis: pos = {pos}")
            }
            ParseError::Empty => write!(f, "ParseError: empty expression")
        }
    }
}

impl Error for ParseError {}