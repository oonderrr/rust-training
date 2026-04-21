use rand::RngExt;

fn main() {
    let call_response = ping("192.161.1.1");

    match call_response {
        HttpStatus::Ok => {
            println!("Http Status is Ok")
        }
        HttpStatus::Accepted => {
            println!("Http Status is Accepted 201")
        }
        HttpStatus::NotFound => {
            println!("Http Status is NotFound 404")
        }
        HttpStatus::BadRequest => {
            println!("Http Status is Bad Request 400")
        }
        HttpStatus::InternalServerError => {
            println!("Http Status Internal Error 500")
        }
    }

    check_exam(Student {
        id: 1,
        full_name: "Önder".to_string(),
        grade: 21,
    });

    let onder = User::new(1, "Önder".to_string(), None);
    println!("user info: {}", onder.info());

    let zin = User::new(2, "zin".to_string(), Some("zin@zin.com".to_string()));
    println!("user info: {}", zin.info());

    let accounts = load_accounts();
    let searching_account = find_account(&accounts, 1001);
    if let Some(account) = searching_account {
        println!(
            "Account for {} found: {} with balance {}",
            account.id, account.holder_name, account.balance
        )
    } else {
        println!("Account not found")
    }
}

fn check_exam(student: Student) {
    match student.grade {
        0..=49 => println!("Failed {}, {}", student.full_name, student.grade),
        50..=79 => println!("Passed {}, {}", student.full_name, student.grade),
        80..100 => println!(
            "Passed with congrats {}, {}",
            student.full_name, student.grade
        ),
        _ => println!("Invalid grade score",),
    }
}

fn ping(service_address: &str) -> HttpStatus {
    println!("Ping for the {}", service_address);
    let mut rng = rand::rng();
    let lucky_num = rng.random_range(0..=10);

    match lucky_num {
        1 => HttpStatus::Ok,
        2..=4 => HttpStatus::Accepted,
        5 => HttpStatus::BadRequest,
        9 | 10 => HttpStatus::NotFound,
        _ => HttpStatus::InternalServerError,
    }
}

#[derive(Debug)]
struct User {
    id: u32,
    title: String,
    email: Option<String>,
}

impl User {
    fn new(id: u32, title: String, email: Option<String>) -> User {
        User { id, title, email }
    }
    fn info(&self) -> String {
        match self.email {
            Some(ref e) => format!("{}-{} ({})", self.id, self.title, e),
            None => format!("{}-{}", self.id, self.title),
        }
    }
}

#[derive(Debug)]
enum HttpStatus {
    Ok,
    Accepted,
    NotFound,
    BadRequest,
    InternalServerError,
}

struct Student {
    id: u32,
    full_name: String,
    grade: u8,
}

#[derive(Debug)]
struct Account {
    id: u32,
    holder_name: String,
    balance: f32,
}

fn find_account(accounts: &Vec<Account>, id: u32) -> Option<&Account> {
    accounts.iter().find(|a| a.id == id)
}

fn load_accounts() -> Vec<Account> {
    return vec![
        Account {
            id: 1001,
            holder_name: "Önder".to_string(),
            balance: 10.5,
        },
        Account {
            id: 1002,
            holder_name: "Ahmet".to_string(),
            balance: 212.4,
        },
    ];
}
