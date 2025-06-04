use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
    CastOn(i64),
    BindOff,
    Knit(String),
    Purl(String, String),
    YarnOver,
    Repeat(i64, Vec<ASTNode>),
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
                *i += 1;
                if let Token::Number(n) = &tokens[*i] {
                    *i += 1;
                    ASTNode::CastOn(*n)
                } else {
                    panic!("Expected number after cast_on");
                }
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
                    ASTNode::Repeat(*n, body)
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