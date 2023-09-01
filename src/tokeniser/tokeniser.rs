use std::str::FromStr;

use crate::shared::types::QuectoType;

use super::token_types::QuectoToken;

pub struct Tokeniser(pub String);

fn to_char(a: &mut u8) -> char
{
    char::from_u32(a.to_owned() as u32).unwrap()
}

impl Tokeniser
{
    pub fn tokenise(mut self) -> Vec<QuectoToken>
    {
        let mut tokens = Vec::new();
        // It's unsafe to write to, we aren't writing to anything therefore this is completely safe.
        let mut iterator = unsafe { self.0.as_mut_vec() }.into_iter().peekable();

        'tokeniser_loop: while let Some(character) = iterator.next()
        {
            let character = char::from_u32(character.clone() as u32).unwrap();

            if character == '"'
            {
                let mut str_buff = String::new();

                while let Some(character) = iterator.next()
                {
                    let character = char::from_u32(character.clone() as u32).unwrap();
                    if character == '"'
                    {
                        break;
                    }

                    str_buff.push(character);
                }

                tokens.push(QuectoToken::StringLiteral(str_buff));
                continue 'tokeniser_loop;
            }

            if character == '\''
            {
                // Fail if there are no tokens to consume, we don't want broken tokens in the output.
                tokens.push(QuectoToken::CharLiteral(to_char(iterator.next().unwrap())));
                assert_eq!('\'', to_char(iterator.next().unwrap()));
                continue 'tokeniser_loop;
            }

            if character.is_alphabetic()
            {
                let mut str_buff = String::from(character);
                while let Some(chara) = iterator.peek()
                {
                    let chara = char::from_u32(**chara as u32).unwrap();
                    if !chara.is_alphabetic() {break;}
                    let chara = iterator.next().unwrap();
                    let chara = char::from_u32(*chara as u32).unwrap();

                    str_buff.push(chara);
                }

                if let Ok(type_name) = QuectoType::from_str(&str_buff)
                {
                    tokens.push(QuectoToken::Type(type_name));
                    continue 'tokeniser_loop;
                }
                else if str_buff == "true" || str_buff == "false"
                {
                    tokens.push(QuectoToken::BoolLiteral(str_buff == "true"));
                    continue 'tokeniser_loop;
                }

                tokens.push(QuectoToken::Identifier(str_buff));
                continue 'tokeniser_loop;
            }
        }

        return tokens;
    }
}

#[cfg(test)]
mod test
{
    use std::collections::HashMap;

    use crate::{
        shared::types::{QuectoOperand, QuectoType},
        tokeniser::token_types::QuectoToken,
    };

    use super::Tokeniser;

    #[test]
    pub fn string_literal_test()
    {
        let subject = "\"Hello, World!\"";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(
            QuectoToken::StringLiteral("Hello, World!".to_owned()),
            tokens[0]
        )
    }

    #[test]
    pub fn char_literal_test()
    {
        let subject = "\'c\'";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::CharLiteral('c'), tokens[0])
    }

    #[test]
    #[should_panic]
    pub fn char_literal_fail_test()
    {
        let subject = "\'c";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::CharLiteral('c'), tokens[0])
    }

    #[test]
    pub fn int_literal_test()
    {
        let subject = "10";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::IntLiteral(10), tokens[0])
    }

    #[test]
    pub fn float_literal_test()
    {
        let subject = "10.5";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::FloatLiteral(10.5), tokens[0]);
        let subject = "167.575";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::FloatLiteral(167.575), tokens[0])
    }

    #[test]
    pub fn bool_literal_test()
    {
        let subject = "true";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::BoolLiteral(true), tokens[0]);
        let subject = "false";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::BoolLiteral(false), tokens[0])
    }

    #[test]
    pub fn identifier_test()
    {
        let subject = "variable_name";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(
            QuectoToken::Identifier("variable_name".to_owned()),
            tokens[0]
        )
    }

    #[test]
    pub fn colon_semi_test()
    {
        let subject = ";:";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::SemiColon, tokens[0]);
        assert_eq!(QuectoToken::Colon, tokens[1])
    }

    #[test]
    pub fn punc_test()
    {
        let subject = ".(";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::OtherPunctuation('.'), tokens[0]);
        assert_eq!(QuectoToken::OtherPunctuation('('), tokens[1])
    }

    #[test]
    pub fn type_test()
    {
        let mut map = HashMap::new();

        map.insert("u8", QuectoType::Qu8);
        map.insert("u16", QuectoType::Qu16);
        map.insert("u32", QuectoType::Qu32);
        map.insert("u64", QuectoType::Qu64);

        map.insert("i8", QuectoType::Qi8);
        map.insert("i16", QuectoType::Qi16);
        map.insert("i32", QuectoType::Qi32);
        map.insert("i64", QuectoType::Qi64);

        map.insert("f32", QuectoType::Qf32);
        map.insert("f64", QuectoType::Qf64);

        map.insert("bool", QuectoType::Qbool);
        map.insert("char", QuectoType::Qchar);
        map.insert("str", QuectoType::Qstr);

        let keys = map.keys().into_iter();

        for k in keys
        {
            let subject = k.to_string();
            let subject = Tokeniser(subject);
            let tokens = subject.tokenise();
            assert_eq!(QuectoToken::Type(map[k]), tokens[0]);
        }
    }

    #[test]
    pub fn operand_test()
    {
        let mut types = Vec::new();
        types.push(QuectoOperand::Add);
        types.push(QuectoOperand::Subtract);
        types.push(QuectoOperand::Divide);
        types.push(QuectoOperand::Multiply);
        types.push(QuectoOperand::Modulus);
        types.push(QuectoOperand::Or);
        types.push(QuectoOperand::And);
        types.push(QuectoOperand::Not);

        for k in types
        {
            let subject = k.to_string();
            let subject = Tokeniser(subject);
            let tokens = subject.tokenise();
            assert_eq!(QuectoToken::Operand(k), tokens[0]);
        }
    }
}
