#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}
use src_derive::visitor;
use srclang::{lexer::Location, Db};
use okstd::prelude::*;
use std::{fmt::Display, ops::Range};
pub const ANON_FN_NAME: &str = "anonymous";
use srclang::parser::span::*;
pub struct Ident(pub String, pub Option<Vec<Spanned<Ident>>>);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Ident {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Ident {
    #[inline]
    fn eq(&self, other: &Ident) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Ident {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Ident", &self.0, &&self.1)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Ident {
    #[inline]
    fn clone(&self) -> Ident {
        Ident(::core::clone::Clone::clone(&self.0), ::core::clone::Clone::clone(&self.1))
    }
}
impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}", self.0))
    }
}
pub enum Visibility {
    #[default]
    Private,
    Public,
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Visibility {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Visibility {
    #[inline]
    fn eq(&self, other: &Visibility) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Visibility {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Visibility::Private => "Private",
                Visibility::Public => "Public",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Visibility {
    #[inline]
    fn clone(&self) -> Visibility {
        match self {
            Visibility::Private => Visibility::Private,
            Visibility::Public => Visibility::Public,
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for Visibility {
    #[inline]
    fn default() -> Visibility {
        Self::Private
    }
}
impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => f.write_fmt(format_args!("pub")),
            Visibility::Private => f.write_fmt(format_args!("priv")),
        }
    }
}
pub struct StringLit(pub String);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for StringLit {}
#[automatically_derived]
impl ::core::cmp::PartialEq for StringLit {
    #[inline]
    fn eq(&self, other: &StringLit) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for StringLit {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "StringLit", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for StringLit {
    #[inline]
    fn clone(&self) -> StringLit {
        StringLit(::core::clone::Clone::clone(&self.0))
    }
}
pub trait BindingVisitor {}
pub struct Binding(pub Spanned<Ident>, pub Box<Spanned<Node>>);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Binding {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Binding {
    #[inline]
    fn eq(&self, other: &Binding) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Binding {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(
            f,
            "Binding",
            &self.0,
            &&self.1,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Binding {
    #[inline]
    fn clone(&self) -> Binding {
        Binding(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
pub enum Literal {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Literal {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Literal {
    #[inline]
    fn eq(&self, other: &Literal) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Literal::Bool(__self_0), Literal::Bool(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Literal::Float(__self_0), Literal::Float(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Literal::Integer(__self_0), Literal::Integer(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Literal::String(__self_0), Literal::String(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Literal {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Literal::Bool(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Bool", &__self_0)
            }
            Literal::Float(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Float", &__self_0)
            }
            Literal::Integer(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Integer",
                    &__self_0,
                )
            }
            Literal::String(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "String", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Literal {
    #[inline]
    fn clone(&self) -> Literal {
        match self {
            Literal::Bool(__self_0) => {
                Literal::Bool(::core::clone::Clone::clone(__self_0))
            }
            Literal::Float(__self_0) => {
                Literal::Float(::core::clone::Clone::clone(__self_0))
            }
            Literal::Integer(__self_0) => {
                Literal::Integer(::core::clone::Clone::clone(__self_0))
            }
            Literal::String(__self_0) => {
                Literal::String(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
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
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Keyword {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Keyword {
    #[inline]
    fn eq(&self, other: &Keyword) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Keyword {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Keyword::None => "None",
                Keyword::Some => "Some",
                Keyword::Let => "Let",
                Keyword::Public => "Public",
                Keyword::Private => "Private",
                Keyword::Fn => "Fn",
                Keyword::If => "If",
                Keyword::Else => "Else",
                Keyword::Match => "Match",
                Keyword::Arrow => "Arrow",
                Keyword::Struct => "Struct",
                Keyword::SelfValue => "SelfValue",
                Keyword::When => "When",
                Keyword::Effect => "Effect",
                Keyword::Impl => "Impl",
                Keyword::Use => "Use",
                Keyword::From => "From",
                Keyword::Where => "Where",
                Keyword::Self_ => "Self_",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Keyword {
    #[inline]
    fn clone(&self) -> Keyword {
        match self {
            Keyword::None => Keyword::None,
            Keyword::Some => Keyword::Some,
            Keyword::Let => Keyword::Let,
            Keyword::Public => Keyword::Public,
            Keyword::Private => Keyword::Private,
            Keyword::Fn => Keyword::Fn,
            Keyword::If => Keyword::If,
            Keyword::Else => Keyword::Else,
            Keyword::Match => Keyword::Match,
            Keyword::Arrow => Keyword::Arrow,
            Keyword::Struct => Keyword::Struct,
            Keyword::SelfValue => Keyword::SelfValue,
            Keyword::When => Keyword::When,
            Keyword::Effect => Keyword::Effect,
            Keyword::Impl => Keyword::Impl,
            Keyword::Use => Keyword::Use,
            Keyword::From => Keyword::From,
            Keyword::Where => Keyword::Where,
            Keyword::Self_ => Keyword::Self_,
        }
    }
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
        f.write_fmt(format_args!("{0}", kw))
    }
}
pub enum Value {
    Literal(Literal),
    Ident(Ident),
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Value {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Value {
    #[inline]
    fn eq(&self, other: &Value) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Value::Literal(__self_0), Value::Literal(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Value::Ident(__self_0), Value::Ident(__arg1_0)) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Value {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Value::Literal(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Literal",
                    &__self_0,
                )
            }
            Value::Ident(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ident", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Value {
    #[inline]
    fn clone(&self) -> Value {
        match self {
            Value::Literal(__self_0) => {
                Value::Literal(::core::clone::Clone::clone(__self_0))
            }
            Value::Ident(__self_0) => Value::Ident(::core::clone::Clone::clone(__self_0)),
        }
    }
}
pub struct Block<T>(pub Vec<T>);
#[automatically_derived]
impl<T> ::core::marker::StructuralPartialEq for Block<T> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Block<T> {
    #[inline]
    fn eq(&self, other: &Block<T>) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Block<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Block", &&self.0)
    }
}
#[automatically_derived]
impl<T: ::core::clone::Clone> ::core::clone::Clone for Block<T> {
    #[inline]
    fn clone(&self) -> Block<T> {
        Block(::core::clone::Clone::clone(&self.0))
    }
}
pub struct Tuple<T>(pub Vec<T>);
#[automatically_derived]
impl<T> ::core::marker::StructuralPartialEq for Tuple<T> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Tuple<T> {
    #[inline]
    fn eq(&self, other: &Tuple<T>) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Tuple<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Tuple", &&self.0)
    }
}
#[automatically_derived]
impl<T: ::core::clone::Clone> ::core::clone::Clone for Tuple<T> {
    #[inline]
    fn clone(&self) -> Tuple<T> {
        Tuple(::core::clone::Clone::clone(&self.0))
    }
}
pub struct Array<T>(pub Vec<T>);
#[automatically_derived]
impl<T> ::core::marker::StructuralPartialEq for Array<T> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Array<T> {
    #[inline]
    fn eq(&self, other: &Array<T>) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Array<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Array", &&self.0)
    }
}
#[automatically_derived]
impl<T: ::core::clone::Clone> ::core::clone::Clone for Array<T> {
    #[inline]
    fn clone(&self) -> Array<T> {
        Array(::core::clone::Clone::clone(&self.0))
    }
}
pub struct BinaryOperation {
    pub lhs: Box<Spanned<Node>>,
    pub op: Operator,
    pub rhs: Box<Spanned<Node>>,
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for BinaryOperation {}
#[automatically_derived]
impl ::core::cmp::PartialEq for BinaryOperation {
    #[inline]
    fn eq(&self, other: &BinaryOperation) -> bool {
        self.lhs == other.lhs && self.op == other.op && self.rhs == other.rhs
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for BinaryOperation {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "BinaryOperation",
            "lhs",
            &self.lhs,
            "op",
            &self.op,
            "rhs",
            &&self.rhs,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for BinaryOperation {
    #[inline]
    fn clone(&self) -> BinaryOperation {
        BinaryOperation {
            lhs: ::core::clone::Clone::clone(&self.lhs),
            op: ::core::clone::Clone::clone(&self.op),
            rhs: ::core::clone::Clone::clone(&self.rhs),
        }
    }
}
pub struct FnCall(pub Spanned<Ident>, pub Vec<Spanned<Node>>);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FnCall {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FnCall {
    #[inline]
    fn eq(&self, other: &FnCall) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for FnCall {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "FnCall", &self.0, &&self.1)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FnCall {
    #[inline]
    fn clone(&self) -> FnCall {
        FnCall(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
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
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Node {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Node {
    #[inline]
    fn eq(&self, other: &Node) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Node::BinaryExpression(__self_0), Node::BinaryExpression(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Node::Bool(__self_0), Node::Bool(__arg1_0)) => __self_0 == __arg1_0,
                (Node::Integer(__self_0), Node::Integer(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Node::Float(__self_0), Node::Float(__arg1_0)) => __self_0 == __arg1_0,
                (Node::Ident(__self_0), Node::Ident(__arg1_0)) => __self_0 == __arg1_0,
                (Node::Binding(__self_0), Node::Binding(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Node::FnCall(__self_0), Node::FnCall(__arg1_0)) => __self_0 == __arg1_0,
                (Node::String(__self_0), Node::String(__arg1_0)) => __self_0 == __arg1_0,
                (Node::FnDef(__self_0), Node::FnDef(__arg1_0)) => __self_0 == __arg1_0,
                (Node::EffectDef(__self_0), Node::EffectDef(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Node::StructDef(__self_0), Node::StructDef(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Node::UseDef(__self_0), Node::UseDef(__arg1_0)) => __self_0 == __arg1_0,
                (Node::Keyword(__self_0), Node::Keyword(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Node::ImplDef(__self_0), Node::ImplDef(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Node::Branch(__self_0), Node::Branch(__arg1_0)) => __self_0 == __arg1_0,
                (Node::FieldAccess(__self_0), Node::FieldAccess(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Node::Visibility(__self_0), Node::Visibility(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                _ => true,
            }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Node {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Node::BinaryExpression(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BinaryExpression",
                    &__self_0,
                )
            }
            Node::Bool(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Bool", &__self_0)
            }
            Node::Integer(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Integer",
                    &__self_0,
                )
            }
            Node::Float(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Float", &__self_0)
            }
            Node::Ident(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ident", &__self_0)
            }
            Node::Binding(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Binding",
                    &__self_0,
                )
            }
            Node::FnCall(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "FnCall", &__self_0)
            }
            Node::String(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "String", &__self_0)
            }
            Node::FnDef(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "FnDef", &__self_0)
            }
            Node::EffectDef(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "EffectDef",
                    &__self_0,
                )
            }
            Node::StructDef(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "StructDef",
                    &__self_0,
                )
            }
            Node::UseDef(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UseDef", &__self_0)
            }
            Node::Keyword(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Keyword",
                    &__self_0,
                )
            }
            Node::ImplDef(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "ImplDef",
                    &__self_0,
                )
            }
            Node::Branch(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Branch", &__self_0)
            }
            Node::FieldAccess(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "FieldAccess",
                    &__self_0,
                )
            }
            Node::Visibility(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Visibility",
                    &__self_0,
                )
            }
            Node::Error => ::core::fmt::Formatter::write_str(f, "Error"),
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Node {
    #[inline]
    fn clone(&self) -> Node {
        match self {
            Node::BinaryExpression(__self_0) => {
                Node::BinaryExpression(::core::clone::Clone::clone(__self_0))
            }
            Node::Bool(__self_0) => Node::Bool(::core::clone::Clone::clone(__self_0)),
            Node::Integer(__self_0) => {
                Node::Integer(::core::clone::Clone::clone(__self_0))
            }
            Node::Float(__self_0) => Node::Float(::core::clone::Clone::clone(__self_0)),
            Node::Ident(__self_0) => Node::Ident(::core::clone::Clone::clone(__self_0)),
            Node::Binding(__self_0) => {
                Node::Binding(::core::clone::Clone::clone(__self_0))
            }
            Node::FnCall(__self_0) => Node::FnCall(::core::clone::Clone::clone(__self_0)),
            Node::String(__self_0) => Node::String(::core::clone::Clone::clone(__self_0)),
            Node::FnDef(__self_0) => Node::FnDef(::core::clone::Clone::clone(__self_0)),
            Node::EffectDef(__self_0) => {
                Node::EffectDef(::core::clone::Clone::clone(__self_0))
            }
            Node::StructDef(__self_0) => {
                Node::StructDef(::core::clone::Clone::clone(__self_0))
            }
            Node::UseDef(__self_0) => Node::UseDef(::core::clone::Clone::clone(__self_0)),
            Node::Keyword(__self_0) => {
                Node::Keyword(::core::clone::Clone::clone(__self_0))
            }
            Node::ImplDef(__self_0) => {
                Node::ImplDef(::core::clone::Clone::clone(__self_0))
            }
            Node::Branch(__self_0) => Node::Branch(::core::clone::Clone::clone(__self_0)),
            Node::FieldAccess(__self_0) => {
                Node::FieldAccess(::core::clone::Clone::clone(__self_0))
            }
            Node::Visibility(__self_0) => {
                Node::Visibility(::core::clone::Clone::clone(__self_0))
            }
            Node::Error => Node::Error,
        }
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::BinaryExpression(bin) => {
                f.write_fmt(format_args!("{0} {1} {2}", bin.lhs, bin.op, bin.rhs))
            }
            Node::Bool(b) => f.write_fmt(format_args!("{0}", b)),
            Node::Integer(i) => f.write_fmt(format_args!("{0}", i)),
            Node::Float(fl) => f.write_fmt(format_args!("{0}", fl)),
            Node::Ident(ident) => f.write_fmt(format_args!("{0}", ident.1)),
            Node::Binding(bind) => f.write_fmt(format_args!("{0} = {1}", bind.0, bind.1)),
            Node::FnCall(call) => {
                f.write_fmt(
                    format_args!(
                        "{0}({1})",
                        call.0,
                        call
                            .1
                            .iter()
                            .map(|e| e.1.to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                    ),
                )
            }
            Node::String(s) => f.write_fmt(format_args!("{0}", s)),
            Node::FnDef(def) => f.write_fmt(format_args!("{0}", def.0)),
            Node::EffectDef(def) => f.write_fmt(format_args!("{0}", def.0)),
            Node::StructDef(def) => f.write_fmt(format_args!("{0}", def.0)),
            Node::UseDef(def) => f.write_fmt(format_args!("{0:#?}", def.0)),
            Node::Keyword(kw) => f.write_fmt(format_args!("{0}", kw)),
            Node::ImplDef(def) => f.write_fmt(format_args!("{0}", def.0)),
            Node::Branch(branch) => f.write_fmt(format_args!("{0}", branch.0)),
            Node::FieldAccess(access) => {
                f.write_fmt(format_args!("{0}.{1}", access.0, access.1))
            }
            Node::Visibility(vis) => f.write_fmt(format_args!("{0}", vis)),
            Node::Error => f.write_fmt(format_args!("Error")),
        }
    }
}
pub enum FnArg {
    Reciever,
    Field(Spanned<FieldDef>),
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FnArg {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FnArg {
    #[inline]
    fn eq(&self, other: &FnArg) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (FnArg::Field(__self_0), FnArg::Field(__arg1_0)) => __self_0 == __arg1_0,
                _ => true,
            }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for FnArg {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            FnArg::Reciever => ::core::fmt::Formatter::write_str(f, "Reciever"),
            FnArg::Field(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Field", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FnArg {
    #[inline]
    fn clone(&self) -> FnArg {
        match self {
            FnArg::Reciever => FnArg::Reciever,
            FnArg::Field(__self_0) => FnArg::Field(::core::clone::Clone::clone(__self_0)),
        }
    }
}
impl Display for FnArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FnArg::Reciever => f.write_fmt(format_args!("self")),
            FnArg::Field(field) => f.write_fmt(format_args!("{0}", field.1)),
        }
    }
}
pub struct Prototype {
    pub name: Spanned<Ident>,
    pub args: Vec<Spanned<FnArg>>,
    pub ret: Option<Spanned<Ident>>,
    pub effects: Vec<Spanned<Ident>>,
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Prototype {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Prototype {
    #[inline]
    fn eq(&self, other: &Prototype) -> bool {
        self.name == other.name && self.args == other.args && self.ret == other.ret
            && self.effects == other.effects
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Prototype {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Prototype",
            "name",
            &self.name,
            "args",
            &self.args,
            "ret",
            &self.ret,
            "effects",
            &&self.effects,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Prototype {
    #[inline]
    fn clone(&self) -> Prototype {
        Prototype {
            name: ::core::clone::Clone::clone(&self.name),
            args: ::core::clone::Clone::clone(&self.args),
            ret: ::core::clone::Clone::clone(&self.ret),
            effects: ::core::clone::Clone::clone(&self.effects),
        }
    }
}
impl Display for Prototype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}(", self.name))?;
        for arg in self.args.iter() {
            f.write_fmt(format_args!("{0}", arg))?;
        }
        f.write_fmt(format_args!(")"))?;
        if let Some(ret) = &self.ret {
            f.write_fmt(format_args!(" -> {0}", ret))?;
        }
        Ok(())
    }
}
pub enum Whitespace {
    Space,
    Tab,
    Newline,
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Whitespace {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Whitespace {
    #[inline]
    fn eq(&self, other: &Whitespace) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Whitespace {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Whitespace::Space => "Space",
                Whitespace::Tab => "Tab",
                Whitespace::Newline => "Newline",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Whitespace {
    #[inline]
    fn clone(&self) -> Whitespace {
        match self {
            Whitespace::Space => Whitespace::Space,
            Whitespace::Tab => Whitespace::Tab,
            Whitespace::Newline => Whitespace::Newline,
        }
    }
}
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
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Operator {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Operator {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Operator {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Operator::Add => "Add",
                Operator::Sub => "Sub",
                Operator::Mul => "Mul",
                Operator::Div => "Div",
                Operator::Modulo => "Modulo",
                Operator::Increment => "Increment",
                Operator::Decrement => "Decrement",
                Operator::Maybe => "Maybe",
                Operator::Not => "Not",
                Operator::Neg => "Neg",
                Operator::Dot => "Dot",
                Operator::Arrow => "Arrow",
                Operator::FatArrow => "FatArrow",
                Operator::DoubleColon => "DoubleColon",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Operator {
    #[inline]
    fn clone(&self) -> Operator {
        match self {
            Operator::Add => Operator::Add,
            Operator::Sub => Operator::Sub,
            Operator::Mul => Operator::Mul,
            Operator::Div => Operator::Div,
            Operator::Modulo => Operator::Modulo,
            Operator::Increment => Operator::Increment,
            Operator::Decrement => Operator::Decrement,
            Operator::Maybe => Operator::Maybe,
            Operator::Not => Operator::Not,
            Operator::Neg => Operator::Neg,
            Operator::Dot => Operator::Dot,
            Operator::Arrow => Operator::Arrow,
            Operator::FatArrow => Operator::FatArrow,
            Operator::DoubleColon => Operator::DoubleColon,
        }
    }
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
        f.write_fmt(format_args!("{0}", op))
    }
}
pub struct FieldAccess(pub Box<Spanned<Node>>, pub Box<Spanned<Node>>);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FieldAccess {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FieldAccess {
    #[inline]
    fn eq(&self, other: &FieldAccess) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for FieldAccess {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(
            f,
            "FieldAccess",
            &self.0,
            &&self.1,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FieldAccess {
    #[inline]
    fn clone(&self) -> FieldAccess {
        FieldAccess(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
pub struct Module(pub Vec<Spanned<Node>>);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Module {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Module {
    #[inline]
    fn eq(&self, other: &Module) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Module {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Module", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Module {
    #[inline]
    fn clone(&self) -> Module {
        Module(::core::clone::Clone::clone(&self.0))
    }
}
pub struct FieldDef(pub Spanned<Visibility>, pub Spanned<Ident>, pub Spanned<Ident>);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FieldDef {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FieldDef {
    #[inline]
    fn eq(&self, other: &FieldDef) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for FieldDef {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field3_finish(
            f,
            "FieldDef",
            &self.0,
            &self.1,
            &&self.2,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FieldDef {
    #[inline]
    fn clone(&self) -> FieldDef {
        FieldDef(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
            ::core::clone::Clone::clone(&self.2),
        )
    }
}
impl Display for FieldDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0}: {1}", self.0, self.1))
    }
}
pub struct StructDef(
    pub KeywordAndVisibility,
    pub Spanned<Ident>,
    pub Block<Spanned<FieldDef>>,
);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for StructDef {}
#[automatically_derived]
impl ::core::cmp::PartialEq for StructDef {
    #[inline]
    fn eq(&self, other: &StructDef) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for StructDef {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field3_finish(
            f,
            "StructDef",
            &self.0,
            &self.1,
            &&self.2,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for StructDef {
    #[inline]
    fn clone(&self) -> StructDef {
        StructDef(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
            ::core::clone::Clone::clone(&self.2),
        )
    }
}
pub struct FnIdent(pub Ident);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FnIdent {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FnIdent {
    #[inline]
    fn eq(&self, other: &FnIdent) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for FnIdent {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "FnIdent", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FnIdent {
    #[inline]
    fn clone(&self) -> FnIdent {
        FnIdent(::core::clone::Clone::clone(&self.0))
    }
}
pub struct EffectDef(
    pub KeywordAndVisibility,
    pub Spanned<Ident>,
    pub Vec<Spanned<Ident>>,
    pub Block<Spanned<Prototype>>,
);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EffectDef {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EffectDef {
    #[inline]
    fn eq(&self, other: &EffectDef) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EffectDef {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field4_finish(
            f,
            "EffectDef",
            &self.0,
            &self.1,
            &self.2,
            &&self.3,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for EffectDef {
    #[inline]
    fn clone(&self) -> EffectDef {
        EffectDef(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
            ::core::clone::Clone::clone(&self.2),
            ::core::clone::Clone::clone(&self.3),
        )
    }
}
pub struct UseDef(pub KeywordAndVisibility, pub Vec<Spanned<Ident>>, pub Spanned<Ident>);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UseDef {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UseDef {
    #[inline]
    fn eq(&self, other: &UseDef) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UseDef {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field3_finish(
            f,
            "UseDef",
            &self.0,
            &self.1,
            &&self.2,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for UseDef {
    #[inline]
    fn clone(&self) -> UseDef {
        UseDef(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
            ::core::clone::Clone::clone(&self.2),
        )
    }
}
impl Display for UseDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0} {1:?} from {2}", self.0, self.1, self.2))
    }
}
pub struct KeywordAndVisibility(pub Spanned<Keyword>, pub Spanned<Visibility>);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for KeywordAndVisibility {}
#[automatically_derived]
impl ::core::cmp::PartialEq for KeywordAndVisibility {
    #[inline]
    fn eq(&self, other: &KeywordAndVisibility) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for KeywordAndVisibility {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(
            f,
            "KeywordAndVisibility",
            &self.0,
            &&self.1,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for KeywordAndVisibility {
    #[inline]
    fn clone(&self) -> KeywordAndVisibility {
        KeywordAndVisibility(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
impl Display for KeywordAndVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0} {1}", self.1, self.0))
    }
}
pub struct ImplDef(
    pub KeywordAndVisibility,
    pub Spanned<Ident>,
    pub Option<Spanned<Ident>>,
    pub Block<Spanned<Node>>,
);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ImplDef {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ImplDef {
    #[inline]
    fn eq(&self, other: &ImplDef) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for ImplDef {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field4_finish(
            f,
            "ImplDef",
            &self.0,
            &self.1,
            &self.2,
            &&self.3,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for ImplDef {
    #[inline]
    fn clone(&self) -> ImplDef {
        ImplDef(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
            ::core::clone::Clone::clone(&self.2),
            ::core::clone::Clone::clone(&self.3),
        )
    }
}
pub struct BranchDef(
    pub Box<Spanned<Node>>,
    pub Vec<(Spanned<Node>, Block<Spanned<Node>>)>,
);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for BranchDef {}
#[automatically_derived]
impl ::core::cmp::PartialEq for BranchDef {
    #[inline]
    fn eq(&self, other: &BranchDef) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for BranchDef {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(
            f,
            "BranchDef",
            &self.0,
            &&self.1,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for BranchDef {
    #[inline]
    fn clone(&self) -> BranchDef {
        BranchDef(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
pub struct FnDef(
    pub KeywordAndVisibility,
    pub Spanned<Prototype>,
    pub Block<Spanned<Node>>,
    pub Vec<(Spanned<Ident>, Block<Spanned<Node>>)>,
);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for FnDef {}
#[automatically_derived]
impl ::core::cmp::PartialEq for FnDef {
    #[inline]
    fn eq(&self, other: &FnDef) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for FnDef {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field4_finish(
            f,
            "FnDef",
            &self.0,
            &self.1,
            &self.2,
            &&self.3,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FnDef {
    #[inline]
    fn clone(&self) -> FnDef {
        FnDef(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
            ::core::clone::Clone::clone(&self.2),
            ::core::clone::Clone::clone(&self.3),
        )
    }
}
impl Display for FnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0} {1} {{", self.0, self.1))?;
        for expr in self.2.0.iter() {
            f.write_fmt(format_args!("{0}", expr))?;
        }
        f.write_fmt(format_args!("}}"))
    }
}
