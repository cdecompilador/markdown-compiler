use std::iter::Peekable;

use super::tokenizer::{Token, Tokenizer};

/// Supported languages by the compiler, needed for the future adding of syntax
/// highlight of code snippets
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CSLanguage {
    /// Uknown language or not provided language so no syntax highlighting
    Uknown,

    /// The supported langs
    /// TODO: Still not supported and needed more langs
    Rust,
    C,
    Cpp,
}

/// A `MDValue` is a definition that can be directly compiled to html,
/// it is built by the `MDParser` from `Tokens`
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MDValue {
    BigHeader(String),
    MediumHeader(String),
    SmallHeader(String),
    VerySmallHeader(String),
    CodeSnippet((CSLanguage, String)),
    Text(String),
    NewLine,
}

/// Markdown Parser Iterator that from the Tokens Iterator will yield
/// MDValues
pub struct MDParser {
    source: Peekable<Tokenizer>,
}

impl MDParser {
    /// Instantiate a new `MDParser` given the `Tokenizer` that contains the
    /// tokens
    pub fn new(tokens: Tokenizer) -> Self {
        MDParser {
            source: tokens.peekable(),
        }
    }
    fn extract_lang(code: String) -> (CSLanguage, String) {
        let code_iter = code.lines();
        match code.lines().next() {
            Some("rust") => {
                (CSLanguage::Rust, code_iter.skip(1).map(|s| {
                    let mut s = s.to_owned();
                    s.push('\n');
                    s
                }).collect())
            }
            Some("c++") | Some("cpp") => {
                (CSLanguage::Cpp, code_iter.skip(1).map(|s| {
                    let mut s = s.to_owned();
                    s.push('\n');
                    s
                }).collect())
            }
            Some("c") => {
                (CSLanguage::C, code_iter.skip(1).map(|s| {
                    let mut s = s.to_owned();
                    s.push('\n');
                    s
                }).collect())
            }
            _ => (CSLanguage::Uknown, code)
        }
    }
}

/// The main usage of the MDParser, as Iterator
impl Iterator for MDParser {
    type Item = MDValue;

    /// TODO: Now with the double ended iterator I can simplify the paradigm
    /// TODO(2): Well i didn't used the double ended iteartor for the moment :p
    /// used here
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(t) = self.source.peek() {
            return Some(match t.clone() {
                // 1 PAD FOUND
                Token::Pad => {
                    self.source.next();
                    if let Some(t2) = self.source.peek() {
                        match t2.clone() {
                            // PAD AND STRING FOUND
                            Token::String(s) => {
                                self.source.next();
                                MDValue::BigHeader(s.clone())
                            },
                            // 2 PAD FOUND
                            Token::Pad => {
                                self.source.next();
                                if let Some(t3) = self.source.peek() {
                                    match t3.clone() {
                                        // 2 PAD AND STRING FOUND
                                        Token::String(s) => {
                                            self.source.next();
                                            MDValue::MediumHeader(s.clone())
                                        }
                                        // 3 PAD FOUND
                                        Token::Pad => {
                                            self.source.next();
                                            if let Some(t4) = self.source.peek() {
                                                match t4.clone() {
                                                    // 3 PAD AND STRING FOUND
                                                    Token::String(s) => {
                                                        self.source.next();
                                                        MDValue::SmallHeader(s.clone())
                                                    }
                                                    // 4 PAD FOUND
                                                    Token::Pad => {
                                                        self.source.next();
                                                        if let Some(t5) = self.source.peek() {
                                                            // 4 PAD AND STRING
                                                            if let Token::String(s) = t5.clone() {
                                                                self.source.next();
                                                                MDValue::VerySmallHeader(s.clone())
                                                            } else {
                                                                panic!("Expected string after 4 pads");
                                                            }
                                                        } else {
                                                            continue;
                                                        }
                                                    }
                                                    _ => panic!("asfaf"),
                                                }
                                            } else {
                                                continue;
                                            }
                                        }
                                        _ => panic!("Expected Pad or String after 2 pads but got {:?}", t3),
                                    } 
                                } else {
                                    continue;
                                }
                            }
                            _ => panic!("Expected Pad or String after pad but got {:?}", t2),
                        }
                    } else {
                        continue;
                    }
                }
                Token::NewLine => {
                    self.source.next();
                    MDValue::NewLine
                },
                Token::Asterisk => {
                    // Implement the bold text
                    todo!()
                }
                Token::ReversedQuote => {
                    self.source.next();
                    if let Some(t2) = self.source.peek() {
                        match t2.clone() {
                            Token::ReversedQuote => {
                                self.source.next();
                                if let Some(Token::ReversedQuote) = self.source.peek() {
                                    self.source.next();
                                    if let Some(Token::Code(c)) = self.source.peek() {
                                        let (lang, c) = MDParser::extract_lang(c.clone());
                                        // TODO: Create different syntax highlighters
                                        self.source.next();
                                        match (self.source.next(), self.source.next(), self.source.next()) {
                                            (Some(Token::ReversedQuote), Some(Token::ReversedQuote), Some(Token::ReversedQuote)) => {
                                                MDValue::CodeSnippet((lang, c))
                                            }
                                            (a,b,c) => panic!("Expected 3 ReversedQuotes after Code, got {:?} {:?} {:?}", a, b, c),
                                        }
                                    } else {
                                        panic!("Expected Code after 3 ReversedQuotes");
                                    }
                                } else {
                                    panic!("Expected another ReversedQuote after ReversedQuote");
                                }
                            }
                            Token::Code(c) => {
                                // TODO: Skip code and end_quote (checking if there is)
                                self.source.next();
                                if let Some(Token::ReversedQuote) = self.source.peek() {
                                    self.source.next();
                                    MDValue::CodeSnippet((CSLanguage::Uknown, c.clone()))
                                } else {
                                    panic!("Expected closing ReversedQuote for Code but got");
                                }
                            }
                            _ => panic!("Expected another ReversedQuoute or Code but got {:?}", t2),
                        }
                    } else {
                        continue;
                    }
                }
                _ => panic!("!! Unexpected, got {:?}", t),
            });
        }
        None
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_tests() {
        // Header tests
        let tokenizer = Tokenizer::new("# Hello World\n## Hello World\n### Hello World\n#### Hello World\n");
        let mut parser = MDParser::new(tokenizer); 
        let mut values = vec![];
        while let Some(v) = parser.next() {
            values.push(v);
        }
        assert_eq!(vec![
                   MDValue::BigHeader("Hello World".to_owned()), MDValue::NewLine,
                   MDValue::MediumHeader("Hello World".to_owned()), MDValue::NewLine,
                   MDValue::SmallHeader("Hello World".to_owned()), MDValue::NewLine,
                   MDValue::VerySmallHeader("Hello World".to_owned()), MDValue::NewLine,
        ], values);

        // Code snippets tests
        let tokenizer = Tokenizer::new("\n`cargo build --release`
```rust\nfn main() {\n}\n```\n");
        let mut parser = MDParser::new(tokenizer); 
        let mut values = vec![];
        while let Some(v) = parser.next() {
            values.push(v);
        }
        assert_eq!(vec![
            MDValue::NewLine,
            MDValue::CodeSnippet((CSLanguage::Uknown,"cargo build --release".to_owned())), 
            MDValue::NewLine,
            MDValue::CodeSnippet((CSLanguage::Rust, "fn main() {\n}\n".to_string())),
            MDValue::NewLine,
        ], values);

        // Nothing test
        let tokenizer = Tokenizer::new("");
        let mut parser = MDParser::new(tokenizer); 
        let mut values = vec![];
        while let Some(v) = parser.next() {
            values.push(v);
        }
        assert!(values.is_empty() == true);
    }
}
