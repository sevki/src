use std::{
    collections::BTreeMap,
    ops::{Range, RangeBounds},
};

use crate::{
    compiler::{errors::Errors, text::{Position, Span, Spanned}},
    parser::ast::{self, EffectDef, Module},
    Db,
};

use self::text::SourceProgram;

mod db;
mod errors;
pub mod ir;
mod tests;
pub mod text;


#[salsa::tracked]
pub fn compile(db: &dyn Db, src: SourceProgram) -> ir::Program {
    let mut errors: Vec<lalrpop_util::ErrorRecovery<usize, crate::lexer::Token, &str>> = vec![];
    let wrapper = crate::lexer::TripleIterator::new(src.text(db));
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, wrapper);
    // let mut errors_in_positions: Vec<ir::Position> = vec![];
    if !errors.is_empty() {
        for error_range in Into::<Errors>::into(errors) {
            text::to_spans(db, src);
        }
        panic!();
    }

    let modul = t.unwrap();
    let mut symbol_table = BTreeMap::new();
    for toplevel in modul.0 {
        match *toplevel {
            ast::Expression::BinaryExpression(_) => todo!(),
            ast::Expression::Bool(_) => todo!(),
            ast::Expression::Integer(_) => todo!(),
            ast::Expression::Float(_) => todo!(),
            ast::Expression::Ident(_) => todo!(),
            ast::Expression::Binding(_) => todo!(),
            ast::Expression::FnCall(_) => todo!(),
            ast::Expression::String(_) => todo!(),
            ast::Expression::FnDef(_) => {}
            ast::Expression::ShellCommand(_, _) => todo!(),
            ast::Expression::EffectDef(_) => todo!(),
            ast::Expression::StructDef(_) => todo!(),
            ast::Expression::UseDef(usedef) => {
                let import =
                    ir::Import::new(db, usedef.0.into_iter().map(|x| x.0).collect(), usedef.1 .0);
                for import in add_imports(db, import) {
                    symbol_table.insert(import, ir::Symbol::new(db, import));
                }
            }
            ast::Expression::Keyword(_) => todo!(),
            ast::Expression::ImplDef(_) => todo!(),
            ast::Expression::Branch(_) => todo!(),
            ast::Expression::Error => todo!(),
        }
    }
    let program = ir::Program::new(db, vec![], symbol_table);

    program
}

#[salsa::tracked]
pub fn compile_effect(db: &dyn Db, effect: ir::EffectDef) {}

#[salsa::tracked]
pub fn add_imports(db: &dyn Db, import: ir::Import) -> Vec<ir::Mangled> {
    let mut mangled = vec![];
    for imp in import.imports(db) {
        mangled.push(ir::Mangled::new(
            db,
            format!("{}_{}", import.module(db), imp),
        ));
    }
    mangled
}
