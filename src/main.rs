use std::env::args;
use std::fs;
use std::collections::LinkedList;
use std::collections::HashMap;


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
    let mut labels: HashMap<&str, u32> = HashMap::new();

    let mut i: u32 = 0;
    for line in file_iter { //parse out the labels, blank lines, and comment lines
        if line == "" || line.starts_with(";"){
            continue;
        }
        if line.ends_with(":") {
            labels.insert(line, i);
            continue;
        }   
        file_lines.push_back(&line[1..]);
        i += 1;
    }
 
    replace_jump_labels(&mut file_lines, &labels);

    for line in file_lines {
        println!("{}", line);
    }

    for line in labels {
        println!("{:?}", line);
    }
}



fn replace_jump_labels<'a>(file_lines: &mut LinkedList<&str>, labels: &HashMap<&'a str, u32>) {

    for line in file_lines {
        
    }
}
