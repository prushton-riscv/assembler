use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
pub enum TokenType {
    Instruction,
    Register,
    Label,
    Immediate,
    LeftBracket,
    RightBracket
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String
}

impl Token {
    const INSTRUCTIONS: [&'static str; 31] = ["add", "addi", "sub", "subi", "mul", "muli", "or", "ori", "xor", "xori", "slt", "slti", "sgt", "sgti", "seq", "seqi", "addc", "addci", "mulc", "mulci", "lshift", "lshifti", "rshift", "rshifti", "jtl", "beq", "blt", "bgt", "lw", "sw", "simem"];
    

    pub fn tokenize(source: &str) -> Vec<Token> {
        
        let mut position: usize = 0;
        
        let mut tokens: Vec<Token> = Vec::new();


        while position < source.len() {
            let slice = &source[position..];
            
            //println!("[{}..]:{}", position, slice);
            let is_token = Self::get_token(slice);

            let token = match is_token.is_some() {
                true => is_token.unwrap(),
                false => { position += 1; continue; }
            };

            position += token.1;
            //println!("Type {:?}, {}", &token.0.token_type, &token.0.lexeme);
            tokens.push(token.0);
            //println!("Position incremented by {} to {}", &token.1, position);

        }

        return tokens;
    }
    

    fn get_token(slice: &str) -> Option<(Token, usize)> {

        let register_regex: Regex = Regex::new(r"^\$[a-z][0-9]*").unwrap();
        let number_regex: Regex = Regex::new(r"^(\-|)[0-9][0-9]*").unwrap();
        let label_regex: Regex = Regex::new(r"^[a-z|A-Z][a-z|A-Z]*").unwrap();
        for i in Self::INSTRUCTIONS {

            if slice.starts_with( &(i.to_owned() + " ") ) {
                return Some((Token {
                    token_type: TokenType::Instruction, 
                    lexeme: (&i[..i.len()]).to_string()
                }, i.len()));
            }

        }

        if register_regex.is_match(slice) {
            
            let mat = register_regex.find(slice).unwrap();
            return Some((Token {
                token_type: TokenType::Register,
                lexeme: mat.as_str().to_string()
            }, mat.as_str().len()));
        }

        if number_regex.is_match(slice) {
            let mat = number_regex.find(slice).unwrap();

            return Some((Token {
                token_type: TokenType::Immediate,
                lexeme: mat.as_str().to_string()
            }, mat.as_str().len()));
        }

        if slice.starts_with("[") {
            return Some((Token {
                token_type: TokenType::LeftBracket,
                lexeme: "[".to_string()
            }, 1));
        }
        if slice.starts_with("]") {
            return Some((Token {
                token_type: TokenType::RightBracket,
                lexeme: "]".to_string()
            }, 1));
        }

        if label_regex.is_match(slice) {
            let mat = label_regex.find(slice).unwrap();

            return Some((Token {
                token_type: TokenType::Label,
                lexeme: mat.as_str().to_string()
            }, mat.as_str().len()));
        }

        return None;

    }
}
