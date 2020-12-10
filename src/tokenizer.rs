use std::iter::Peekable;
use std::str::Chars;

/// Representation of a Markdown Token, the ::Code one can be the more tricky
/// Because to appear must have 1 or 3 ::ReversedQuote s preceding and 
/// at the end, its not an ::String because it can contain reserved tokens 
/// inside
#[derive(Debug, Clone, PartialEq)]
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

/// The tokenizer is an Iterator that given a source and some indle-status
/// control variables will yield Tokens consuming the source
pub struct Tokenizer<'a> {
    /// Source Iterator that contains the raw text to tokenize
    source: Peekable<Chars<'a>>,

    /// Control variables
    /// TODO: Replace them with an enum for simpler and cleaner code
    number_of_quotes: usize,
    possible_code: bool,
    done_code: bool,
}

impl<'a> Tokenizer<'a> {
    /// Instantiates a new Tokenizer given the input `source`
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source: source.chars().peekable(),
            number_of_quotes: 0,
            possible_code: false,
            done_code: false,
        }
    }

    /// Reset the tokenizer to initial state replacing the `source`
    pub fn reset(&mut self, source: &'a str) {
        self.source = source.chars().peekable();

        // Reset control variables to initial state
        self.number_of_quotes = 0;
        self.possible_code = false;
        self.done_code = false;
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
impl<'a> Iterator for Tokenizer<'a> {
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