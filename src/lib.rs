//! # src-lang
//! src-lang is a compiler is a language and a specification for a domain specific language
//! that is aimed at manipulating source code.
//!
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// `analyzer` is responsible for analyzing the syntax tree and building the symbol table.
pub mod analyzer;
/// `ast` contains the abstract syntax tree for the src-lang.
pub mod ast;
/// `compiler` contains the compiler for the src-lang.
pub mod compiler;
/// `lexer` contains the intermediate representation for the src-lang.
pub mod lexer;
/// `ops` contains the operations tree traversal operations for the src-lang.
pub mod ops;
/// `parser` contains the parser for the src-lang, which is written in LALRPOP.
pub mod parser;

use compiler::text;

use crate::compiler::ir;

#[salsa::jar(db = Db)]
/// The salsa database for the compiler.
/// The default convention is to implement this at the crate level.
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

/// The Db trait is a marker trait that is used to ensure that the Jar struct is a valid salsa database.
/// The default convention is to implement this at the crate level.
pub trait Db: salsa::DbWithJar<Jar> {}

/// Implement the Db trait for the Jar struct.
/// The default convention is to implement this at the crate level.
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
