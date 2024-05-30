#[cfg(test)]
use proptest::{prelude::*};

#[allow(unused_imports)]
use crate::lexer::{Position, Spanned, Lexer, Token};

proptest! {

    #[test]
    fn test_strings(rnd in ("[a-z]+", 1..10)) {
        let input = format!(r#"let {} = "{}""#, rnd.0, rnd.1);
        let lexer = Lexer::new(&input, 0);
        let tokens: Vec<Spanned<Token, Position>> = lexer.collect();
        assert_eq!(tokens.len(), 4);
    }
}
