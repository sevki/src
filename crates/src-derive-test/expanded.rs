#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use src_derive::node;
use srclang::lexer::Location;
use srclang::ops;
use srclang::parser::span::Spanned;
use srclang::span;
use std::fmt::Display;
use std::ops::Range;
struct Ident {
    name: Spanned<String>,
    generics: Vec<Spanned<Ident>>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Ident {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Ident",
            "name",
            &self.name,
            "generics",
            &&self.generics,
        )
    }
}
trait IdentVisitor {
    fn visit_name(
        &self,
        node: &String,
        span: &Range<Location>,
    ) -> ops::traversal::Result;
    fn visit_generics(
        &self,
        node: &Ident,
        span: &Range<Location>,
    ) -> ops::traversal::Result;
}
impl From<(Spanned<String>, Vec<Spanned<Ident>>)> for Ident {
    fn from(ident: (Spanned<String>, Vec<Spanned<Ident>>)) -> Self {
        Self::new(ident.0, ident.1)
    }
}
impl Ident {
    fn new(name: Spanned<String>, generics: Vec<Spanned<Ident>>) -> Self {
        Self { name, generics }
    }
}
impl Ident {
    fn accept(&self, visitor: &impl IdentVisitor) {
        if let cont = visitor.visit_name(&self.name.1, &(self.name.0..self.name.2)) {
            return;
        }
        for inner in self.generics.iter() {
            if let cont = visitor.visit_generics(&inner.1, &(inner.0..inner.2)) {
                return;
            }
        }
    }
}
struct Field {
    vis: Option<Spanned<Visibility>>,
    name: Spanned<String>,
    ty: Spanned<Ident>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Field {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Field",
            "vis",
            &self.vis,
            "name",
            &self.name,
            "ty",
            &&self.ty,
        )
    }
}
trait FieldVisitor {
    fn visit_vis(
        &self,
        node: &Visibility,
        span: &Range<Location>,
    ) -> ops::traversal::Result;
    fn visit_name(
        &self,
        node: &String,
        span: &Range<Location>,
    ) -> ops::traversal::Result;
    fn visit_ty(&self, node: &Ident, span: &Range<Location>) -> ops::traversal::Result;
}
impl From<(Option<Spanned<Visibility>>, Spanned<String>, Spanned<Ident>)> for Field {
    fn from(
        field: (Option<Spanned<Visibility>>, Spanned<String>, Spanned<Ident>),
    ) -> Self {
        Self::new(field.0, field.1, field.2)
    }
}
impl Field {
    fn new(
        vis: Option<Spanned<Visibility>>,
        name: Spanned<String>,
        ty: Spanned<Ident>,
    ) -> Self {
        Self { vis, name, ty }
    }
}
impl Field {
    fn accept(&self, visitor: &impl FieldVisitor) {
        if let Some(inner) = &self.vis {
            if let cont = visitor.visit_vis(&inner.1, &(inner.0..inner.2)) {
                return;
            }
        }
        if let cont = visitor.visit_name(&self.name.1, &(self.name.0..self.name.2)) {
            return;
        }
        if let cont = visitor.visit_ty(&self.ty.1, &(self.ty.0..self.ty.2)) {
            return;
        }
    }
}
pub enum Literal {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
}
pub enum Visibility {
    Private,
    Public,
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
impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => f.write_fmt(format_args!("pub")),
            Visibility::Private => f.write_fmt(format_args!("priv")),
        }
    }
}
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
struct PrettyPrinter;
impl FieldVisitor for PrettyPrinter {
    fn visit_vis(
        &self,
        vis: &Visibility,
        range: &Range<Location>,
    ) -> ops::traversal::Result {
        {
            ::std::io::_print(format_args!("{0} ", vis));
        };
        ops::traversal::Result::Continue
    }
    fn visit_name(
        &self,
        name: &String,
        range: &Range<Location>,
    ) -> ops::traversal::Result {
        {
            ::std::io::_print(format_args!("{0} :", name));
        };
        ops::traversal::Result::Continue
    }
    fn visit_ty(&self, ty: &Ident, range: &Range<Location>) -> ops::traversal::Result {
        ty.accept(self);
        ops::traversal::Result::Continue
    }
}
impl IdentVisitor for PrettyPrinter {
    fn visit_name(
        &self,
        name: &String,
        range: &Range<Location>,
    ) -> ops::traversal::Result {
        {
            ::std::io::_print(format_args!("{0}", name));
        };
        ops::traversal::Result::Continue
    }
    fn visit_generics(
        &self,
        generic: &Ident,
        range: &Range<Location>,
    ) -> ops::traversal::Result {
        {
            ::std::io::_print(format_args!("<"));
        };
        generic.accept(self);
        {
            ::std::io::_print(format_args!(">"));
        };
        ops::traversal::Result::Continue
    }
}
fn main() {
    let b = Ident::new(
        Spanned(Location::default(), "b".to_string(), Location::default()),
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                Spanned(
                    Location::default(),
                    Ident::new(
                        Spanned(
                            Location::default(),
                            "c".to_string(),
                            Location::default(),
                        ),
                        ::alloc::vec::Vec::new(),
                    ),
                    Location::default(),
                ),
            ]),
        ),
    );
    let a = Field::new(
        Some(Spanned(Location::default(), Visibility::Public, Location::default())),
        Spanned(Location::default(), "a".to_string(), Location::default()),
        Spanned(Location::default(), b, Location::default()),
    );
    let pretty_printer = PrettyPrinter;
    a.accept(&pretty_printer);
    {
        ::std::io::_print(format_args!("\n"));
    };
}
