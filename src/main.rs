mod lexer;
mod parser;
mod interpreter;
mod knit_bn;

use clap::{Parser as ClapParser};

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
    let ast = parser::parse(tokens);
    interpreter::run(ast);
}