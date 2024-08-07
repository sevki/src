use std::collections::BTreeMap;

use okstd::prelude::debug;

use crate::{compiler::errors::Errors, lexer::Location, parser::ast, Db};

use self::text::SourceProgram;

pub mod errors;
pub mod ir;
pub mod text;

#[cfg(test)]
mod tests;

#[salsa::tracked]
pub fn compile(db: &dyn Db, src: SourceProgram) -> ir::Program {
    let mut errors: Vec<lalrpop_util::ErrorRecovery<Location, crate::lexer::Token<'_>, &str>> = vec![];
    let wrapper = crate::lexer::TripleIterator::new(src.text(db));
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, db, wrapper);
    // let mut errors_in_positions: Vec<ir::Position> = vec![];

    if !errors.is_empty() {
        let spans = text::to_spans(db, src);
        let _tokens = spans.tokens(db);

        for _error_range in Into::<Errors>::into(errors) {}
        // panic!();
    }

    let modul = t.unwrap();
    let symbol_table = BTreeMap::new();
    for toplevel in modul.0 {
        match toplevel.1 {
            ast::Node::Visibility(_) => todo!(),
            ast::Node::BinaryExpression(_) => todo!(),
            ast::Node::Bool(_) => todo!(),
            ast::Node::Integer(_) => todo!(),
            ast::Node::Float(_) => todo!(),
            ast::Node::Ident(_) => todo!(),
            ast::Node::Binding(_) => todo!(),
            ast::Node::FnCall(_) => todo!(),
            ast::Node::String(_) => todo!(),
            ast::Node::FnDef(_) => {
                debug!("Function definition");
            }
            ast::Node::EffectDef(_) => todo!(),
            ast::Node::StructDef(_) => todo!(),
            ast::Node::UseDef(_usedef) => {}
            ast::Node::Keyword(_) => todo!(),
            ast::Node::ImplDef(_) => todo!(),
            ast::Node::Branch(_) => todo!(),
            ast::Node::Error => todo!(),
            ast::Node::FieldAccess(_) => todo!(),
        }
    }
    

    ir::Program::new(db, vec![], symbol_table)
}

#[salsa::tracked]
pub fn compile_effect(_db: &dyn Db, _effect: ir::EffectDef) {}

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
