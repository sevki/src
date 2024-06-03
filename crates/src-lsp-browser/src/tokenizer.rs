use js_sys::JsString;
use srclang::lexer::{self};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub scope: TokenType,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum TokenType {
    Pipe,
    Ampersand,
    Semicolon,
    Equals,
    LessThan,
    GreaterThan,
    Variable,
    Word,
    String,
    Comment,
    Integer,
    Float,
    Eof,
    NewLine,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Underscore,
    Minus,
    Plus,
    Arrow,
    FatArrow,
    Divide,
    Multiply,
    Percent,
    Dollar,
    Exclamation,
    Question,
    Tilde,
    At,
    Caret,
    Shebang,
}

#[wasm_bindgen]
pub fn token_type_as_js_string(token_type: TokenType) -> JsString {
    match token_type {
        TokenType::Pipe => "Pipe",
        TokenType::Ampersand => "Ampersand",
        TokenType::Semicolon => "Semicolon",
        TokenType::Equals => "Equals",
        TokenType::LessThan => "LessThan",
        TokenType::GreaterThan => "GreaterThan",
        TokenType::Variable => "Variable",
        TokenType::Word => "Word",
        TokenType::String => "String",
        TokenType::Comment => "Comment",
        TokenType::Integer => "Integer",
        TokenType::Float => "Float",
        TokenType::Eof => "Eof",
        TokenType::NewLine => "NewLine",
        TokenType::LeftParen => "LeftParen",
        TokenType::RightParen => "RightParen",
        TokenType::LeftBrace => "LeftBrace",
        TokenType::RightBrace => "RightBrace",
        TokenType::LeftBracket => "LeftBracket",
        TokenType::RightBracket => "RightBracket",
        TokenType::Comma => "Comma",
        TokenType::Dot => "Dot",
        TokenType::Colon => "Colon",
        TokenType::Underscore => "Underscore",
        TokenType::Minus => "Minus",
        TokenType::Plus => "Plus",
        TokenType::Arrow => "Arrow",
        TokenType::FatArrow => "FatArrow",
        TokenType::Divide => "Divide",
        TokenType::Multiply => "Multiply",
        TokenType::Percent => "Percent",
        TokenType::Dollar => "Dollar",
        TokenType::Exclamation => "Exclamation",
        TokenType::Question => "Question",
        TokenType::Tilde => "Tilde",
        TokenType::At => "At",
        TokenType::Caret => "Caret",
        TokenType::Shebang => "Shebang",
    }
    .into()
}

#[wasm_bindgen]
pub fn tokenize(input: &str) -> Result<Vec<TokenSpan>, JsValue> {
    let lexer = srclang::lexer::Lexer::new(input, 0);

    let tokens: Vec<TokenSpan> = lexer
        .map(|token| TokenSpan {
            start: token.start,
            end: token.end,
            scope: match token.node {
                lexer::Token::Pipe => TokenType::Pipe,
                lexer::Token::Ampersand => TokenType::Ampersand,
                lexer::Token::Semicolon => TokenType::Semicolon,
                lexer::Token::Equals => TokenType::Equals,
                lexer::Token::LessThan => TokenType::LessThan,
                lexer::Token::GreaterThan => TokenType::GreaterThan,
                lexer::Token::Variable(_) => TokenType::Variable,
                lexer::Token::Word(_) => TokenType::Word,
                lexer::Token::String(_) => TokenType::String,
                lexer::Token::Comment(_) => TokenType::Comment,
                lexer::Token::Integer(_) => TokenType::Integer,
                lexer::Token::Float(_) => TokenType::Float,
                lexer::Token::Eof => TokenType::Eof,
                lexer::Token::NewLine => TokenType::NewLine,
                lexer::Token::LeftParen => TokenType::LeftParen,
                lexer::Token::RightParen => TokenType::RightParen,
                lexer::Token::LeftBrace => TokenType::LeftBrace,
                lexer::Token::RightBrace => TokenType::RightBrace,
                lexer::Token::LeftBracket => TokenType::LeftBracket,
                lexer::Token::RightBracket => TokenType::RightBracket,
                lexer::Token::Comma => TokenType::Comma,
                lexer::Token::Dot => TokenType::Dot,
                lexer::Token::Colon => TokenType::Colon,
                lexer::Token::Underscore => TokenType::Underscore,
                lexer::Token::Minus => TokenType::Minus,
                lexer::Token::Plus => TokenType::Plus,
                lexer::Token::Arrow => TokenType::Arrow,
                lexer::Token::FatArrow => TokenType::FatArrow,
                lexer::Token::Divide => TokenType::Divide,
                lexer::Token::Multiply => TokenType::Multiply,
                lexer::Token::Percent => TokenType::Percent,
                lexer::Token::Dollar => TokenType::Dollar,
                lexer::Token::Exclamation => TokenType::Exclamation,
                lexer::Token::Question => TokenType::Question,
                lexer::Token::Tilde => TokenType::Tilde,
                lexer::Token::At => TokenType::At,
                lexer::Token::Caret => TokenType::Caret,
                lexer::Token::Shebang => TokenType::Shebang,
            },
        })
        .collect();

    Ok(tokens)
}
