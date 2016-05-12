fn main() {
    let input: u64 = 2000000;
    let mut house = 0;

    loop {
        let presents = house_presents(house);

        if house % 10000 == 0 {
            println!("house: {}\tpresents: {}", house, presents);
        }
        if presents >= input {
            break
        } else {
            house += 1;
        }
    }
    println!("house: {}", house);
}


fn house_presents(number: u64) -> u64 {
    let mut total = 0;

    for i in 1..(number/2 + 2) {
        if number % i == 0 {
            total += i * 10;
        }
    }

    total += number * 10;

    return total;
}
