use std::fmt;
use super::parser::{CSLanguage, MDValue, MDParser};

/// The compilation of the MDValue would be just implementing the Debug Trait
impl fmt::Display for MDValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = String::new();
        match self {
            MDValue::BigHeader(text) => {
                value.push_str(&format!("<h1>{}</h1>", text));
            },
            MDValue::MediumHeader(text) => {
                value.push_str(&format!("<h2>{}</h2>", text));
            }
            MDValue::SmallHeader(text) => {
                value.push_str(&format!("<h3>{}</h3>", text));
            }
            MDValue::VerySmallHeader(text) => {
                value.push_str(&format!("<h4>{}</h4>", text));
            }
            // Maybe use prettier library for syntax highlighting
            MDValue::CodeSnippet((lang, text)) => {
                value.push_str(&format!("<code>{}<code>", text));
            }
            MDValue::NewLine => {
                value.push('\n');
            }
            MDValue::Text(text) => {
                value.push_str(text);
            }
        };
        write!(f,"{}",value)
    }
}

