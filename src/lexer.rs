#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    CastOn,
    BindOff,
    Knit,
    Purl,
    YarnOver,
    Repeat,
    LBrace,
    RBrace,
    Equals,
    Identifier(String),
    Number(i64),
    StringLiteral(String),
    Unknown(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.split_whitespace().peekable();

    while let Some(word) = iter.next() {
        let token = match word {
            "cast_on" => Token::CastOn,
            "bind_off" => Token::BindOff,
            "K" => Token::Knit,
            "P" => Token::Purl,
            "YO" => Token::YarnOver,
            "repeat" => Token::Repeat,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            "=" => Token::Equals,
            w if w.parse::<i64>().is_ok() => Token::Number(w.parse().unwrap()),
            w if w.starts_with("\"") && w.ends_with("\"") => {
                Token::StringLiteral(w.trim_matches('"').to_string())
            }
            w => Token::Identifier(w.to_string()),
        };

        tokens.push(token);
    }

    tokens
}
