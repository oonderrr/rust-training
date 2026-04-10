fn main() {
    let hello_world = "Hello world!"; // &str
    println!("{}", hello_world);
    let name = "Önder".to_string(); // String
    let short_name = name.replace("Önder", "Ön.");
    println!("{}", short_name);

    let position = String::from("Quarter Back");
    println!("{}", position);

    let greetings = "Greeting dear".to_string();
    println!("{}", greetings);
    let short_greetings = greetings.get(0..8).unwrap();
    println!("{:#?}", short_greetings);

    let hello = "Hello!";
    let hello_world = "Hello world!".to_string();
    let hello_ref = &hello_world;
    println!("{}", hello);
    println!("{}", hello_world);
    println!("{}", hello_ref);

    let konnichiwa = "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}";
    println!("{}", konnichiwa);
}
