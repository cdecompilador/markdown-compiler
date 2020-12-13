use std::iter::Peekable;

use super::tokenizer::{Token, Tokenizer};

/// Supported languages by the compiler, needed for the future adding of syntax
/// highlight of code snippets
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
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
}

/// The main usage of the MDParser, as Iterator
impl Iterator for MDParser {
    type Item = MDValue;

    /// TODO: Now with the double ended iterator I can simplify the paradigm
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
                                                match t4 {
                                                    // 3 PAD AND STRING FOUND
                                                    Token::String(s) => {
                                                        MDValue::SmallHeader(s.clone())
                                                    }
                                                    // 4 PAD FOUND
                                                    Token::Pad => {
                                                        self.source.next();
                                                        if let Some(t5) = self.source.peek() {
                                                            // 4 PAD AND STRING
                                                            if let Token::String(s) = t5 {
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
                Token::NewLine => MDValue::NewLine,
                Token::Asterisk => {
                    // Implement the bold text
                    todo!()
                }
                Token::ReversedQuote => {
                    self.source.next();
                    if let Some(t) = self.source.peek() {
                        match t {
                            Token::ReversedQuote => {
                                todo!();
                            }
                            Token::Code(c) => {
                                // TODO: Skip code and end_quote (checking if there is)
                                MDValue::CodeSnippet((CSLanguage::Uknown, c.clone()))
                            }
                            _ => panic!("Expected another ReversedQuoute or Code"),
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
        let tokenizer = Tokenizer::new("# Hello World");
        let mut parser = MDParser::new(tokenizer); 
        let mut values = vec![];
        while let Some(v) = parser.next() {
            values.push(v);
        }

        assert_eq!(vec![MDValue::BigHeader("Hello World".to_owned())], values);
    }
}