pub mod analyzer;
pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod ops;
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
