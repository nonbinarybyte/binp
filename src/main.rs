mod lexer;
mod parser;
mod interpreter;
mod knit_bn;
mod test;

use clap::{Parser as ClapParser};
use crate::parser::ASTNode;

#[derive(ClapParser)]
#[command(name = "knitlang")]
#[command(about = "A knitting-inspired programming language 🧶", long_about = None)]
struct Cli {
    #[arg()]
    file: String,
}

fn main() {
    let cli = Cli::parse();

    let source = std::fs::read_to_string(&cli.file)
        .unwrap_or_else(|_| panic!("Could not read file: {}", &cli.file));

    let tokens = lexer::tokenize(&source);
    let ast: Vec<ASTNode> = parser::parse(tokens)
        .iter()
        .map(|x| x.clone().unwrap())
        .collect();
    interpreter::run(ast);
}