mod parser;
mod tokenizer;
mod compiler;
mod errors;

/// Function that given an `source` &str compiles it from Markdown to Html
/// `Tokenize -> Parse -> Compile`
pub fn compile_markdown(source: &str) -> String {
    // Tokenize it
    let t = tokenizer::Tokenizer::new(source);
    // Parse it
    let mut parser = parser::MDParser::new(t); 
    let mut values = vec![];
    while let Some(v) = parser.next() {
        values.push(v);
    }
    let mut final_value = String::new();
    // Compile it
    for v in values {
        final_value.push_str(&format!("{}", v));
    }

    final_value
}