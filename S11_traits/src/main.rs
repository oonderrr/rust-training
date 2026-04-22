fn main() {
    let mut redis_instance = Redis;
    let mut watson = HealthCheck { is_online: false };

    sample_one(&mut redis_instance);
    sample_one(&mut watson);

    println!("------------------------");

    let debit_payment = DebitCard;
    let company_payment = CompanyAccount;
    let card_payment = CreditCard;
    debit_payment.pay(100.0);
    company_payment.pay(150.0);
    card_payment.pay(180.0);

    println!("--------------------------");

    let red_ball = Circle {};
    let blue_ball = Circle {};
    let wall = Square {};
    let warrior = Player {};
    let level_one: Vec<&dyn Draw> = vec![&red_ball, &blue_ball, &wall, &warrior];
    draw_shapes(&level_one)
}

fn sample_one<T: Service>(service: &mut T) {
    service.activate();
    println!("Service: {:?}", service.status());
    service.deactivate();
    println!("Service: {:?}", service.status());
}

trait Service {
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn status(&self) {
        println!("Service status: ");
    }
}

struct Redis;
struct HealthCheck {
    is_online: bool,
}

impl Service for Redis {
    fn activate(&mut self) {
        println!("service activate redis");
    }

    fn deactivate(&mut self) {
        println!("service deactivate redis");
    }
}

impl Service for HealthCheck {
    fn activate(&mut self) {
        self.is_online = true;
        println!("health check activate");
    }

    fn deactivate(&mut self) {
        self.is_online = false;
        println!("health check deactivate");
    }

    fn status(&self) {
        println!("health check status is {}", self.is_online);
    }
}

trait Payment {
    fn pay(&self, amount: f32) {
        println!("Payment amount {:.2} with cash.", amount);
    }
}

struct DebitCard;
impl Payment for DebitCard {}

struct CreditCard;
impl Payment for CreditCard {
    fn pay(&self, amount: f32) {
        let amount_with_commission = amount * 1.1;
        println!(
            "Payment amount {:.2} with credit card.",
            amount_with_commission
        );
    }
}

struct CompanyAccount;
impl Payment for CompanyAccount {}

trait Draw {
    fn draw(&self);
}

struct Circle {}
struct Square {}
struct Player {}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle")
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing square")
    }
}

impl Draw for Player {
    fn draw(&self) {
        println!("Drawing player")
    }
}

fn draw_shapes(shapes: &Vec<&dyn Draw>) {
    for shape in shapes.iter() {
        shape.draw()
    }
}
