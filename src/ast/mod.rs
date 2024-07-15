use crate::lexer::Location;
use crate::ops;
use crate::parser::span::Spanned;
use src_derive::node;
use std::fmt::Display;
use std::ops::Range;

/// ast node representing an identifier.
#[node]
pub struct Ident {
    name: String,
    generics: Vec<Ident>,
}

/// ast node representing a field.
#[node]
pub struct Field {
    vis: Option<Visibility>,
    name: String,
    ty: Ident,
}

/// An enum representing the different types of literals that can be used in an expression.
pub enum Literal {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
}

#[derive(Debug)]
/// An enum representing the visibility of a field or method.
pub enum Visibility {
    Private,
    Public,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "pub"),
            Visibility::Private => write!(f, "priv"),
        }
    }
}

/// An enum representing the different operators that can be used in an expression.
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

pub enum Node {
    Ident(Ident),
    Field(Field),
    Literal(Literal),
    Visibility(Visibility),
    Operator(Operator),
}
