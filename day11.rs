fn main() {
    let mut password = Password::new("hxbxxyzz");

    loop {
        password.increment();
        if password.is_valid() {
            println!("{}", password.to_string());
            break;
        }
    }

    // println!("{}", password.to_string());
    // for n in &password.passw {
    //     println!("{}", n);
    // }
    // password.increment();
    // println!("{}", password.to_string());
}



struct Password {
    passw: Vec<u8>,
}


impl Password {
    fn new(s: &str) -> Self {
        let mut p: Vec<u8> = Vec::new();

        for c in s.chars() {
            p.push(c as u8);
        }

        Password { passw: p }
    }


    fn to_string(&self) -> String {
        let mut res = String::new();

        for n in &self.passw {
            res.push(*n as char);
        }

        return res;
    }


    fn increment(&mut self) {
        let length = self.passw.len() - 1;
        self.increment_index(length);
    }

    fn increment_index(&mut self, index: usize) {
        self.passw[index] += 1;

        if self.passw[index] > 122 {
            self.passw[index] = 97;
            if index > 0 {
                self.increment_index(index - 1);
            }
        }
    }


    fn is_valid(&self) -> bool {
        let mut res = true;
        if !self.has_straight() {
            res = false
        } else if !self.no_forbidden_letters() {
            res = false;
        } else if !self.has_two_pairs() {
            res = false;
        }
        // no "o", "i" or "l"
        // password has 2 pairs of non-overlapping letters
        res
    }

    fn has_straight(&self) -> bool{
        let mut res = false;

        for n in 0..(self.passw.len() - 2) {
            let cur_n = self.passw[n];

            if (cur_n + 1 == self.passw[n+1]) && (cur_n + 2 == self.passw[n+2]) {
                res = true;
                break;
            }
        }

        res
    }

    fn no_forbidden_letters(&self) -> bool {
        let mut res = true;

        for n in &self.passw {
            if *n == 'i' as u8 {
                res = false;
            } else if *n == 'o' as u8 {
                res = false;
            } else if *n == 'l' as u8 {
                res = false;
            }

            if !res { break; }
        }

        res
    }

    fn has_two_pairs(&self) -> bool {
        let mut skipping = false;
        let mut number_of_pairs = 0;

        for n in 0..(self.passw.len() - 1) {
            if skipping {
                skipping = false;
                continue;
            }

            if self.passw[n] == self.passw[n+1] {
                number_of_pairs += 1;
                skipping = true;
            }
        }

        number_of_pairs >= 2
    }
}
