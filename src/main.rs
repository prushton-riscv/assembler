use std::env::args;
use std::fs;
use std::collections::LinkedList;
use std::collections::HashMap;

mod tokenizer;
mod parser;

use crate::parser::{Parsed};
use crate::tokenizer::{Token, TokenType};


fn main() {
    let input: String = match args().nth(1).is_none()  {
        true => {println!("No input file supplied"); return;},
        false => args().nth(1).unwrap(),
    };

    let file_obj = fs::read_to_string(&input);
    let file: String = match file_obj.is_ok() {
        true => file_obj.unwrap(),
        false => {println!("Invalid File"); return;}
    };

    let file_iter = file.split("\n");
    let mut file_lines = LinkedList::new();
    let mut labels: HashMap<&str, u64> = HashMap::new();

    let mut i: u64 = 0;
    for line in file_iter { //parse out the labels, blank lines, and comment lines
        if line == "" || line.starts_with(";"){
            continue;
        }
        if line.ends_with(":") {
            let label = &line[..line.len()-1];
            
            labels.insert(label, i);
            continue;
        }   
        file_lines.push_back(&line[1..]);
        i += 1;
    }
 
    


    for line in file_lines {
        println!("Line: {}\n    Tokens:", &line);
        let tokens = Token::tokenize(line);
        for token in &tokens {
            println!("      {:?}: {:?}; ", token.token_type, token.lexeme);
        }
        println!("  Parsed Tokens:");
        let parsed_tokens = Parsed::parse_tokens(&tokens, &labels);
        for token in &parsed_tokens {
            println!("      {:?}: {:?}; ", token.token_type, token.lexeme);
        }
        println!("  Binary");
        let binary = Parsed::from_parsed_tokens(&parsed_tokens);
        println!("      {}", binary.binary);
    }

    println!("Label Addresses");
    for line in labels {
        println!("  {:?}", line);
    }
}
