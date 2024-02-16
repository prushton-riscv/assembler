use crate::tokenizer::{TokenType, Token};
use regex::Regex;
use std::collections::HashMap;
use phf::phf_map;

static OPCODES: phf::Map<&'static str, &'static str> = phf_map!{
    "add" => "0000_0000_0000",
    "sub" => "0000_0000_0001",
    "mul" => "0000_0000_0010",
    "and" => "0000_0000_0011",
    "or"  => "0000_0000_0100",
    "xor" => "0000_0000_0101",
    "slt" => "0000_0000_0110",
    "sgt" => "0000_0000_0111",
    "seq" => "0000_0000_1000",
    "addc"   => "0000_0000_1001",
    "mulc"   => "0000_0000_1010",
    "lshift" => "0000_0000_1011",
    "rshift" => "0000_0000_1100",

    "addi" => "0001_0000_0000",
    "subi" => "0001_0000_0001",
    "muli" => "0001_0000_0010",
    "andi" => "0001_0000_0011",
    "ori"  => "0001_0000_0100",
    "xori" => "0001_0000_0101",
    "slti" => "0001_0000_0110",
    "sgti" => "0001_0000_0111",
    "seqi" => "0001_0000_1000",
    "addci"   => "0001_0000_1001",
    "mulci"   => "0001_0000_1010",
    "lshifti" => "0001_0000_1011",
    "rshifti" => "0001_0000_1100",

    "jtl" => "0001_0010_0000",
    "beq" => "0001_0010_0001",
    "blt" => "0001_0010_0010",
    "bgt" => "0001_0010_0011",

    "sw" => "0010_0000_0000",
    "simem" => "0010_0000_0001",

    "lw" => "0011_0000_0000",
};

pub struct Parsed {
    pub binary: String,
}

impl Parsed {
    
    const REGISTER_ALPHABET: [&'static str; 4] = ["c", "a", "v", "t"];
    
    pub fn parse_tokens<'a>(tokens: &'a Vec<Token>, labels: &'a HashMap<&'a str, u64>) -> Vec<Token> {
        let mut parsed_tokens: Vec<Token> = Vec::new();

        for i in 0..tokens.len() {
            let token = &tokens[i];

            match token.token_type {
                TokenType::Label => {
                    let label_string = token.lexeme.clone();
                    let label = labels.get(label_string.as_str());
                    
                    if label.is_none() {
                        println!("Invalid Label {} at line {}", token.lexeme, i);
                        std::process::exit(1);
                    }

                    parsed_tokens.push(Token {
                        token_type: TokenType::Immediate,
                        lexeme: label.unwrap().to_string()
                    })
                },
                TokenType::Register => {
                    let letter = Regex::new("[a-z]").unwrap().find(&token.lexeme).unwrap().as_str();
                    let number = Regex::new("[0-9][0-9]*").unwrap().find(&token.lexeme).unwrap().as_str();
                    let letter_offset = Self::REGISTER_ALPHABET.iter().position(|&x| x == letter);

                    if letter_offset.is_none() {
                        println!("Invalid Letter {}, expected one of c, a, v, or t", letter); std::process::exit(1); }
                    
                    let register = (letter_offset.unwrap() as u64 * 32) + number.parse::<u64>().unwrap();
                    //println!("Register {} converted to {}", &token.lexeme, register);
                    parsed_tokens.push(Token {token_type: TokenType::Immediate, lexeme: register.to_string() });

                },
                _ => {
                    parsed_tokens.push(token.clone());
                }
            }
        }

        
        return parsed_tokens;

    }

    pub fn from_parsed_tokens(parsed_tokens: &Vec<Token>) -> Self {
        let opcode: &str = OPCODES.get(&parsed_tokens[0].lexeme).unwrap();
        let mut full_binary: String = opcode.to_string().clone();
        

        full_binary.replace_range(4..5, "");
        full_binary.replace_range(8..9, "");
        
        match &opcode[0..4] {
            "0000" => {
                for i in 1..4 {
                    let bin_arg = format!("{:010b}", parsed_tokens[i].lexeme.parse::<i64>().unwrap());
                    full_binary.insert_str(0, bin_arg.as_str());
                }
                full_binary.insert_str(0, "0000000000000000000000");
            },
            "0001" => {
                for i in 1..3 {
                    let bin_arg = format!("{:010b}", parsed_tokens[i].lexeme.parse::<i64>().unwrap());
                    full_binary.insert_str(0, bin_arg.as_str());
                }
                let bin_imm = format!("{:032b}", parsed_tokens[3].lexeme.parse::<i64>().unwrap());
                full_binary.insert_str(0, &bin_imm);
            },
            "0010" => {
                full_binary.insert_str(0, "0000000000");
                let offset = format!("{:022b}", parsed_tokens[3].lexeme.parse::<i64>().unwrap());
                let ma = format!("{:010b}", parsed_tokens[1].lexeme.parse::<i64>().unwrap());
                let rs = format!("{:010b}", parsed_tokens[5].lexeme.parse::<i64>().unwrap());
                
                full_binary.insert_str(0, ma.as_str());
                full_binary.insert_str(0, rs.as_str());
                full_binary.insert_str(0, offset.as_str());
            },
            "0011" => {
                let offset = format!("{:022b}", parsed_tokens[4].lexeme.parse::<i64>().unwrap());
                let ma = format!("{:010b}", parsed_tokens[2].lexeme.parse::<i64>().unwrap());
                let rd = format!("{:010b}", parsed_tokens[1].lexeme.parse::<i64>().unwrap());
                
                full_binary.insert_str(0, rd.as_str());
                full_binary.insert_str(0, ma.as_str());
                full_binary.insert_str(0, "0000000000");
                full_binary.insert_str(0, offset.as_str());
            },
            _ => {println!("Unrecognized opcode {}", opcode); std::process::exit(1)}
        }
        

        return Self {binary: full_binary.clone()}
    }
}
