fn main() {
    let input = 15000;
    let mut house = 1;

    loop {
        let presents = house_presents(house);

        if presents >= input {
            break
        } else {
            house += 1;
        }
    }

    println!("house: {}", house);
}


fn house_presents(number: usize) -> usize {
    let mut total = 0;

    for i in 1..(number + 1) {
        if number % i == 0 {
            total += i * 10;
        }
    }

    return total;
}
