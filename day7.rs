use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("./day7_input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let s = s;  //make s immutable

    // create hash map with all the values. Keep looping over the gate list and check if a certain
    // gate can be "executed" in order to calculate the value of a new item.
    let mut wire_values: HashMap<&str, u16> = HashMap::new();


    process_gates(&s, &mut wire_values);

    let mut wire_values2: HashMap<&str, u16> = HashMap::new();
    wire_values2.insert("b", wire_values["a"]);
    process_gates(&s, &mut wire_values2);


    // print out all wire values
    for (k, v) in wire_values2.iter() {
        println!("Wire: {}\tvalue: {}", k, v);
    }
}

fn process_gates<'a>(input: &'a str, wire_values: &mut HashMap<&'a str, u16>) {
    let s = input.clone();
    // create list with all the gates
    let mut gates: Vec<Vec<&str>> = Vec::new();

    for line in s.split("\n") {
        if line == "" { continue }

        let l = line.split(" ");

        let mut line_items: Vec<&str> = Vec::new();
        for item in l {
            if item != "->" {
                line_items.push(item);
            }
        }
        gates.push(line_items);
    }


    let mut changed = true;

    while changed {
        changed = false;

        for gate in gates.iter_mut() {
            // skip if gate was already processed
            if wire_values.contains_key(gate[gate.len() - 1]) { continue; }


            if gate.len() == 2 {
                let new_val = get_val(gate[0], &wire_values);

                match new_val {
                    None => continue,
                    Some(v) => {
                        wire_values.insert(gate[1], v);
                        changed = true;
                    }
                }


            } else if gate.len() == 3 && wire_values.contains_key(gate[1]) {
                let new_val = get_val(gate[1], &wire_values);
                match new_val {
                    None => continue,
                    Some(v) => {
                        wire_values.insert(gate[2], !v);
                        changed = true;
                    }
                }


            } else if gate.len() == 4 {
                let first_val = match get_val(gate[0], &wire_values){
                    Some(v) => v,
                    None => continue,
                };
                let second_val = match get_val(gate[2], &wire_values){
                    Some(v) => v,
                    None => continue,
                };


                let res = match gate[1] {
                    "AND" => Some(first_val & second_val),
                    "OR" => Some(first_val | second_val),
                    "LSHIFT" => Some(first_val << second_val),
                    "RSHIFT" => Some(first_val >> second_val),
                    _ => None,
                };

                match res {
                    None => continue,
                    Some(v) => {
                        wire_values.insert(gate[3], v);
                        changed = true;
                    },
                };
            }
        }
    }
}



fn get_val(input: &str, wires: &HashMap<&str, u16>) -> Option<u16> {
    if let Ok(v) = input.parse::<u16>() {
        Some(v)
    } else {
        if wires.contains_key(input) {
            Some(wires[input])
        } else {
            None
        }
    }
}
