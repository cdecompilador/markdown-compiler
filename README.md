# Markdown Compiler Library

## Purpose
Just translates markdown to html, probably the markdown implementation
will differ from the original in the future, some features will be missing
and others would be new.

The only purpose of this library is to let me write blog entries easily <br>
Blog Repo: [repo](https://github.com/cdecompilador/cdecompilador-blog-skeleton)<br>
Blog Website: [cdecompilador blog](https://cdecompilador.github.io/cdecompilador-blog-skeleton/)

## Demo example
```rust
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
```

## Objectives

✅ Parse Headers <br>
✅ Parse raw text <br>
✅ Parse Code snippets <br>
❌ Syntax highlighting <br>
❌ Parse bold and italic text <br>
❌ Add links support <br>
❌ Remove unnecessary code (DoubleEndedItearator not used) <br>
❌ Fix that tab hell in the parser <br>
❌ A lot more... <br>