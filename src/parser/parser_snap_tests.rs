
use crate::parser::errors::pretty_errors;
use insta::assert_snapshot;


#[cfg(test)]
#[okstd::test]
fn test_empty_parser() {
    let input = " ";
    let mut errors = vec![];
    let wrapper = crate::lexer::TripleIterator::new(input);
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, wrapper);
    assert!(errors.is_empty());
    assert_snapshot!(format!("{:#?}", t.unwrap()), @r###"
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
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, wrapper);
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
    } when throws {
        raise(1)
    }"#;
    let mut errors = vec![];
    let wrapper = crate::lexer::TripleIterator::new(input);
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, wrapper);
    assert!(errors.is_empty());
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
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, wrapper);
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
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, wrapper);
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
            self.host.read("jobserver")
            if self.host.exec(arg0, args) {
                raise(1)
            }
        }
    }"#;
    let mut errors: Vec<lalrpop_util::ErrorRecovery<usize, crate::lexer::Token, &str>> = vec![];
    let wrapper = crate::lexer::TripleIterator::new(input);
    let t = crate::parser::src::SourceParser::new().parse(&mut errors, wrapper);
    if !errors.is_empty() {
        panic!("{}", pretty_errors(&input, errors));
    }
    assert_snapshot!(format!("{:#?}", t.unwrap()));
}
