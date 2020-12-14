use markdown_compiler::compile_markdown;
use std::io::{self, Read};

/// Usage `cargo r --example demo < ./examples/input.txt`
fn main() -> io::Result<()> {
    // Read the markdown from the stdin
    let mut source = String::new(); 
    std::io::stdin().read_to_string(&mut source)?;

    // Compile it!
    let resulting_html = compile_markdown(&source);

    // Write it to `./result.html`
    std::fs::write("result.html", resulting_html)?;

    Ok(())
}