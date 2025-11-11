use anyhow::{Result, anyhow};
use miniconf_parser::parser::{MiniConfParser, Rule};
use pest::Parser;
use pest::iterators::Pair;

fn parse_pair<'input>(rule: Rule, input: &'input str) -> Result<Pair<'input, Rule>> {
    let mut pairs = MiniConfParser::parse(rule, input)?;
    pairs
        .next()
        .ok_or_else(|| anyhow!("no pair matched for {rule:?}"))
}

#[test]
fn section_header_parses_hyphenated_names() -> Result<()> {
    let input = "[build-target]";
    let pair = parse_pair(Rule::section_header, input)?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn key_value_allows_inline_comments() -> Result<()> {
    let input = r#"name = "alpha" # inline comment"#;
    let pair = parse_pair(Rule::key_value, input)?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn value_rule_supports_all_atoms() -> Result<()> {
    let cases = ["\"quoted\"", "bare_word", "42", "-3.14", "true", "no"];

    for input in cases {
        let pair = parse_pair(Rule::value, input)?;
        assert_eq!(pair.as_str(), input);
    }
    Ok(())
}

#[test]
fn array_rule_preserves_nested_values() -> Result<()> {
    let pair = parse_pair(Rule::array, "[one, 2, \"three\"]")?;
    let values = pair
        .into_inner()
        .filter(|inner| inner.as_rule() == Rule::value)
        .count();
    assert_eq!(values, 3);
    Ok(())
}

#[test]
fn text_rule_handles_sections_and_root() -> Result<()> {
    let input = r#"
    # doc header
    title = Example
    enabled = true

    [database]
    host = localhost
    tags = [primary, read]
    "#;

    let pair = parse_pair(Rule::text, input)?;
    let text = pair.as_str();
    assert!(text.contains("title = Example"));
    assert!(text.contains("[database]"));
    Ok(())
}
