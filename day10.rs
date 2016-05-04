fn main() {
    let mut sequence: Vec<i32> = vec![3,1,1,3,3,2,2,1,1,3];

    for _i in 0..50 {
        let mut tmp: Vec<i32> = Vec::new();
        let mut cur_number = sequence[0];
        let mut count = 0;

        for n in &sequence {
            if cur_number == *n {
                count += 1;
            } else {
                tmp.push(count);
                tmp.push(cur_number);
                count = 1;
                cur_number = *n;
            }
        }
        tmp.push(count);
        tmp.push(cur_number);

        sequence = tmp;
    }


    println!("result size: {}", sequence.len());
}
