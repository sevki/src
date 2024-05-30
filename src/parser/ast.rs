use std::fmt::Display;

pub const ANON_FN_NAME: &str = "anonymous";

#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub String, pub Option<Vec<Ident>>);

#[derive(PartialEq, Debug)]
pub struct StringLit(pub String);

#[derive(PartialEq, Debug)]
pub struct Binding(pub Ident, pub Box<Expression>);

#[derive(PartialEq, Debug)]
pub enum Literal {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
}

#[derive(PartialEq, Debug)]
pub enum Keyword {
    None,
    Some,
    Let,
    Action,
    Saga,
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
#[derive(PartialEq, Debug)]
pub enum Value {
    Literal(Literal),
    Ident(Ident),
}

#[derive(PartialEq, Debug)]
pub struct Block<T>(pub Vec<T>);

#[derive(PartialEq, Debug)]
pub struct Tuple<T>(pub Vec<T>);
#[derive(PartialEq, Debug)]
pub struct Array<T>(pub Vec<T>);

#[derive(PartialEq, Debug)]
pub struct BinaryOperation {
    pub lhs: Box<Expression>,
    pub op: Operator,
    pub rhs: Box<Expression>,
}

#[derive(PartialEq, Debug)]
pub struct FnCall(pub Ident, pub Vec<Box<Expression>>);

#[derive(PartialEq, Debug)]
pub enum Expression {
    BinaryExpression(BinaryOperation),
    Bool(bool),
    Integer(i64),
    Float(f64),
    Ident(Ident),
    Binding(Binding),
    FnCall(FnCall),
    String(String),
    FnDef(FnDef),
    ShellCommand(Vec<Ident>, Vec<Box<Expression>>),
    EffectDef(EffectDef),
    StructDef(StructDef),
    UseDef(UseDef),
    Keyword(Keyword),
    ImplDef(ImplDef),
    Branch(Branch),
    Error,
}

#[derive(PartialEq, Debug)]
pub struct Field(pub Ident, pub Ident);

#[derive(PartialEq, Debug)]
pub enum FnArg {
    Reciever,
    Field(Field)
}

#[derive(PartialEq, Debug)]
pub struct Prototype {
    pub name: Ident,
    pub args: Vec<FnArg>,
    pub ret: Option<Ident>,
    pub effects: Vec<Ident>,
}

#[derive(PartialEq, Debug)]
pub struct FnDef(
    pub Prototype,
    pub Block<Box<Expression>>,
    pub Vec<(Ident, Block<Box<Expression>>)>,
);

#[derive(PartialEq, Debug)]
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
        };
        write!(f, "{}", op)
    }
}

#[derive(PartialEq, Debug)]
pub struct StructDef(pub Ident, pub Block<Field>);

#[derive(PartialEq, Debug)]
pub struct FnIdent(pub Ident);

#[derive(PartialEq, Debug)]
pub struct EffectDef(pub Ident, pub Vec<Ident>, pub Block<Prototype>);

#[derive(PartialEq, Debug)]
pub struct UseDef(pub Vec<Ident>, pub Ident);

#[derive(PartialEq, Debug)]
pub struct ImplDef(pub Ident, pub Option<Ident>, pub Block<Box<Expression>>);

#[derive(PartialEq, Debug)]
pub struct Branch(pub Box<Expression>, pub Vec<(Expression, Block<Box<Expression>>)>);

#[derive(PartialEq, Debug)]
pub struct Module(pub Vec<Box<Expression>>);


#[cfg(test)]
use proptest::prelude::*;


#[cfg(test)]
impl Arbitrary for Operator {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: ()) -> Self::Strategy {
        prop_oneof![
            Just(Operator::Add),
            Just(Operator::Sub),
            Just(Operator::Mul),
            Just(Operator::Div),
        ]
        .boxed()
    }
}
