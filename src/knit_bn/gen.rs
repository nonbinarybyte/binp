use crate::parser::ASTNode;

enum Instructions{
    CastOn = 0x01,
    BindOff = 0x02,
    Knit = 0x03,
    Purl = 0x04,
    YarnOver = 0x05,
    Repeat = 0xFF
}

struct Generator{}

impl Generator {
    pub fn generate(nodes: &[ASTNode]){
        for i in nodes{
            match i {
                ASTNode::CastOn(x) => {}
                ASTNode::BindOff => {}
                ASTNode::Knit(_) => {}
                ASTNode::Purl(_, _) => {}
                ASTNode::YarnOver => {}
                ASTNode::Repeat(_, _) => {}
            }
        }
    }
}