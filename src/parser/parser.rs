use std::{iter::Peekable, str::FromStr, vec::IntoIter};

use crate::{tokeniser::{QuectoToken, Tokeniser}, shared::types::QuectoType};

use super::{
    node_types::{ModuleType, QuectoNode},
    QuectoValue,
};

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
        rsp : &mut usize
    ) -> (Option<QuectoNode>, Peekable<IntoIter<QuectoToken>>)
    {
        let node = match token.next().unwrap()
        {
            crate::tokeniser::QuectoToken::IntLiteral(n) =>
            {
                Some(QuectoNode::Value(QuectoValue::QWORD(n as u64), QuectoType::Qu64))
            }
            crate::tokeniser::QuectoToken::FloatLiteral(n) =>
            {
                Some(QuectoNode::Value(QuectoValue::QWORD(f64::to_bits(n)), QuectoType::Qf64))
            }
            crate::tokeniser::QuectoToken::BoolLiteral(v) =>
            {
                Some(QuectoNode::Value(QuectoValue::BYTE(v as u8), QuectoType::Qbool))
            }
            crate::tokeniser::QuectoToken::CharLiteral(v) =>
            {
                Some(QuectoNode::Value(QuectoValue::BYTE(v as u8), QuectoType::Qchar))
            }
            crate::tokeniser::QuectoToken::StringLiteral(_v) =>
            {
                todo!()
            }
            crate::tokeniser::QuectoToken::Identifier(i) => match i.as_str()
            {
                "return" =>
                {
                    let (return_value, a) = Parser::parse_token(token, rsp);
                    token = a;
                    Some(QuectoNode::Return(Box::new(return_value.unwrap())))
                }
                "let" =>
                {
                    assert_eq!(token.next().unwrap(), QuectoToken::Colon);
                    let variable_type = if let QuectoToken::Type(t) = token.next().unwrap()
                    {
                        t
                    }
                    else
                    {
                        panic!("Expected type for variable declaration")
                    };

                    let name = if let QuectoToken::Identifier(n) = token.next().unwrap()
                    {
                        n
                    }
                    else
                    {
                        panic!("Expected Name for variable declaration")
                    };

                    assert_eq!(token.next().unwrap(), QuectoToken::OtherPunctuation('='));

                    let result = Parser::parse_token(token, rsp);
                    token = result.1;
                    let (value, qtype) = if let QuectoNode::Value(v, t) = result.0.unwrap()
                    {
                        (v,t)
                    }
                    else
                    {
                        panic!("Expected Value for variable declaration")
                    };

                    *rsp += value.get_size_in_bytes();

                    Some(QuectoNode::VariableDeclaration(name, value, qtype, *rsp))
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

                    let (scope, a) = Parser::parse_token(token, rsp);
                    token = a;

                    if let QuectoNode::Scope(nodes, rsp_size) = scope.unwrap()
                    {
                        Some(QuectoNode::FunctionDeclaration(
                            func_type,
                            name,
                            nodes,
                            rsp_size
                        ))
                    } else
                    {
                        None
                    }
                }
                identifier =>
                {
                    let left_brace = token.peek();
                    let result = if let Some(left_brace) = left_brace
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
                    };

                    if result.is_none()
                    {
                        Some(QuectoNode::IdentifierReference(identifier.to_owned()))
                    }
                    else
                    {
                        result
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
                    let mut rsp_size = 0;

                    while let Some(t) = token.peek()
                    {
                        if t == &QuectoToken::OtherPunctuation('}')
                        {
                            break;
                        }
                        let (a, b) = Parser::parse_token(token, rsp);
                        token = b;
                        if let Some(node) = a
                        {
                            if let QuectoNode::VariableDeclaration(_, val, _, _) = node
                            {
                                rsp_size += val.get_size_in_bytes();
                            }
                            nodes.push(node);
                        }
                    }

                    Some(QuectoNode::Scope(nodes, rsp_size))
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
        let mut rsp = 0;

        while let Some(_) = tokens.peek()   
        {
            let (node, token) = Self::parse_token(tokens, &mut rsp);
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

    use crate::{parser::{
        node_types::{ModuleType, QuectoNode},
        QuectoValue,
    }, shared::types::QuectoType};

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
                vec![QuectoNode::Return(Box::new(QuectoNode::Value(
                    QuectoValue::QWORD(0),
                    QuectoType::Qu64
                )))]
            )
        );
    }
}
