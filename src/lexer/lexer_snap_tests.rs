#[cfg(test)]
use crate::lexer::{Lexer, TokenStreamDisplay};

use insta::assert_snapshot;
use okstd::prelude::*;


#[okstd::test]
fn test_empty_lexer() {
    let input = " ";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    "###);
}

#[okstd::test]
fn test_1_plus_1() {
    let input = "1 + 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Integer(1), 0:1
    - Plus, 0:3
    - Integer(1), 0:5
    "###);
}

#[okstd::test]
fn test_1_plus_1_plus_1() {
    let input = "1 + 1 + 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Integer(1), 0:1
    - Plus, 0:3
    - Integer(1), 0:5
    - Plus, 0:7
    - Integer(1), 0:9
    "###);
}

#[okstd::test]
fn test_1_plus_1_plus_1_plus_1() {
    let input = "1 + 1 / 1 % 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Integer(1), 0:1
    - Plus, 0:3
    - Integer(1), 0:5
    - Divide, 0:7
    - Integer(1), 0:9
    - Percent, 0:11
    - Integer(1), 0:13
    "###);
}

#[okstd::test]
fn test_let_a_equals_1() {
    let input = "let a = 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Let), 0:3
    - Word(Ident("a")), 0:5
    - Equals, 0:7
    - Integer(1), 0:9
    "###);
}

#[okstd::test]
fn test_let_a_equals_1_plus_1() {
    let input = "let a = 1 + 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Let), 0:3
    - Word(Ident("a")), 0:5
    - Equals, 0:7
    - Integer(1), 0:9
    - Plus, 0:11
    - Integer(1), 0:13
    "###);
}

#[okstd::test]
fn test_let_a_equals_1_plus_3_point_14() {
    let input = "let a = 1 + 3.14";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Let), 0:3
    - Word(Ident("a")), 0:5
    - Equals, 0:7
    - Integer(1), 0:9
    - Plus, 0:11
    - Float(3.14), 0:16
    "###);
}

#[okstd::test]
fn test_let_a_equals_1_plus_3_point_14_plus_1() {
    let input = "let a = 1 + 3.14 + 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Let), 0:3
    - Word(Ident("a")), 0:5
    - Equals, 0:7
    - Integer(1), 0:9
    - Plus, 0:11
    - Float(3.14), 0:16
    - Plus, 0:18
    - Integer(1), 0:20
    "###);
}

#[okstd::test]
fn test_fn_foo() {
    let input = "fn foo() {}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 0:2
    - Word(Ident("foo")), 0:6
    - LeftParen, 0:7
    - RightParen, 0:8
    - LeftBrace, 0:10
    - RightBrace, 0:11
    "###);
}

#[okstd::test]
fn test_fn_foo_bar() {
    let input = "fn foo(bar) {}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 0:2
    - Word(Ident("foo")), 0:6
    - LeftParen, 0:7
    - Word(Ident("bar")), 0:10
    - RightParen, 0:11
    - LeftBrace, 0:13
    - RightBrace, 0:14
    "###);
}

#[okstd::test]
fn test_fn_foo_bar_baz() {
    let input = "fn foo(bar, baz) {

}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 0:2
    - Word(Ident("foo")), 0:6
    - LeftParen, 0:7
    - Word(Ident("bar")), 0:10
    - Comma, 0:11
    - Word(Ident("baz")), 0:15
    - RightParen, 0:16
    - LeftBrace, 0:18
    - NewLine, 1:0
    - NewLine, 2:0
    - RightBrace, 2:1
    "###);
}

#[okstd::test]
fn test_fn_foo_bar_baz_qux() {
    let input = "fn foo(bar, baz) {
    qux()
}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 0:2
    - Word(Ident("foo")), 0:6
    - LeftParen, 0:7
    - Word(Ident("bar")), 0:10
    - Comma, 0:11
    - Word(Ident("baz")), 0:15
    - RightParen, 0:16
    - LeftBrace, 0:18
    - NewLine, 1:0
    - Word(Ident("qux")), 1:7
    - LeftParen, 1:8
    - RightParen, 1:9
    - NewLine, 2:0
    - RightBrace, 2:1
    "###);
}

#[okstd::test]
fn test_fn_foo_bar_baz_qux_quux() {
    let input = "fn foo(bar, baz) {
    qux(quux)
}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 0:2
    - Word(Ident("foo")), 0:6
    - LeftParen, 0:7
    - Word(Ident("bar")), 0:10
    - Comma, 0:11
    - Word(Ident("baz")), 0:15
    - RightParen, 0:16
    - LeftBrace, 0:18
    - NewLine, 1:0
    - Word(Ident("qux")), 1:7
    - LeftParen, 1:8
    - Word(Ident("quux")), 1:12
    - RightParen, 1:13
    - NewLine, 2:0
    - RightBrace, 2:1
    "###);
}

#[okstd::test]
fn test_fn_foo_bar_baz_qux_quux_quuz() {
    let input = "fn foo(bar, baz) {
    qux(quux, 3.14,0xdeadbeef)
}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 0:2
    - Word(Ident("foo")), 0:6
    - LeftParen, 0:7
    - Word(Ident("bar")), 0:10
    - Comma, 0:11
    - Word(Ident("baz")), 0:15
    - RightParen, 0:16
    - LeftBrace, 0:18
    - NewLine, 1:0
    - Word(Ident("qux")), 1:7
    - LeftParen, 1:8
    - Word(Ident("quux")), 1:12
    - Comma, 1:13
    - Float(3.14), 1:18
    - Comma, 1:19
    - Integer(3735928559), 1:29
    - RightParen, 1:30
    - NewLine, 2:0
    - RightBrace, 2:1
    "###);
}

#[okstd::test]
fn test_func_with_generics() {
    let input = "fn foo<T>(bar: T)[throws, awaits, execs] {
    qux()
}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 0:2
    - Word(Ident("foo")), 0:6
    - LessThan, 0:7
    - Word(Ident("T")), 0:8
    - GreaterThan, 0:9
    - LeftParen, 0:10
    - Word(Ident("bar")), 0:13
    - Colon, 0:14
    - Word(Ident("T")), 0:16
    - RightParen, 0:17
    - LeftBracket, 0:18
    - Word(Ident("throws")), 0:24
    - Comma, 0:25
    - Word(Ident("awaits")), 0:32
    - Comma, 0:33
    - Word(Ident("execs")), 0:39
    - RightBracket, 0:40
    - LeftBrace, 0:42
    - NewLine, 1:0
    - Word(Ident("qux")), 1:7
    - LeftParen, 1:8
    - RightParen, 1:9
    - NewLine, 2:0
    - RightBrace, 2:1
    "###);
}

#[okstd::test]
fn test_func_call_with_generics() {
    let input = "foo<T>(bar: T)[vm]";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Ident("foo")), 0:3
    - LessThan, 0:4
    - Word(Ident("T")), 0:5
    - GreaterThan, 0:6
    - LeftParen, 0:7
    - Word(Ident("bar")), 0:10
    - Colon, 0:11
    - Word(Ident("T")), 0:13
    - RightParen, 0:14
    - LeftBracket, 0:15
    - Word(Ident("vm")), 0:17
    - RightBracket, 0:18
    "###);
}

#[okstd::test]
fn test_identifier_with_member_access() {
    let input = "foo.bar.baz.qux";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Ident("foo")), 0:3
    - Dot, 0:4
    - Word(Ident("bar")), 0:7
    - Dot, 0:8
    - Word(Ident("baz")), 0:11
    - Dot, 0:12
    - Word(Ident("qux")), 0:15
    "###);
}
