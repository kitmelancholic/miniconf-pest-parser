use miniconf_pest_parser::{Document, MiniConfError, Value};

fn main() -> Result<(), MiniConfError> {
    let example = r#"
# Коментар
title = "My Project"
version = 1.0
features = ["fast", "elegant"]
flags = { debug: true, metrics: false }
nothing = null

[database]
host = "localhost"
port = 5432
tags = ["primary", "ro"]
"#;

    let doc = Document::parse(example)?;

    println!("{:#?}", doc);

    if let Some(Value::Num(port)) = doc.sections.get("database").and_then(|db| db.get("port")) {
        println!("DB port = {}", port);
    }
    Ok(())
}
