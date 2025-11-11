use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Parser as ClapParser, Subcommand, ValueEnum};
use miniconf_parser::{Document, Value};

/// Command-line interface for the MiniConf parser.
#[derive(Debug, ClapParser)]
#[command(
    name = "miniconf-parser",
    about = "Parse and validate MiniConf files",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Parse a file and print the resulting document.
    Parse(ParseArgs),
    /// Validate a file without printing its contents.
    Check(CheckArgs),
}

#[derive(Debug, Args)]
struct ParseArgs {
    /// Path to the MiniConf file.
    input: PathBuf,
    /// Optional output path. Writes to STDOUT when omitted.
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Selects how the parsed document is rendered.
    #[arg(short = 'f', long = "format", value_enum, default_value_t = OutputFormat::Pretty)]
    format: OutputFormat,
}

#[derive(Debug, Args)]
struct CheckArgs {
    /// Path to the MiniConf file.
    input: PathBuf,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Pretty,
    Json,
}

impl Cli {
    /// Executes the CLI command selected by the user.
    pub fn run(self) -> Result<()> {
        match self.command {
            Command::Parse(args) => run_parse(args),
            Command::Check(args) => run_check(args),
        }
    }
}

fn run_parse(args: ParseArgs) -> Result<()> {
    let contents = read_file(&args.input)?;
    let document = miniconf_parser::parse_str(&contents)?;
    let rendered = match args.format {
        OutputFormat::Pretty => render_pretty(&document),
        OutputFormat::Json => serde_json::to_string_pretty(&document)?,
    };

    if let Some(path) = args.output {
        fs::write(&path, rendered)
            .with_context(|| format!("failed to write {}", path.display()))?;
    } else {
        println!("{rendered}");
    }
    Ok(())
}

fn run_check(args: CheckArgs) -> Result<()> {
    let contents = read_file(&args.input)?;
    miniconf_parser::parse_str(&contents)?;
    println!("✓ {} is valid", args.input.display());
    Ok(())
}

fn read_file(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))
}

fn render_pretty(document: &Document) -> String {
    let mut out = String::new();
    for (name, section) in document.sections() {
        if name != "root" {
            out.push_str(&format!("[{name}]\n"));
        }
        for entry in &section.entries {
            out.push_str(&format!("{} = {}\n", entry.key, format_value(&entry.value)));
        }
        if !section.entries.is_empty() {
            out.push('\n');
        }
    }
    out.trim_end().to_string()
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            if n.fract() == 0.0 {
                format!("{:.0}", n)
            } else {
                n.to_string()
            }
        }
        Value::Bool(b) => b.to_string(),
        Value::Array(values) => {
            let items: Vec<String> = values.iter().map(format_value).collect();
            format!("[{}]", items.join(", "))
        }
    }
}
