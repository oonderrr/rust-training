use rand::RngExt;
use rand::rngs::ThreadRng;

fn main() {
    let mut rnd = rand::rng();
    let some_number = rnd.random_range(1..1_000);
    println!("some number: {}", some_number);

    if some_number % 2 == 0 {
        println!("{some_number} is even");
    } else if some_number % 3 == 0 {
        println!("{some_number} is odd");
    } else {
        println!("{some_number} is another");
    }

    loop {
        let number = rnd.random_range(1..101);
        println!("number: {}", number);
        if number % 19 == 0 {
            println!("i got you {number}");
            break;
        } else {
            continue;
        }
    }

    let mut try_count = 0;
    while try_count < 100 {
        let number = rnd.random_range(1..101);
        if number % 17 == 0 {
            println!("i found the number {} in {} try", number, try_count);
            break;
        }
        try_count += 1;
    }

    let rand_numbers = get_random_numbers(rnd, 7);
    println!("random numbers: {:?}", rand_numbers);

    for (index, value) in rand_numbers.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }
}

fn get_random_numbers(mut rnd: ThreadRng, upper_limit: u8) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    for _ in 0..upper_limit {
        let n = rnd.random_range(1..1000);

        if numbers.contains(&n) {
            continue;
        }

        numbers.push(n);
    }

    numbers
}
