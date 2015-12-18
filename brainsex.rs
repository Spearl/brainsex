use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::str;

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

    let mut data: [u8; 30000] = [0; 30000];
    let mut ip = 0;
    let mut dp = 0;
    while ip < code.len() {
        match code[ip] {
            '>' => dp += 1,
            '<' => dp -= 1,
            '+' => data[dp] += 1,
            '-' => data[dp] -= 1,
            '.' => print!("{}", str::from_utf8(&[data[dp]]).unwrap()),
            ',' => data[dp] = std::io::stdin().bytes().next().and_then(|result| result.ok()).unwrap(),
            '[' if data[dp] == 0 => ip = match loop_map.get(&ip) {
                Some(&new_ip) => new_ip,
                _ => panic!(),
            },
            ']' if data[dp] != 0 => ip = match loop_map.get(&ip) {
                Some(&new_ip) => new_ip,
                _ => panic!(),
            },
            _ => {},
        }
        ip += 1;
    }
}
