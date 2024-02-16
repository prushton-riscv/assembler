use phf::{phf_map};

pub struct Instruction<'a> {
    instruction: &'a str,
    type: &'a str,
    rd: &'a str,
    rs1: &'a str,
    rs2: &'a str,
    imm: &'a str,
    offset: &'a str,
}

struct Parser<'a> {
    regex: &'a str, // "(.*) "
    type: &'a str
}

static instruction_format = phf::Map<&'static str, &'static str> = phf_map! {
    "(add|sub|mul|and|or|xor|slt|sgt|seq|addc|mulc|lshift|rshift)" <= Parser {regex: "", type: "R"}
}


impl Instruction {
    pub fn from_str(asm: &str) {
                      
    }
}
