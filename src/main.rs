use std::env::args;
use std::fs;


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




    println!("---{}---", &input);
    println!("{}", &file);
}
