fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let quit = Message::Quit;
    let move_message = Message::Move { x: 10, y: 20 };
    let write = Message::Write(String::from("Hello, world!"));
    let change_color = Message::ChangeColor(255, 0, 0);

    quit.call();
    move_message.call();
    write.call();
    change_color.call();

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Alaska);

    println!("Penny: {}", value_in_cents(penny));
    println!("Nickel: {}", value_in_cents(nickel));
    println!("Dime: {}", value_in_cents(dime));
    println!("Quarter: {}", value_in_cents(quarter));
}

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4(ip) => println!("V4: {}", ip),
        IpAddrKind::V6(ip) => println!("V6: {}", ip),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to r: {}, g: {}, b: {}", r, g, b)
            }
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}
