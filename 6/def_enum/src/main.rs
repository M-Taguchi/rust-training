// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("123.0.0.1"),
// }

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// }

// fn route(ip_type: IpAddrKind) { }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// let home = IpAddr::V4(String::from("123.0.0.1"));

// let loopback = IpAddr::V6(String::from("::1"));

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

let m = Message::Write(String::from("hello"));
m.call();

let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;

let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;