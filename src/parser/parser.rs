use std::{iter::Peekable, str::FromStr, vec::IntoIter};

use crate::{
    shared::types::QuectoTypeContainer,
    tokeniser::{QuectoToken, Tokeniser},
};

use super::node_types::{ModuleType, QuectoNode};

#[derive(Clone)]
pub struct Parser(pub Tokeniser);

impl FromStr for Parser
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let tokens = Tokeniser(s.to_string());
        Ok(Parser(tokens))
    }
}

impl Parser
{
    fn parse_token(
        mut token: Peekable<IntoIter<QuectoToken>>,
        depth: Option<usize>,
    ) -> (QuectoNode, Peekable<IntoIter<QuectoToken>>, usize)
    {
        let mut depth = depth.unwrap_or(0);
        let node = match token.nth(depth).unwrap()
        {
            crate::tokeniser::QuectoToken::IntLiteral(n) => QuectoNode::IntLiteral(n.clone()),
            crate::tokeniser::QuectoToken::FloatLiteral(n) => QuectoNode::FloatLiteral(n.clone()),
            crate::tokeniser::QuectoToken::BoolLiteral(v) =>
            {
                QuectoNode::Value(QuectoTypeContainer::Qbool(v.clone()))
            }
            crate::tokeniser::QuectoToken::CharLiteral(v) =>
            {
                QuectoNode::Value(QuectoTypeContainer::Qchar(v.clone()))
            }
            crate::tokeniser::QuectoToken::StringLiteral(v) =>
            {
                QuectoNode::Value(QuectoTypeContainer::Qstr(v.clone()))
            }
            crate::tokeniser::QuectoToken::Identifier(i) => match i.as_str()
            {
                "return" =>
                {
                    let (return_value, a, new_depth) = Parser::parse_token(token, Some(depth));
                    token = a;
                    depth = new_depth;
                    QuectoNode::Return(Box::new(return_value))
                }
                _ => panic!(),
            },
            crate::tokeniser::QuectoToken::Colon => todo!(),
            crate::tokeniser::QuectoToken::SemiColon => todo!(),
            crate::tokeniser::QuectoToken::OtherPunctuation(_) => todo!(),
            crate::tokeniser::QuectoToken::Type(_) => todo!(),
            crate::tokeniser::QuectoToken::Operand(_) => todo!(),
            crate::tokeniser::QuectoToken::Unknown => todo!(),
        };

        (node, token, depth + 1)
    }

    pub fn parse(self) -> QuectoNode
    {
        let mut program = QuectoNode::Module(ModuleType::Main, Vec::new());
        let mut tokens = self.0.tokenise().into_iter().peekable();

        while let Some(_) = tokens.peek()
        {
            let (node, token, advance_by) = Self::parse_token(tokens, None);
            tokens = token;
            tokens.nth(advance_by);
            if let QuectoNode::Module(t, mut nodes) = program
            {
                nodes.push(node);
                program = QuectoNode::Module(t, nodes);
            }

            tokens.next();
        }

        program
    }
}

#[cfg(test)]
mod test
{
    use std::str::FromStr;

    use crate::parser::node_types::{ModuleType, QuectoNode};

    use super::Parser;

    #[test]
    fn return_statement()
    {
        let parser = Parser::from_str("return 0;").unwrap();
        let nodes = parser.parse();
        assert_eq!(
            nodes,
            QuectoNode::Module(
                ModuleType::Main,
                vec![QuectoNode::Return(Box::new(QuectoNode::IntLiteral(0)))]
            )
        );
    }
}
