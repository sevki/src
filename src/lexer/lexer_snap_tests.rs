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
    - Integer(1), 1:1
    - Plus, 1:3
    - Integer(1), 1:5
    "###);
}

#[okstd::test]
fn test_1_plus_1_plus_1() {
    let input = "1 + 1 + 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Integer(1), 1:1
    - Plus, 1:3
    - Integer(1), 1:5
    - Plus, 1:7
    - Integer(1), 1:9
    "###);
}

#[okstd::test]
fn test_1_plus_1_plus_1_plus_1() {
    let input = "1 + 1 / 1 % 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Integer(1), 1:1
    - Plus, 1:3
    - Integer(1), 1:5
    - Divide, 1:7
    - Integer(1), 1:9
    - Percent, 1:11
    - Integer(1), 1:13
    "###);
}

#[okstd::test]
fn test_let_a_equals_1() {
    let input = "let a = 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Let), 1:3
    - Word(Ident("a")), 1:5
    - Equals, 1:7
    - Integer(1), 1:9
    "###);
}

#[okstd::test]
fn test_let_a_equals_1_plus_1() {
    let input = "let a = 1 + 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Let), 1:3
    - Word(Ident("a")), 1:5
    - Equals, 1:7
    - Integer(1), 1:9
    - Plus, 1:11
    - Integer(1), 1:13
    "###);
}

#[okstd::test]
fn test_let_a_equals_1_plus_3_point_14() {
    let input = "let a = 1 + 3.14";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Let), 1:3
    - Word(Ident("a")), 1:5
    - Equals, 1:7
    - Integer(1), 1:9
    - Plus, 1:11
    - Float(3.14), 1:16
    "###);
}

#[okstd::test]
fn test_let_a_equals_1_plus_3_point_14_plus_1() {
    let input = "let a = 1 + 3.14 + 1";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Let), 1:3
    - Word(Ident("a")), 1:5
    - Equals, 1:7
    - Integer(1), 1:9
    - Plus, 1:11
    - Float(3.14), 1:16
    - Plus, 1:18
    - Integer(1), 1:20
    "###);
}

#[okstd::test]
fn test_fn_foo() {
    let input = "fn foo() {}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 1:2
    - Word(Ident("foo")), 1:6
    - LeftParen, 1:7
    - RightParen, 1:8
    - LeftBrace, 1:10
    - RightBrace, 1:11
    "###);
}

#[okstd::test]
fn test_fn_foo_bar() {
    let input = "fn foo(bar) {}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 1:2
    - Word(Ident("foo")), 1:6
    - LeftParen, 1:7
    - Word(Ident("bar")), 1:10
    - RightParen, 1:11
    - LeftBrace, 1:13
    - RightBrace, 1:14
    "###);
}

#[okstd::test]
fn test_fn_foo_bar_baz() {
    let input = "fn foo(bar, baz) {

}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 1:2
    - Word(Ident("foo")), 1:6
    - LeftParen, 1:7
    - Word(Ident("bar")), 1:10
    - Comma, 1:11
    - Word(Ident("baz")), 1:15
    - RightParen, 1:16
    - LeftBrace, 1:18
    - NewLine, 2:0
    - NewLine, 3:0
    - RightBrace, 3:1
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
    - Word(Fn), 1:2
    - Word(Ident("foo")), 1:6
    - LeftParen, 1:7
    - Word(Ident("bar")), 1:10
    - Comma, 1:11
    - Word(Ident("baz")), 1:15
    - RightParen, 1:16
    - LeftBrace, 1:18
    - NewLine, 2:0
    - Word(Ident("qux")), 2:7
    - LeftParen, 2:8
    - RightParen, 2:9
    - NewLine, 3:0
    - RightBrace, 3:1
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
    - Word(Fn), 1:2
    - Word(Ident("foo")), 1:6
    - LeftParen, 1:7
    - Word(Ident("bar")), 1:10
    - Comma, 1:11
    - Word(Ident("baz")), 1:15
    - RightParen, 1:16
    - LeftBrace, 1:18
    - NewLine, 2:0
    - Word(Ident("qux")), 2:7
    - LeftParen, 2:8
    - Word(Ident("quux")), 2:12
    - RightParen, 2:13
    - NewLine, 3:0
    - RightBrace, 3:1
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
    - Word(Fn), 1:2
    - Word(Ident("foo")), 1:6
    - LeftParen, 1:7
    - Word(Ident("bar")), 1:10
    - Comma, 1:11
    - Word(Ident("baz")), 1:15
    - RightParen, 1:16
    - LeftBrace, 1:18
    - NewLine, 2:0
    - Word(Ident("qux")), 2:7
    - LeftParen, 2:8
    - Word(Ident("quux")), 2:12
    - Comma, 2:13
    - Float(3.14), 2:18
    - Comma, 2:19
    - Integer(3735928559), 2:29
    - RightParen, 2:30
    - NewLine, 3:0
    - RightBrace, 3:1
    "###);
}

#[okstd::test]
fn test_func_with_genetics() {
    let input = "fn foo<T>(bar: T)[throws, awaits, execs] {
    qux()
}";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Fn), 1:2
    - Word(Ident("foo")), 1:6
    - LessThan, 1:7
    - Word(Ident("T")), 1:8
    - GreaterThan, 1:9
    - LeftParen, 1:10
    - Word(Ident("bar")), 1:13
    - Colon, 1:14
    - Word(Ident("T")), 1:16
    - RightParen, 1:17
    - LeftBracket, 1:18
    - Word(Ident("throws")), 1:24
    - Comma, 1:25
    - Word(Ident("awaits")), 1:32
    - Comma, 1:33
    - Word(Ident("execs")), 1:39
    - RightBracket, 1:40
    - LeftBrace, 1:42
    - NewLine, 2:0
    - Word(Ident("qux")), 2:7
    - LeftParen, 2:8
    - RightParen, 2:9
    - NewLine, 3:0
    - RightBrace, 3:1
    "###);
}

#[okstd::test]
fn test_func_call_with_genetics() {
    let input = "foo<T>(bar: T)[vm]";
    let lexer = Lexer::new(input, 0);
    let actual_tokens = lexer.map_while(|t| Some(t)).collect::<Vec<_>>();
    assert_snapshot!(TokenStreamDisplay::from(actual_tokens), @r###"
    - Word(Ident("foo")), 1:3
    - LessThan, 1:4
    - Word(Ident("T")), 1:5
    - GreaterThan, 1:6
    - LeftParen, 1:7
    - Word(Ident("bar")), 1:10
    - Colon, 1:11
    - Word(Ident("T")), 1:13
    - RightParen, 1:14
    - LeftBracket, 1:15
    - Word(Ident("vm")), 1:17
    - RightBracket, 1:18
    "###);
}
