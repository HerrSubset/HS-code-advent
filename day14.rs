use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("./day14_input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let s = s;  // make s immutable

    let mut reindeer: Vec<Reindeer> = Vec::new();

    for line in s.split("\n") {
        if line == "" {
            continue;
        }
        let mut words = line.split(" ");

        let speed = words.nth(3).unwrap().parse::<usize>().unwrap();
        let flytime = words.nth(2).unwrap().parse::<usize>().unwrap();
        let resttime = words.nth(6).unwrap().parse::<usize>().unwrap();

        reindeer.push(Reindeer::new(resttime, flytime, speed));
    }



    for _i in 0..2503 {
        for r in &mut reindeer {
            r.go();
        }

        let leading_distance = get_leading_distance(&reindeer);

        for r in &mut reindeer {
            if r.distance == leading_distance {
                r.score();
            }
        }
    }


    for r in &reindeer {
        r.print();
    }
}


fn get_leading_distance(reindeer: &Vec<Reindeer>) -> usize {
    let mut res = 0;

    for r in reindeer {
        if r.distance > res {
            res = r.distance;
        }
    }

    res
}




struct Reindeer {
    distance: usize,
    speed: usize,
    rest_time: usize,
    fly_time: usize,
    counter: usize,
    flying: bool,
    score: usize,
}


impl Reindeer {
    // constructor
    fn new(rt: usize, ft: usize, s: usize) -> Self {
        Reindeer {  distance: 0,
                    speed: s,
                    rest_time: rt,
                    fly_time: ft,
                    counter: ft,
                    flying: true,
                    score: 0}
    }

    // proceed by one second
    fn go(&mut self) {
        if self.flying {
            self.distance += self.speed;
        }
        self.counter -= 1;

        if self.counter == 0 {
            self.reset_counter();
        }
    }

    // reset counter after switching mode. New counter value depends on whether the reindeer
    // was resting or not
    fn reset_counter(&mut self) {
        if self.flying {
            self.flying = false;
            self.counter = self.rest_time;
        } else {
            self.flying = true;
            self.counter = self.fly_time;
        }
    }

    // print reindeer info to console
    fn print(&self) {
        println!("distance:\t{}\nscore:\t{}\nspeed\t{}\nflying:\t{}\ncounter:\t{}\n", self.distance, self.score, self.speed, self.flying, self.counter);
    }

    // give the reindeer a point
    fn score(&mut self) {
        self.score += 1;
    }
}
