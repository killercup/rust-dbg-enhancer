use from_pest::FromPest;
use parser::RustDebug;
use pest::Parser;
use snafu::Snafu;

pub fn enhance(input: &str) -> Result<String, Error> {
    let mut parse_tree =
        RustDebug::parse(parser::Rule::source, input).map_err(|e| Error::ParseGrammar {
            description: format!("{}", e),
        })?;
    let syntax_tree =
        ast::Source::from_pest(&mut parse_tree).map_err(|e| Error::AstGeneration {
            description: format!("{:?}", e),
        })?;

    Ok(syntax_tree.to_string())
}

pub(crate) mod parser {
    #[derive(Debug, Clone, pest_derive::Parser)]
    #[grammar = "rust_debug.pest"]
    pub struct RustDebug;
}

pub(crate) mod ast;
pub(crate) mod display;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not parse grammar: {}", description))]
    ParseGrammar { description: String },
    #[snafu(display("ast generation failed: {}", description))]
    AstGeneration { description: String },
}
