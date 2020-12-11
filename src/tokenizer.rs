use std::iter::{Peekable, DoubleEndedIterator};
use std::str::Chars;

/// Representation of a Markdown Token, the ::Code one can be the more tricky
/// Because to appear must have 1 or 3 ::ReversedQuote s preceding and 
/// at the end, its not an ::String because it can contain reserved tokens 
/// inside
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// #: for headers
    Pad, 
    String(String),
    /// \n
    NewLine, 
    /// `: for code snippets
    ReversedQuote, 
    /// _: for italic
    LowBar,   
    /// *: for bold
    Asterisk, 
    /// ^ Read enum descr.
    Code(String), 
}

/// The TokenizerIterator is an Iterator that given a source and some indle-status
/// control variables will yield Tokens consuming the source
struct TokenizerIterator<'a> {
    /// Source Iterator that contains the raw text to tokenize
    source: Peekable<Chars<'a>>,

    /// Control variables
    /// TODO: Replace them with an enum for simpler and cleaner code
    number_of_quotes: usize,
    possible_code: bool,
    done_code: bool,
}

impl<'a> TokenizerIterator<'a> {
    /// Instantiates a new Tokenizer given the input `source`
    fn new(source: &'a str) -> Self {
        TokenizerIterator {
            source: source.chars().peekable(),

            number_of_quotes: 0,
            possible_code: false,
            done_code: false,
        }
    }

    /// Internal method that parses from the source a String, that its no more
    /// than the common text presented in markdown
    fn parse_string(&mut self, first_ch: char) -> String {
        let mut string = String::new();
        string.push(first_ch);
        while let Some(&ch) = self.source.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '\'' | ' '
                | '"' | '.' | '?' | '¿' | ',' | ':' | '(' | ')' => {
                    string.push(ch);
                    self.source.next();
                }
                '\n' | '#' | '_' | '`' | '*' => break,
                _ => panic!("Unexpected char while parsing string: {}", ch),
            }
        }
        string
    }

    /// Internal method that parses from the source code, its less restrictive
    /// than `self.parse_string` because code snippets can contain 
    /// reserved tokens
    fn parse_code(&mut self, first_ch: char) -> String {
        let mut string = String::new();
        string.push(first_ch);
        while let Some(&ch) = self.source.peek() {
            match ch {
                '`' => break,
                // Allow every kind of char to be put into the code snippet
                _ => {
                    string.push(ch);
                    self.source.next();   
                }
            }
        }
        string
    }
}

/// Iterator imlementation for the token, the main usage that the struct'll have
impl<'a> Iterator for TokenizerIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.source.next() {
            return Some(match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' 
                | '"' | '.' | '?' | '¿' | ',' | ':' | '(' | ')' => {
                    if self.possible_code {
                        let code_string = self.parse_code(ch);
                        self.done_code = true;
                        Token::Code(code_string)
                    } else {
                        let string = self.parse_string(ch);
                        Token::String(string)
                    }
                }
                ' '  => {
                    // The code snippets can start with an space but md not
                    if self.possible_code {
                        let code_string = self.parse_code(ch);
                        self.done_code = true;
                        Token::Code(code_string)
                    } else {
                        continue;
                    }
                },
                '\n' => Token::NewLine,
                '#'  => Token::Pad,
                '_'  => Token::LowBar,
                '`'  => {
                    // A little tricky logic to allow single ReversedQuote and
                    // triple ReversedQuote code snippets
                    self.number_of_quotes += 1;
                    if self.number_of_quotes == 1 
                        || self.number_of_quotes == 3 {
                        self.possible_code = true;
                    } else {
                        self.possible_code = false;
                    }
                    if self.done_code && (self.number_of_quotes == 6 
                        || self.number_of_quotes == 2) {
                        self.number_of_quotes = 0;
                        self.done_code = false;
                        self.possible_code = false;
                    }
                    Token::ReversedQuote

                },
                '*'  => Token::Asterisk,
                _    => panic!("Unexpected char: {}", ch),
            });
        }
        None
    }
}

/// Abstration of the TokenizerIterator to allow a Double ended Itearator, as 
/// result of this especific implementation its not only a DEIterator, its also
/// a circular iterator, once you get to the end after a `None` the iterator 
/// starts from the begining
#[derive(Debug)]
pub struct Tokenizer {
    /// The tokens already calculated
    tokens: Vec<Token>,

    /// The current token that its an optional tuple of the token and the 
    /// corresponding index
    curr_token: Option<(usize, Token)>,
}

impl Tokenizer {
    /// Instantiates a new `Tokenizer`, in the process pre tokenizes all the 
    /// source, so this instantiation can be slow
    pub fn new(source: &str) -> Self {
        let mut tok_iter = TokenizerIterator::new(source);
        let mut tokens = vec![];

        // Collect the tokens
        while let Some(t) = tok_iter.next() {
            tokens.push(t);
        }

        Tokenizer {
            tokens,
            curr_token: None,
        }
    }

    /// Replaces the tokenizer state with a new source recalculating all the 
    /// tokens, as the instantiation this can be slow
    pub fn reset(&mut self, source: &str) {
        let mut tok_iter = TokenizerIterator::new(source);
        self.tokens.clear();

        // Collect the tokens
        while let Some(t) = tok_iter.next() {
            self.tokens.push(t);
        }
        self.curr_token = None;
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // If the idx == last one it returns None, if it's None it returns 
        // the first element, otherwise just `next_back`. 
        // As result its a circular iterator
        if let Some((idx, _)) = &self.curr_token {
            if *idx == self.tokens.len() - 1 {
                self.curr_token = None;
            } else {
                self.curr_token = Some((idx+1, self.tokens[idx+1].clone()));
            }
        } else {
            self.curr_token = Some((0, self.tokens[0].clone()))
        }

        // From (idx, tok) to just tok
        self.curr_token.clone().map(|(_, tok)| tok)
    }
}

impl DoubleEndedIterator for Tokenizer {
    fn next_back(&mut self) -> Option<Self::Item> {
        // If the idx == 0 it returns None, if it's None it returns the last
        // element, otherwise just `next_back`. As result its a circular iterator
        if let Some((idx, _)) = &self.curr_token {
            if *idx == 0 {
                self.curr_token = None;
            } else {
                self.curr_token = Some((idx-1, self.tokens[idx-1].clone()));
            }
        } else {
            let pos = self.tokens.len() - 1;
            self.curr_token = Some((pos, self.tokens[pos].clone()))
        }

        // From (idx, tok) to just tok
        self.curr_token.clone().map(|(_, tok)| tok)
    }
}

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
        assert_eq!(tokens, 
            vec![
                Token::ReversedQuote, Token::ReversedQuote, Token::ReversedQuote,
                Token::Code("rust\nfn main() {\n\tlet a = vec![];\n\treturn a;\n}\n".to_owned()),
                Token::ReversedQuote, Token::ReversedQuote, Token::ReversedQuote,
            ]
        );
    }

    #[test]
    fn double_ended_tokenizer_iterator_test() {
        let mut tokenizer = Tokenizer::new("#***");
        tokenizer.next();
        tokenizer.next();
        tokenizer.next();
        tokenizer.next();

        tokenizer.next_back();
        tokenizer.next_back();
        assert_eq!(Some(Token::Pad), tokenizer.next_back());
    }
}