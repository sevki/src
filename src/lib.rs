pub mod lexer;
pub mod parser;
pub mod compiler;

use compiler::text;


use crate::compiler::ir;

#[salsa::jar(db = Db)]
pub struct Jar(
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
    ir::Program,
    ir::Function,
    ir::InternedEffect,
    ir::Symbol,
    ir::EffectDef,
    ir::Import,
    ir::Mangled,
);

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
