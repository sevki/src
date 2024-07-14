use crate::{analyzer, lexer::Location, parser::errors::pretty_errors};
use insta::assert_snapshot;

#[cfg(test)]
#[okstd::test]
fn test_empty_parser() {
    use crate::analyzer;

    let input = " ";
    let mut errors = vec![];
    let wrapper = crate::lexer::TripleIterator::new(input);
    let db = analyzer::db::Database::default();
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, &db, wrapper);
    assert!(errors.is_empty());
    let fmted = format!("{:#?}", t.unwrap());
    assert_snapshot!(fmted, @r###"
    Module(
        [],
    )
    "###);
}

#[okstd::test]
fn test_fn_call_parser_with_multiple_args_and_strings() {
    let input = "fn some()[] {let a = some_fnExpr(1, \"2\", 3)}";
    let mut errors = vec![];
    let wrapper = crate::lexer::TripleIterator::new(input);
    let db = analyzer::db::Database::default();
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, &db, wrapper);
    if !errors.is_empty() {
        panic!("{}", pretty_errors(&input, errors));
    }
    assert_snapshot!(format!("{:#?}", t.unwrap()));
}

#[okstd::test]
fn test_fn_def_parser() {
    let input = r#"fn call(a:b, b:c) [throws, awaits, execs] {
    call(1+1)
    let a = 1
}"#;
    let mut errors = vec![];
    let wrapper = crate::lexer::TripleIterator::new(input);
    let db = analyzer::db::Database::default();
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, &db, wrapper);
    if !errors.is_empty() {
        panic!("{}", pretty_errors(&input, errors));
    }
    assert_snapshot!(format!("{:#?}", t.unwrap()));
}

#[okstd::test]
fn test_effect() {
    let input = r#"effect VM: async + throws + execs {
    catch() []
    await<T>(f: Future<T>) [] -> T
    exec(arg0: string, args: stringvec) []
}"#;
    let mut errors = vec![];
    let wrapper = crate::lexer::TripleIterator::new(input);
    let db = analyzer::db::Database::default();
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, &db, wrapper);
    assert!(errors.is_empty());
    assert_snapshot!(format!("{:#?}", t.unwrap()));
}

#[okstd::test]
fn test_struct_parser() {
    let input = r#"struct VM {
    a: string
    b: string
}"#;
    let mut errors = vec![];
    let wrapper = crate::lexer::TripleIterator::new(input);
    let db = analyzer::db::Database::default();
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, &db, wrapper);
    assert!(errors.is_empty());
    assert_snapshot!(format!("{:#?}", t.unwrap()));
}

#[okstd::test]
fn test_enum_parser() {
    let input = r#"use { exec } from host

effect Make: async + throws + execs + reads + writes {
    catch() [throws]
    await<T>(f: Future<T>) [async, throws] -> T
    exec(arg0: string, args: stringvec) [Make] -> i32
    read(name: string) [reads] -> string
    write(name: string, value: string) [writes]
}

struct Local {
    host: host
}

impl Make for Local {
    fn catch(self) [throws] {
    }
    fn await<T>(f: Future<T>) [async, trhows] -> T {
        yield()
    }
    fn exec(self, arg0: string, args: vec<string>) [Vm] -> i32 {
        self.host.read("jobserver").await
        if self.host.exec(arg0, args).await {
            raise(1)
        }
    }
}"#;
    let mut errors = vec![];
    let wrapper = crate::lexer::TripleIterator::new(input);
    let db = analyzer::db::Database::default();
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, &db, wrapper);
    if !errors.is_empty() {
        panic!("{}", pretty_errors(&input, errors));
    }
    assert_snapshot!(format!("{:#?}", t.unwrap()));
}
