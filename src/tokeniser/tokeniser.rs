use super::token_types::QuectoToken;

pub struct Tokeniser(pub String);

fn to_char(a : &mut u8) -> char
{
    char::from_u32(a.to_owned() as u32).unwrap()
}

impl Tokeniser
{
    pub fn tokenise(mut self) -> Vec<QuectoToken>
    {
        let mut tokens = Vec::new();
        // It's unsafe to write to, we aren't writing to anything therefore this is completely safe.
        let mut iterator = unsafe {self.0.as_mut_vec()}.into_iter();


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
        }

        return tokens;
    }
}

#[cfg(test)]
mod test
{
    use crate::tokeniser::token_types::QuectoToken;

    use super::Tokeniser;

    #[test]
    pub fn string_literal_test()
    {
        let subject = "\"Hello, World!\"";
        let subject = Tokeniser(subject.to_owned());
        let tokens = subject.tokenise();
        assert_eq!(QuectoToken::StringLiteral("Hello, World!".to_owned()), tokens[0])
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
}