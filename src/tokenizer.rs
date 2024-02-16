pub enum TokenType {
    Instruction,
    Register,
    Label,
    Immediate,
    LeftBracket,
    RightBracket,
    NewLine
}

pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str
}

impl Token<'_> {
    const INSTRUCTIONS: [&'static str; 2] = ["add", "addi"];

    pub fn tokenize(source: &str) -> Vec<Token> {
        
        let mut position: usize = 0;
        
        let mut tokens: Vec<Token> = Vec::new();


        while position < source.len() {
            let slice = &source[position..];
            
            let is_token = Self::get_token(slice);

            let token = match is_token.is_some() {
                true => is_token.unwrap(),
                false => { println!("Error somwhere (you problem now!)"); std::process::exit(1); }
            };

            tokens.push(token.0);
            position += token.1;

        }

        return tokens;

    }


    fn get_token(slice: &str) -> Option<(Token, usize)> {

        
        for i in Self::INSTRUCTIONS {

            if slice.starts_with( &(i.to_owned() + " ") ) {
                return Some((Token {
                    token_type: TokenType::Instruction, 
                    lexeme: &i[..i.len()-1]
                }, i.len()));
            }

        }


        return None;

    }
}


