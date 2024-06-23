use crate::lexer::Location;

pub mod ast;
mod errors;
#[cfg(test)]
mod parser_snap_tests;
mod string;
pub mod span;

pub mod src;


// pub fn parse(
//     src: &str,
// ) -> Result<
//     crate::parser::ast::Module,
//     Vec<lalrpop_util::ErrorRecovery<Coord, crate::lexer::Token, &str>>,
// > {
//     let mut errors: Vec<lalrpop_util::ErrorRecovery<Coord, crate::lexer::Token, &str>> = vec![];
//     let wrapper = crate::lexer::TripleIterator::new(src);
//     let module = crate::parser::src::SourceParser::new().parse(&mut errors, wrapper);
//     if !errors.is_empty() {
//         return Err(errors);
//     }
//     Ok(module.unwrap())
// }
