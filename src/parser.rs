use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::ast::{Document, Entry, Value};
use crate::error::{MiniConfError, ParseErrorKind};

const ROOT_SECTION: &str = "root";

#[derive(Parser)]
#[grammar = "grammar.pest"]
/// Generated pest parser for MiniConf.
pub struct MiniConfParser;

/// Parses MiniConf input into a [`Document`].
pub fn parse_document(source: &str) -> Result<Document, MiniConfError> {
    let mut document = Document::new();
    document.ensure_section_mut(ROOT_SECTION);
    let mut current_section = ROOT_SECTION.to_string();

    let pairs = MiniConfParser::parse(Rule::text, source)?
        .next()
        .expect("text rule must produce a pair")
        .into_inner();

    for pair in pairs {
        match pair.as_rule() {
            Rule::section_header => {
                let name = pair.into_inner().next().unwrap().as_str().to_string();
                current_section = name;
                document.ensure_section_mut(&current_section);
            }
            Rule::key_value => {
                handle_key_value(&mut document, &current_section, pair)?;
            }
            Rule::comment_line | Rule::EOI => {
                // comments and explicit end markers are intentionally ignored
            }
            _ => {}
        }
    }

    Ok(document)
}

fn handle_key_value(
    document: &mut Document,
    current_section: &str,
    pair: Pair<'_, Rule>,
) -> Result<(), MiniConfError> {
    let line = line_of(&pair);
    let mut inner = pair.into_inner();
    let key = inner.next().expect("key present").as_str().to_string();
    let value_pair = inner
        .find(|p| matches!(p.as_rule(), Rule::value))
        .expect("value present");
    let value = parse_value(value_pair)?;

    let section = document.ensure_section_mut(current_section);
    if section.entries.iter().any(|entry| entry.key == key) {
        return Err(MiniConfError::semantic(
            ParseErrorKind::DuplicateKey,
            line,
            format!("key `{key}` already defined in [{current_section}]"),
        ));
    }

    section.entries.push(Entry::new(key, value, line));
    Ok(())
}

fn parse_value(pair: Pair<'_, Rule>) -> Result<Value, MiniConfError> {
    match pair.as_rule() {
        Rule::value => {
            let inner = pair.into_inner().next().expect("value inner");
            parse_value(inner)
        }
        Rule::quoted_string => Ok(Value::String(unescape(pair.as_str()))),
        Rule::bare_string => Ok(Value::String(pair.as_str().to_string())),
        Rule::number => {
            let line = line_of(&pair);
            match pair.as_str().parse::<f64>() {
                Ok(num) => Ok(Value::Number(num)),
                Err(err) => Err(MiniConfError::semantic(
                    ParseErrorKind::InvalidValue,
                    line,
                    format!("invalid number: {err}"),
                )),
            }
        }
        Rule::boolean => Ok(Value::Bool(matches!(pair.as_str(), "true" | "yes"))),
        Rule::array => parse_array(pair),
        other => unreachable!("unexpected value rule: {other:?}"),
    }
}

fn parse_array(pair: Pair<'_, Rule>) -> Result<Value, MiniConfError> {
    let mut values = Vec::new();
    for inner in pair.into_inner() {
        if matches!(inner.as_rule(), Rule::value) {
            values.push(parse_value(inner)?);
        }
    }
    Ok(Value::Array(values))
}

fn unescape(raw: &str) -> String {
    let body = &raw[1..raw.len() - 1];
    let mut buf = String::with_capacity(body.len());
    let mut chars = body.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                match next {
                    '"' => buf.push('"'),
                    'n' => buf.push('\n'),
                    't' => buf.push('\t'),
                    '\\' => buf.push('\\'),
                    other => buf.push(other),
                }
            }
        } else {
            buf.push(ch);
        }
    }
    buf
}

fn line_of(pair: &Pair<'_, Rule>) -> usize {
    pair.as_span().start_pos().line_col().0
}
