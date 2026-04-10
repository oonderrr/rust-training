fn main() {
    let mut scores: Vec<u8> = vec![10, 11, 12, 13, 14, 15];
    scores.push(88);
    scores.push(77);
    println!("{:?}", scores);

    for score in scores.iter() {
        println!("score: {}", score);
    }

    let last_score = scores.pop(); // Some(77) .unwrap(); 77
    println!("last_score: {:?}", last_score);
    println!("{:?}", scores);

    let mut colors = Vec::new();
    colors.push(String::from("red"));
    colors.push(String::from("green"));
    colors.push(String::from("blue"));
    // colors.push(7) error
    println!("{:?}", colors);
    colors.reverse();
    println!("{:?}", colors);

    let codes: Vec<u8> = (50..=100).collect();
    println!("{:?}", codes);

    let numbers = (11..44).collect::<Vec<u8>>();
    let first_two = numbers[0..2].to_vec();
    println!("{:?}", first_two);
}
