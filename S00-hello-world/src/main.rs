fn main() {
    let name = String::from("Önder Buğdaci");
    println!("Hello, {name}");

    let mut player_score = 99;
    player_score += 1;
    println!("Player score: {player_score}");

    let _delta_time = 1.25;
    let _delta_time: f32 = 1.25;
    let _delta_time = 1.25f32;
    let delta_time = 1.25_f32;

    println!("Delta time: {delta_time}");

    let _total_points: u8 = 1 + 5 + 7;

    let colors_in_hex = 0xFF0046;
    println!("Colors: {colors_in_hex:x}");

    // let salary = 1_434_434;
    let gate_flag: u8 = 0b1001_1010;
    println!("Gate flag: {gate_flag:b} / {gate_flag}");

    let is_active = true;
    println!("Is active: {is_active}");

    let first_char = 'a';
    println!("First char: {first_char}");

    let config = (640, 480, String::from("Main Title"), false);
    println!("Config: {config:#?}");

    let width = config.0;
    let height = config.1;
    let (w, h) = (width, height);
    println!("The screen resolution is {w}:{h}");

    let mut scores: [u8; 5] = [1, 23, 35, 42, 56];
    println!("Scores: {scores:?}");
    println!(
        "First score is {}. Scores length: {}",
        scores[0],
        scores.len()
    );
    scores[0] += 50;
    println!("Scores: {scores:?}");

    println!("Background color: {BACKGROUND_COLOR:?}");
}

const BACKGROUND_COLOR: (u8, u8, u8) = (0xff, 0xff, 0xff);
