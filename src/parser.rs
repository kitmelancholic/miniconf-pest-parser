use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "miniconf.pest"]
struct MiniConfParser;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document {
    /// Map<section, Map<key, value>>
    pub sections: HashMap<String, HashMap<String, Value>>,
}

#[derive(thiserror::Error, Debug)]
pub enum MiniConfError {
    #[error("parse error: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
    #[error("number parse error: {0}")]
    Num(#[from] std::num::ParseFloatError),
}

impl Document {
    pub fn parse(input: &str) -> Result<Self, MiniConfError> {
        let mut current_section = String::from("root");
        let mut doc = Document::default();
        doc.sections.entry(current_section.clone()).or_default();

        let pairs = MiniConfParser::parse(Rule::file, input)?;

        for pair in pairs.flatten() {
            match pair.as_rule() {
                Rule::section => {
                    let mut inner = pair.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();
                    current_section = name;
                    doc.sections.entry(current_section.clone()).or_default();
                }
                Rule::kv => {
                    let mut inner = pair.into_inner();
                    let key = inner.next().unwrap().as_str().to_string();
                    let val_pair = inner.next().unwrap();
                    let val = parse_value(val_pair)?;
                    doc.sections
                        .entry(current_section.clone())
                        .or_default()
                        .insert(key, val);
                }
                _ => {}
            }
        }
        Ok(doc)
    }
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> Result<Value, MiniConfError> {
    Ok(match pair.as_rule() {
        Rule::string => {
            let raw = pair.as_str();
            let inner = &raw[1..raw.len() - 1];
            let s = inner
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\t", "\t");
            Value::Str(s)
        }
        Rule::number => Value::Num(pair.as_str().parse::<f64>()?),
        Rule::boolean => Value::Bool(pair.as_str() == "true"),
        Rule::null => Value::Null,
        Rule::array => {
            let items = pair
                .into_inner()
                .filter(|p| matches!(p.as_rule(), Rule::value))
                .map(parse_value)
                .collect::<Result<Vec<_>, _>>()?;
            Value::Array(items)
        }
        Rule::object => {
            let mut map = HashMap::new();
            let mut it = pair.into_inner().peekable();
            while let Some(next) = it.next() {
                if next.as_rule() == Rule::ident {
                    let key = next.as_str().to_string();
                    let val_pair = it.next().expect("value after key");
                    let val = parse_value(val_pair)?;
                    map.insert(key, val);
                }
            }
            Value::Object(map)
        }
        Rule::value => {
            let inner = pair.into_inner().next().unwrap();
            parse_value(inner)?
        }
        _ => unreachable!("unexpected rule: {:?}", pair.as_rule()),
    })
}
