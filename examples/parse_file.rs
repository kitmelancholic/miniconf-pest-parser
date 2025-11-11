use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("provide a file path");
    let contents = fs::read_to_string(&path)?;
    let document = miniconf_parser::parse_str(&contents)?;

    println!("Parsed sections:");
    for (name, section) in document.sections() {
        println!("[{name}] ({} entries)", section.entries.len());
    }

    Ok(())
}
