pub mod analyzer;
pub mod compiler;
pub mod lexer;
pub mod parser;

use compiler::text;

use crate::compiler::ir;

#[salsa::jar(db = Db)]
pub struct Jar(
    parser::span::ByteOrLineColOrCoordInterned,
    parser::span::SourceMap,
    analyzer::SyntaxTree,
    compiler::compile,
    compiler::compile_effect,
    compiler::add_imports,
    text::to_spans,
    text::calculate_line_lengths,
    text::Span,
    text::Spanned,
    text::Position,
    text::SourceMap,
    text::SourceProgram,
    text::Document,
    ir::Program,
    ir::Function,
    ir::InternedEffect,
    ir::Symbol,
    ir::EffectDef,
    ir::Import,
    ir::Mangled,
    analyzer::get_symbol,
    analyzer::add_file,
    analyzer::Url,
    analyzer::span_text,
);

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}

#[macro_export]
macro_rules! visitor {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($($field:ident: $ty:ty),*);) => {
        $(#[$meta])*
        $vis struct $name($($field: $ty),*);

        paste::paste! {
            trait [<$name Visitor>] {
                $(
                    visitor!(@visit $field, $ty);
                )*
            }
        }
    };

    (@visit $field:ident, Spanned<$t:ty>) => {
        paste::paste! {
            fn [<visit_ $t:snake _field>](&mut self, $t, Range<Location>);
        }
    };

    (@visit $field:ident, Option<Spanned<$t:ty>>) => {
        paste::paste! {
            fn [<visit_ $t:snake _field>](&mut self, Option<$t>, Range<Location>);
        }
    };

    (@visit $field:ident, Box<Spanned<$t:ty>>) => {
        paste::paste! {
            fn [<visit_ $t:snake _field>](&mut self, $t, Range<Location>);
        }
    };

    (@visit $field:ident, Vec<Spanned<$t:ty>>) => {
        paste::paste! {
            fn [<visit_ $t:snake _field>](&mut self, $t, Range<Location>);
        }
    };

    (@visit $field:ident, Block<Spanned<$t:ty>>) => {
        paste::paste! {
            fn [<visit_ $t:snake _field>](&mut self, $t, Range<Location>);
        }
    };
}
