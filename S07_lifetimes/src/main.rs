fn main() {
    let value;

    {
        let number = 32.2;
        // value = &number error: `number` does not live long enough
        value = number;
    }

    println!("value: {}", value);

    let name = String::from("Önder");
    let onder = Account {
        customer_name: &name,
        balance: value,
    };

    println!("{} has a {} coin", onder.customer_name, onder.balance);

    let settings = get_application_settings();
    println!("server address: {}", settings.server_address);
    println!("connection string: {}", settings.connection_string)
}

struct Account<'a> {
    // "a" lifetime notice
    customer_name: &'a str,
    balance: f32,
}

struct ApplicationSettings<'a> {
    server_address: &'a str,
    connection_string: &'a str,
}

fn get_application_settings() -> ApplicationSettings<'static> {
    ApplicationSettings {
        server_address: "localhost",
        connection_string: "tcp://127.0.0.1:8080",
    }
}
