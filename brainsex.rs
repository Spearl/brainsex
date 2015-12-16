use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(err) => panic!("Unable to open file: {}", err),
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => {},
        Err(err) => panic!("Unable to read file: {}", err),
    };
    let valid_chars = "+-><[],.";
    let code: Vec<char> = buffer.chars()
                                .filter(|&c| valid_chars.contains(c))
                                .collect();
    
    let mut loop_stack: Vec<usize> = vec![];
    let mut loop_map = HashMap::new();
    for (i, &c) in code.iter().enumerate() {
        if c == '[' {
            loop_stack.push(i);
        } else if c == ']' {
            let entrance = match loop_stack.pop() {
                Some(e) => e,
                None => panic!("Mismatched loop"),
            };
            loop_map.insert(entrance, i);
            loop_map.insert(i, entrance);
        }
    }


    let s: String = code.into_iter().collect();
    println!("{}", s);
}
