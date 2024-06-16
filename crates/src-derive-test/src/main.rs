fn main() {
    println!("Hello, world!");
}
use src_derive::visitor;
use srclang::{lexer::Location, Db};
use okstd::prelude::*;
use std::{fmt::Display, ops::Range};

pub const ANON_FN_NAME: &str = "anonymous";
use srclang::parser::span::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub String, pub Option<Vec<Spanned<Ident>>>);

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(PartialEq, Debug, Clone, Default)]
pub enum Visibility {
    #[default]
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

#[visitor]
#[derive(PartialEq, Debug, Clone)]
pub struct StringLit(pub String);


#[derive(PartialEq, Debug, Clone)]
pub struct Binding(pub Spanned<Ident>, pub Box<Spanned<Node>>);

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    None,
    Some,
    Let,
    Public,
    Private,
    Fn,
    If,
    Else,
    Match,
    Arrow,
    Struct,
    SelfValue,
    When,
    Effect,
    Impl,
    Use,
    From,
    Where,
    Self_,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = match self {
            Keyword::None => "none",
            Keyword::Some => "some",
            Keyword::Let => "let",
            Keyword::Fn => "fn",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::Match => "match",
            Keyword::Arrow => "=>",
            Keyword::Struct => "struct",
            Keyword::SelfValue => "self",
            Keyword::When => "when",
            Keyword::Effect => "effect",
            Keyword::Impl => "impl",
            Keyword::Use => "use",
            Keyword::From => "from",
            Keyword::Where => "where",
            Keyword::Self_ => "Self",
            Keyword::Public => "pub",
            Keyword::Private => "priv",
        };
        write!(f, "{}", kw)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Ident(Ident),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Block<T>(pub Vec<T>);

#[derive(PartialEq, Debug, Clone)]
pub struct Tuple<T>(pub Vec<T>);

#[derive(PartialEq, Debug, Clone)]
pub struct Array<T>(pub Vec<T>);

#[derive(PartialEq, Debug, Clone)]
pub struct BinaryOperation {
    pub lhs: Box<Spanned<Node>>,
    pub op: Operator,
    pub rhs: Box<Spanned<Node>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FnCall(pub Spanned<Ident>, pub Vec<Spanned<Node>>);

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    BinaryExpression(BinaryOperation),
    Bool(bool),
    Integer(i64),
    Float(f64),
    Ident(Spanned<Ident>),
    Binding(Binding),
    FnCall(FnCall),
    String(String),
    FnDef(FnDef),
    EffectDef(EffectDef),
    StructDef(StructDef),
    UseDef(UseDef),
    Keyword(Keyword),
    ImplDef(ImplDef),
    Branch(BranchDef),
    FieldAccess(FieldAccess),
    Visibility(Visibility),
    Error,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::BinaryExpression(bin) => write!(f, "{} {} {}", bin.lhs, bin.op, bin.rhs),
            Node::Bool(b) => write!(f, "{}", b),
            Node::Integer(i) => write!(f, "{}", i),
            Node::Float(fl) => write!(f, "{}", fl),
            Node::Ident(ident) => write!(f, "{}", ident.1),
            Node::Binding(bind) => write!(f, "{} = {}", bind.0, bind.1),
            Node::FnCall(call) => write!(
                f,
                "{}({})",
                call.0,
                call.1
                    .iter()
                    .map(|e| e.1.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Node::String(s) => write!(f, "{}", s),
            Node::FnDef(def) => write!(f, "{}", def.0),
            Node::EffectDef(def) => write!(f, "{}", def.0),
            Node::StructDef(def) => write!(f, "{}", def.0),
            Node::UseDef(def) => write!(f, "{:#?}", def.0),
            Node::Keyword(kw) => write!(f, "{}", kw),
            Node::ImplDef(def) => write!(f, "{}", def.0),
            Node::Branch(branch) => write!(f, "{}", branch.0),
            Node::FieldAccess(access) => write!(f, "{}.{}", access.0, access.1),
            Node::Visibility(vis) => write!(f, "{}", vis),
            Node::Error => write!(f, "Error"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum FnArg {
    Reciever,
    Field(Spanned<FieldDef>),
}

impl Display for FnArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FnArg::Reciever => write!(f, "self"),
            FnArg::Field(field) => write!(f, "{}", field.1),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Prototype {
    pub name: Spanned<Ident>,
    pub args: Vec<Spanned<FnArg>>,
    pub ret: Option<Spanned<Ident>>,
    pub effects: Vec<Spanned<Ident>>,
}

impl Display for Prototype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.name)?;
        for arg in self.args.iter() {
            write!(f, "{}", arg)?;
        }
        write!(f, ")")?;
        if let Some(ret) = &self.ret {
            write!(f, " -> {}", ret)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Whitespace {
    Space,
    Tab,
    Newline,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Increment,
    Decrement,
    Maybe,
    Not,
    Neg,
    Dot,
    Arrow,
    FatArrow,
    DoubleColon,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Modulo => "%",
            Operator::Increment => "++",
            Operator::Decrement => "--",
            Operator::Maybe => "?",
            Operator::Not => "!",
            Operator::Neg => "-",
            Operator::Dot => ".",
            Operator::Arrow => "->",
            Operator::FatArrow => "=>",
            Operator::DoubleColon => "::",
        };
        write!(f, "{}", op)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FieldAccess(pub Box<Spanned<Node>>, pub Box<Spanned<Node>>);

#[derive(PartialEq, Debug, Clone)]
pub struct Module(pub Vec<Spanned<Node>>);

// defs

#[derive(PartialEq, Debug, Clone)]
pub struct FieldDef(
    pub Spanned<Visibility>,
    pub Spanned<Ident>,
    pub Spanned<Ident>,
);

impl Display for FieldDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct StructDef(
    pub KeywordAndVisibility,
    pub Spanned<Ident>,
    pub Block<Spanned<FieldDef>>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct FnIdent(pub Ident);

#[derive(PartialEq, Debug, Clone)]
pub struct EffectDef(
    pub KeywordAndVisibility,
    pub Spanned<Ident>,
    pub Vec<Spanned<Ident>>,
    pub Block<Spanned<Prototype>>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct UseDef(
    pub KeywordAndVisibility,
    pub Vec<Spanned<Ident>>,
    pub Spanned<Ident>,
);

impl Display for UseDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} from {}", self.0, self.1, self.2)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct KeywordAndVisibility(pub Spanned<Keyword>, pub Spanned<Visibility>);

impl Display for KeywordAndVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.1, self.0)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ImplDef(
    pub KeywordAndVisibility,
    pub Spanned<Ident>,
    pub Option<Spanned<Ident>>,
    pub Block<Spanned<Node>>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct BranchDef(
    pub Box<Spanned<Node>>,
    pub Vec<(Spanned<Node>, Block<Spanned<Node>>)>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct FnDef(
    pub KeywordAndVisibility,
    pub Spanned<Prototype>,
    pub Block<Spanned<Node>>,
    pub Vec<(Spanned<Ident>, Block<Spanned<Node>>)>,
);

impl Display for FnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {{", self.0, self.1)?;
        for expr in self.2 .0.iter() {
            write!(f, "{}", expr)?;
        }
        write!(f, "}}")
    }
}
