use crate::lexer::Token;
use crate::parser::ProgramError::{ExpectedAfterIdentifier, ExpectedIdentifier, NotInBounds, UnexpectedToken};

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

#[derive(Debug, Clone)]
pub enum ProgramError{
    ExpectedIdentifier(String),
    ExpectedAfterIdentifier(String),
    NotInBounds(String),
    UnexpectedToken(String),
}

pub fn parse(tokens: Vec<Token>) -> Vec<Result<ASTNode, ProgramError>> {
    let mut ast = Vec::new();
    let mut i = 0;

    fn parse_block(tokens: &[Token], i: &mut usize) -> Vec<Result<ASTNode, ProgramError>> {
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

    fn parse_stmt(tokens: &[Token], i: &mut usize) -> Result<ASTNode, ProgramError> {
        match &tokens[*i] {
            Token::CastOn => {
                Ok(ASTNode::CastOn)
            }
            Token::BindOff => {
                *i += 1;
                Ok(ASTNode::BindOff)
            }
            Token::Knit => {
                *i += 1;
                if let Token::Identifier(name) = &tokens[*i] {
                    *i += 1;
                    Ok(ASTNode::Knit(name.clone()))
                } else {
                    Err(ExpectedIdentifier("Expected identifier after \"knit\"".to_string()))
                }
            }
            Token::Purl => {
                purl_parser(tokens, i)
            }
            Token::YarnOver => {
                *i += 1;
                Ok(ASTNode::YarnOver)
            }
            Token::Repeat => {
                *i += 1;
                if let Token::Number(n) = &tokens[*i] {
                    *i += 1;
                    if tokens[*i] != Token::LBrace {
                        return Err(ExpectedIdentifier("Expected {{ after repeat".to_string()))
                    }
                    *i += 1;
                    let body = parse_block(tokens, i);

                    if let Ok(x) = u32::try_from(*n) {
                        let tmp_body: Result<Vec<_>, _> = body.into_iter().collect();
                        Ok(ASTNode::Repeat(x, tmp_body?))
                    }
                    else{
                        Err(NotInBounds("Repeat count has to be a positive number below 2^32".to_string()))
                    }
                } else {
                    Err(ExpectedIdentifier("Expected number after repeat".to_string()))
                }
            }
            _ => Err(UnexpectedToken("Token not expected".to_string())),
        }
    }

    while i < tokens.len() {
        ast.push(parse_stmt(&tokens, &mut i));
    }

    ast
}

fn purl_parser(tokens: &[Token], i: &mut usize) -> Result<ASTNode, ProgramError> {
    *i += 1;
    if let Token::Identifier(name) = &tokens[*i] {
        *i += 1;
        if tokens[*i] != Token::Equals {
            return Err(ExpectedIdentifier("Expected = after identifier".to_string()));
        }
        *i += 1;
        if let Token::StringLiteral(val) = &tokens[*i] {
            *i += 1;
            Ok(ASTNode::Purl(name.clone(), val.clone()))
        } else {
            Err(ExpectedAfterIdentifier("Expected string after identifier".to_string()))
        }
    } else {
        Err(ExpectedIdentifier("Expected identifier after \"purl".to_string()))
    }
}