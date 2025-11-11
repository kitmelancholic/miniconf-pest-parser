use miniconf_parser::{Value, parse_str};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = parse_str(
        r#"
        [service]
        name = example
        replicas = 3
        "#,
    )?;

    if let Some(service) = doc.section("service") {
        for entry in &service.entries {
            match &entry.value {
                Value::String(s) => println!("{} = {s}", entry.key),
                Value::Number(n) => println!("{} = {n}", entry.key),
                other => println!("{} = {:?}", entry.key, other),
            }
        }
    }

    Ok(())
}
