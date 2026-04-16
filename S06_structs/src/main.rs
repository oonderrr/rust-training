fn main() {
    let spiderman = spawn_random_player("Spiderman".to_string());
    println!("{:#?} is running", spiderman);

    let position = Position(1.1, 4_f32);
    let x_value = position.0;
    let y_value = position.1;
    println!("x: {}, y: {}", x_value, y_value);

    let mut rectangle = Rectangle::new(33, 42);
    println!("rectangle area: {}", rectangle.area());
    rectangle.resize(83, 33);
    let rectangle_area_2 = Rectangle::area(&rectangle);
    println!("rectangle area 2: {}", rectangle_area_2);
}

fn spawn_random_player(name: String) -> Player {
    Player {
        name,
        level: 101,
        is_active: true,
        position: Position(322_f32, 232.33),
    }
}

#[derive(Debug)]
struct Position(f32, f32);

#[derive(Debug)]
struct Player {
    name: String,
    level: u8,
    position: Position,
    is_active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(w: u32, height: u32) -> Self {
        Rectangle { width: w, height }
    }
    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn resize(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
    }
}
