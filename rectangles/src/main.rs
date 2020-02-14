#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if (self.height > other.height && self.width > other.width) ||
            (self.height > other.width && self.width > other.height) {
            true
        } else {
            false
        }
    }
}

fn main() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// let some_number = Some(5);
// let some_string = Some("a string");
// let absent_number: Option<i32> = None;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State {:?}", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
