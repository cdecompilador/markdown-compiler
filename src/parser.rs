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
                Token::Pad => {
                    self.source.next();
                    if let Some(t2) = self.source.peek() {
                        match t2.clone() {
                            Token::String(s) => {
                                self.source.next();
                                MDValue::BigHeader(s.clone())
                            },
                            _ => panic!("Unexpected, got {:?}", t2),
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