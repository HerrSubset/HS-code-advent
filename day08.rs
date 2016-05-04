use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("./day8_input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let s = s;  //make s immutable


    let mut total_code_chars = 0;
    let mut total_mem_chars = 0;
    let mut total_new_chars = 0;

    for line in s.split("\n") {
        if line == "" { continue }
        let (code_chars, memory_chars, new_chars) = get_lengths(line);
        total_code_chars += code_chars;
        total_mem_chars += memory_chars;
        total_new_chars += new_chars;
    }

    println!("code: {}\tmemory: {}\tnew: {}", total_code_chars, total_mem_chars, total_new_chars);
    println!("difference: {}", total_code_chars - total_mem_chars);
    println!("difference2: {}", total_new_chars - total_code_chars);
}



fn get_lengths(line: &str) -> (i32, i32, i32) {
    let mut code_chars = 0;
    let mut memory_chars = 0;
    let mut new_representation_chars = 0;
    let mut escaping = true;
    let mut skip_turns = 0;

    for c in line.chars() {
        code_chars += 1;

        if (c == '\\') || (c == '"') {
            new_representation_chars += 2;
        } else {
            new_representation_chars += 1;
        }

        if skip_turns > 0 {
            skip_turns -= 1;
            continue;
        }

        if escaping {
            escaping = false;
            if escapable(c) {
                if c == 'x' {
                    skip_turns = 2;
                }
            } else {
                memory_chars += 1;
            }
        } else {
            memory_chars += 1;

            if c == '\\' { escaping = true }
        }
    }

    // don't count last double quote
    memory_chars -= 1;
    new_representation_chars += 2;

    println!("code: {}\tmem: {}\tnew: {}", code_chars, memory_chars, new_representation_chars);

    (code_chars, memory_chars, new_representation_chars)
}


fn escapable(c: char) -> bool {
    if c == '\\' { true }
    else if c == 'x' { true }
    else if c == '"' { true }
    else { false }
}
