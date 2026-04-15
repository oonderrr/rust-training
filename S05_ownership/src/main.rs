fn main() {
    {
        let player_score = 33;
        println!("player_score: {}", player_score);
    }

    // println!("player_score: {}", player_score); error: Cannot find value

    move_rule();

    let first_name = String::from("John");
    let full_name = first_name + "Doe";
    // println!("name: {}", first_name); error: Value used after being moved
    println!("full_name: {}", full_name);

    let book_title = String::from("Programming with Rust");
    search_book_title(book_title);
    // println!("book_title: {}", book_title); error: borrow of moved value: `book_title`

    let book_title_2 = String::from("Programming with C");
    search_book_title_2(&book_title_2);
    println!("book_title_2: {}", book_title_2);

    let department = String::from("Finance");
    let department = check_department(department);
    println!("department: {}", department);

    only_one_mutable()
}

fn only_one_mutable() {
    let mut greetings = String::from("hello");

    let ref_1 = &greetings;
    let ref_2 = &greetings;
    //let mutable_reference = &mut greetings; error: cannot borrow `greetings` as mutable because it is also borrowed as immutable [E0502]
    // mutable borrow occurs here
    println!("ref_1: {}, ref_2: {}", ref_1, ref_2);
    // println!("mutable_reference: {}", mutable_reference);
}

fn check_department(department: String) -> String {
    println!("checking department: {}", department);
    department
}

fn search_book_title(book_title: String) {
    println!("{} is searching...", book_title);
}

fn search_book_title_2(book_title: &String) {
    println!("{} is searching...", book_title);
}

fn move_rule() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let colors_again = colors;

    println!("colors again: {:?}", colors_again);
    // println!("colors {:?}", colors) error: Value used after being moved

    let number = 25_u8;
    let mut other_number = number;
    other_number = number + 2;
    println!("number: {}", number);
    println!("other_number: {}", other_number);
}
