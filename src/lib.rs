mod tokenizer;

use tokenizer::{Tokenizer, Token};

#[cfg(test)]
mod tests {
    use super::*;

    /// TODO: More intensive tests
    #[test]
    fn tokenizer_tests() {
        // Test for simple text
        let simple_text = 
        "# This is a big header\nThis is normal code\n`This is a code snippet`";
        let mut tokenizer = Tokenizer::new(simple_text);

        let mut tokens = vec![];
        while let Some(t) = tokenizer.next() {
            tokens.push(t);
        }
        assert_eq!(tokens, 
            vec![
                Token::Pad, Token::String("This is a big header".to_owned()), Token::NewLine, 
                Token::String("This is normal code".to_owned()), Token::NewLine,
                Token::ReversedQuote, Token::Code("This is a code snippet".to_owned()), Token::ReversedQuote
            ]
        );

        // Test for code
        let code_text =
"```rust
fn main() {\n\tlet a = vec![];\n\treturn a;\n}
```";
        tokenizer.reset(code_text);
        let mut tokens = vec![];
        while let Some(t) = tokenizer.next() {
            tokens.push(t);
        }
        println!("{:?}", tokens);
        assert_eq!(tokens, 
            vec![
                Token::ReversedQuote, Token::ReversedQuote, Token::ReversedQuote,
                Token::Code("rust\nfn main() {\n\tlet a = vec![];\n\treturn a;\n}\n".to_owned()),
                Token::ReversedQuote, Token::ReversedQuote, Token::ReversedQuote,
            ]
        );
    }
}
