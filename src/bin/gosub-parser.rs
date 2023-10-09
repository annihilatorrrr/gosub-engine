use anyhow::Result;
use gosub_engine::html5_parser::parser::document::Document;
use gosub_engine::html5_parser::{
    input_stream::{Confidence, Encoding, InputStream},
    parser::Html5Parser,
};
use std::cell::RefCell;
use std::fs;
use std::process::exit;
use std::rc::Rc;

fn bail(message: &str) -> ! {
    println!("{}", message);
    exit(1);
}

fn main() -> Result<()> {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| bail("Usage: gosub-parser <url>"));

    let html = if url.starts_with("http://") || url.starts_with("https://") {
        // Fetch the html from the url
        let response = ureq::get(&url).call()?;
        if response.status() != 200 {
            bail(&format!(
                "Could not get url. Status code {}",
                response.status()
            ));
        }
        response.into_string()?
    } else {
        // Get html from the file
        fs::read_to_string(&url)?
    };

    let mut stream = InputStream::new();
    stream.read_from_str(&html, Some(Encoding::UTF8));
    stream.set_confidence(Confidence::Certain);

    // If the encoding confidence is not Confidence::Certain, we should detect the encoding.
    if !stream.is_certain_encoding() {
        stream.detect_encoding()
    }

    let mut parser = Html5Parser::new(&mut stream);
    let document = Rc::new(RefCell::new(Document::new()));
    let parse_errors = parser.parse(document.clone())?;

    println!("Generated tree: \n\n {}", document.borrow());

    for e in parse_errors {
        println!("Parse Error: {}", e.message)
    }

    Ok(())
}
