use std::fmt::Debug;
use std::ops::Add;

fn main() {
    log_any(32);
    log_any("Account not found");
    log_any(false);

    println!("----------------------------");

    let point1 = Point::<i32>::new(2, 4, 6);
    println!("point1: {}", point1.info());

    let point2 = Point::new(3, 5, 7);
    println!("point2: {}", point2.info());

    let point3 = point1.add(point2); // point moved here
    // point1.info(); error
    println!("point1 + point2 = point3: {}", point3.info());
}

fn log_any<T: Debug>(object: T) {
    println!("Logged object is {:?}", object);
}

struct Point<T: Debug> {
    x: T,
    y: T,
    z: T,
}

impl<T: Debug + Add<Output = T>> Point<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Point { x, y, z }
    }
    fn info(&self) -> String {
        format!("x: {:?}, y: {:?}, z: {:?}", self.x, self.y, self.z)
    }
    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
