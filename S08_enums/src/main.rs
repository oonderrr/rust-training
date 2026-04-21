use chrono::{DateTime, Utc};
use rand::RngExt;

enum Status {
    Success,
    InProgress,
    Error(String),
}

impl Status {
    fn get_info(&self) -> &str {
        match self {
            Status::Success => "Success",
            Status::InProgress => "InProgress",
            Status::Error(detail) => detail,
        }
    }
}

fn create_report(title: String) -> Status {
    if title.is_empty() {
        return Status::Error("Title cannot be empty.".to_string());
    }

    let mut random = rand::rng();
    let value = random.random_range(1..=3);

    if value % 3 == 0 {
        return Status::InProgress;
    }

    Status::Success
}

#[derive(Debug)]
enum Level {
    Rookie,
    Pro,
    Elit(f32),
}

#[derive(Debug)]
struct Player {
    name: String,
    level: Level,
    is_active: bool,
}

impl Player {
    fn new(name: String, level: Level) -> Self {
        Player {
            name,
            level,
            is_active: false,
        }
    }
    fn change_level(&mut self, level: Level) {
        self.level = level;
    }
    fn activate(&mut self) {
        self.is_active = true;
    }
    fn deactivate(&mut self) {
        self.is_active = false;
    }
}

#[derive(Debug)]
enum User {
    Inactive {
        name: String,
    },
    Active {
        name: String,
        activation_date: DateTime<Utc>,
    },
}

impl User {
    fn activate(&mut self, activation_date: DateTime<Utc>) -> Option<User> {
        match self {
            User::Inactive { name, .. } => {
                let created = User::Active {
                    name: name.to_string(),
                    activation_date,
                };

                Some(created)
            }
            User::Active { name, .. } => None,
        }
    }
}

fn main() {
    let report = create_report(String::from("Title"));
    println!("report: {}", report.get_info());

    let report = create_report(String::new());
    println!("report: {}", report.get_info());

    let mut player = Player::new(String::from("Önder"), Level::Rookie);
    println!("player: {:#?}", player);

    player.activate();
    player.change_level(Level::Elit(1.0));
    println!("player : {:#?}", player);

    // match player.level {
    //     Level::Elit(elo_rate) => {
    //         return println!("Now Önder is on Elite league with {} rate point", elo_rate);
    //     }
    //     _ => {}
    // }

    let mut mario = User::Inactive {
        name: "Mario".to_string(),
    };
    println!("User {:?}", mario);

    if let Some(usr) = mario.activate(Utc::now()) {
        println!("user activated: {:#?}", usr);
    }
}
