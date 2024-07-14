use src_derive::node;
use srclang::lexer::Location;
use srclang::ops;
use srclang::parser::span::Spanned;
use srclang::span;
use std::fmt::Display;
use std::ops::Range;

#[node]
struct Ident {
    name: String,
    generics: Vec<Ident>,
}

#[node]
struct Field {
    vis: Option<Visibility>,
    name: String,
    ty: Ident,
}

pub enum Literal {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
}

#[derive(Debug)]
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
    fn visit_vis(&self, vis: &Visibility, range: &Range<Location>) -> ops::traversal::Result {
        print!("{} ", vis);
        ops::traversal::Result::Continue
    }

    fn visit_name(&self, name: &String, range: &Range<Location>) -> ops::traversal::Result {
        print!("{} :", name);
        ops::traversal::Result::Continue
    }

    fn visit_ty(&self, ty: &Ident, range: &Range<Location>) -> ops::traversal::Result {
        ty.accept(self);
        ops::traversal::Result::Continue
    }
}

impl IdentVisitor for PrettyPrinter {
    fn visit_name(&self, name: &String, range: &Range<Location>) -> ops::traversal::Result {
        print!("{}", name);
        ops::traversal::Result::Continue
    }

    fn visit_generics(&self, generic: &Ident, range: &Range<Location>) -> ops::traversal::Result {
        print!("<");
        generic.accept(self);
        print!(">");
        ops::traversal::Result::Continue
    }
}

fn main() {
    let b = Ident::new(
        span!(Location::default(), "b".to_string(), Location::default()),
        vec![span!(
            Location::default(),
            Ident::new(
                span!(Location::default(), "c".to_string(), Location::default()),
                vec![]
            ),
            Location::default()
        )],
    );
    let a = Field::new(
        Some(span!(
            Location::default(),
            Visibility::Public,
            Location::default()
        )),
        span!(Location::default(), "a".to_string(), Location::default()),
        span!(Location::default(), b, Location::default()),
    );

    let pretty_printer = PrettyPrinter;
    a.accept(&pretty_printer);
    println!("");
}
