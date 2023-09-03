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
    ) -> (Option<QuectoNode>, Peekable<IntoIter<QuectoToken>>)
    {
        let node = match token.next().unwrap()
        {
            crate::tokeniser::QuectoToken::IntLiteral(n) => Some(QuectoNode::IntLiteral(n.clone())),
            crate::tokeniser::QuectoToken::FloatLiteral(n) =>
            {
                Some(QuectoNode::FloatLiteral(n.clone()))
            }
            crate::tokeniser::QuectoToken::BoolLiteral(v) =>
            {
                Some(QuectoNode::Value(QuectoTypeContainer::Qbool(v.clone())))
            }
            crate::tokeniser::QuectoToken::CharLiteral(v) =>
            {
                Some(QuectoNode::Value(QuectoTypeContainer::Qchar(v.clone())))
            }
            crate::tokeniser::QuectoToken::StringLiteral(v) =>
            {
                Some(QuectoNode::Value(QuectoTypeContainer::Qstr(v.clone())))
            }
            crate::tokeniser::QuectoToken::Identifier(i) => match i.as_str()
            {
                "return" =>
                {
                    let (return_value, a) = Parser::parse_token(token);
                    token = a;
                    Some(QuectoNode::Return(Box::new(return_value.unwrap())))
                }
                "fn" =>
                {
                    assert_eq!(token.next().unwrap(), QuectoToken::Colon);

                    let func_type_tok = token.next().unwrap();
                    let func_type = if let QuectoToken::Type(t) = func_type_tok
                    {
                        t
                    }
                    else
                    {
                        panic!(
                            "Expected Type for Function Declaration, Got {:#?}",
                            func_type_tok
                        );
                    };

                    let name = if let QuectoToken::Identifier(name) = token.next().unwrap()
                    {
                        name
                    }
                    else
                    {
                        panic!("EXPECTED IDENTIFIER FOR FUNCTION NAME!")
                    };

                    assert_eq!(token.next(), Some(QuectoToken::OtherPunctuation('(')));

                    assert_eq!(token.next(), Some(QuectoToken::OtherPunctuation(')')));

                    let (scope, a) = Parser::parse_token(token);
                    token = a;

                    Some(QuectoNode::FunctionDeclaration(
                        func_type,
                        name,
                        Box::new(scope.unwrap()),
                    ))
                }
                identifier =>
                {
                    let left_brace = token.peek();
                    if let Some(left_brace) = left_brace
                    {
                        if let QuectoToken::OtherPunctuation(c) = left_brace
                        {
                            if *c == '('
                            {
                                // It is indeed a function call.
                                token.next();

                                // TODO: PARAMETERS

                                let right_brace = token.next().unwrap();
                                assert_eq!(right_brace, QuectoToken::OtherPunctuation(')'));

                                Some(QuectoNode::FunctionCall(identifier.to_string()))
                            }
                            else
                            {
                                None
                            }
                        }
                        else
                        {
                            None
                        }
                    }
                    else
                    {
                        None
                    }
                }
            },
            crate::tokeniser::QuectoToken::Colon => panic!("Unexpected Colon!"),
            crate::tokeniser::QuectoToken::SemiColon => None,
            crate::tokeniser::QuectoToken::OtherPunctuation(punc) => match punc
            {
                '{' =>
                {
                    let mut nodes = Vec::new();

                    while let Some(t) = token.peek()
                    {
                        if t == &QuectoToken::OtherPunctuation('}')
                        {
                            break;
                        }
                        let (a, b) = Parser::parse_token(token);
                        token = b;
                        if let Some(node) = a
                        {
                            nodes.push(node);
                        }
                    }

                    Some(QuectoNode::Scope(nodes))
                }
                char => panic!("Unexpected Punctuation: {char}"),
            },
            crate::tokeniser::QuectoToken::Type(t) => panic!("Unexpected Type {}", t),
            crate::tokeniser::QuectoToken::Operand(_) => panic!("NOT IMPLEMENTED"),
            crate::tokeniser::QuectoToken::Unknown => panic!(
                "Buddy, Somehow you got this type, it should be impossible, submit an issue."
            ),
        };

        (node, token)
    }

    pub fn parse(self) -> QuectoNode
    {
        let mut nodes = Vec::new();
        let mut tokens = self.0.tokenise().into_iter().peekable();

        while let Some(_) = tokens.peek()
        {
            let (node, token) = Self::parse_token(tokens);
            tokens = token;
            if let Some(node) = node
            {
                nodes.push(node);
            }

            tokens.next();
        }

        let program = QuectoNode::Module(ModuleType::Main, nodes);
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
