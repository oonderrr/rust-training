fn main() {
    greetings("Önder");

    let value_1 = 11.4;
    let value_2 = 33.2;
    let total = sum(value_1, value_2);
    println!("Total: {}", total);

    let numbers = [43, 32, 23, 65, 21, 54, 24, 65, 12];
    println!("Numbers: {:?}", numbers);

    let odds = get_odds(&numbers);
    println!("Odds: {:?}", odds);

    let evens = get_evens(&numbers);
    println!("Evens: {:?}", evens);

    let (x1, y1) = (23.32, 54.43);
    println!("x1: {}, y1: {}", x1, y1);

    let (x2, y2) = move_position(x1, y1, 10.3);
    println!("x2: {}, y2: {}", x2, y2);
}

fn greetings(name: &str) {
    println!("Hello, {}!", name)
}

fn sum(a: f32, b: f32) -> f32 {
    a + b
}

fn get_odds(numbers: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for number in numbers {
        if number % 3 == 0 {
            result.push(*number)
        }
    }

    result
}

fn get_evens(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&number| number % 2 == 0)
        .copied() // or .cloned()
        .collect()
}

fn move_position(mut x: f32, mut y: f32, acceleration: f32) -> (f32, f32) {
    x += acceleration;
    y += acceleration;
    (x, y)
}
