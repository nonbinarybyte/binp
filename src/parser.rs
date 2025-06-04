use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
    /// Start/init block
    CastOn,

    /// Return/end
    BindOff,

    ///
    Knit(String),

    /// Variable declaration
    Purl(String, String),

    /// Increment/ loop init
    YarnOver,

    /// Loop n times
    Repeat(u32, Vec<ASTNode>),
}

pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut i = 0;

    fn parse_block(tokens: &[Token], i: &mut usize) -> Vec<ASTNode> {
        let mut block = Vec::new();
        while *i < tokens.len() {
            match &tokens[*i] {
                Token::RBrace => {
                    *i += 1;
                    break;
                }
                _ => block.push(parse_stmt(tokens, i)),
            }
        }
        block
    }

    fn parse_stmt(tokens: &[Token], i: &mut usize) -> ASTNode {
        match &tokens[*i] {
            Token::CastOn => {
               ASTNode::CastOn
            }
            Token::BindOff => {
                *i += 1;
                ASTNode::BindOff
            }
            Token::Knit => {
                *i += 1;
                if let Token::Identifier(name) = &tokens[*i] {
                    *i += 1;
                    ASTNode::Knit(name.clone())
                } else {
                    panic!("Expected identifier after K");
                }
            }
            Token::Purl => {
                *i += 1;
                if let Token::Identifier(name) = &tokens[*i] {
                    *i += 1;
                    if tokens[*i] != Token::Equals {
                        panic!("Expected = after identifier");
                    }
                    *i += 1;
                    if let Token::StringLiteral(val) = &tokens[*i] {
                        *i += 1;
                        ASTNode::Purl(name.clone(), val.clone())
                    } else {
                        panic!("Expected string after =");
                    }
                } else {
                    panic!("Expected identifier after P");
                }
            }
            Token::YarnOver => {
                *i += 1;
                ASTNode::YarnOver
            }
            Token::Repeat => {
                *i += 1;
                if let Token::Number(n) = &tokens[*i] {
                    *i += 1;
                    if tokens[*i] != Token::LBrace {
                        panic!("Expected '{{' after repeat");
                    }
                    *i += 1;
                    let body = parse_block(tokens, i);

                    if let Ok(x) = u32::try_from(*n) {
                        ASTNode::Repeat(x, body)
                    }
                    else{
                        panic!("Repeat count has to be a positive number below 2^32")
                    }
                } else {
                    panic!("Expected number after repeat");
                }
            }
            _ => panic!("Unexpected token: {:?}", tokens[*i]),
        }
    }

    while i < tokens.len() {
        ast.push(parse_stmt(&tokens, &mut i));
    }

    ast
}