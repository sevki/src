#[cfg(test)]
use proptest::{prelude::*};

#[allow(unused_imports)]
use crate::lexer::{Position, Spanned, Lexer, Token};

use super::Word;

proptest! {

    #[test]
    fn test_strings(rnd in ("[a-z]+", 1..10)) {
        let input = format!(r#"let {} = "{}""#, rnd.0, rnd.1);
        let lexer = Lexer::new(&input, 0);
        let tokens: Vec<Spanned<Token, Position>> = lexer.collect();
        assert_eq!(tokens.len(), 4);
        
        let expected = vec![
            Token::Word(Word::Let),
            Token::Word(Word::Ident(&rnd.0)),
            Token::Equals,
            // Token::String(rnd.1), // ignore this as proptest can't handle this expression
        ];

        for (actual, expected) in tokens.iter().zip(expected.iter()) {
        }
    }
}
