use std::fs::File;
use std::io::Read;


fn main() {
    let mut f = File::open("./day12_input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let s = s;  //make s immutable


    let mut negative = false;
    let mut cur_number: isize = 0;
    let mut total: isize = 0;

    for c in s.chars() {
        let ascii_value = c as u8;

        // do things if the char is a number
        if (ascii_value >= 48) && (ascii_value < 58) {
            cur_number = (cur_number * 10) + ((ascii_value as isize) - 48);

        // do things if char is a dash
        } else if ascii_value == 45 {
            negative = true;

        // do stuff if char is not a dash or number
        } else {
            if cur_number > 0 {
                let mut tmp = cur_number as isize;
                if negative { tmp *= -1}
                total += tmp;
            }

            negative = false;
            cur_number = 0;
        }
    }
    println!("total: {}", total);
}
