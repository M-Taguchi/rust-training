
// fn main() {
    //     // panic!("crash and burn");
    //     let v = vec![1, 2, 3];
    
    //     v[99];
// }
    
use std::fs::File;
use std::io::ErrorKind;

// fn main() {
//     // let f = File::open("hello.txt");
//     // let f = File::open("hello.txt").unwrap();
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");

//     // let f = match f {
//     //     Ok(file) => file,
//     //     // Err(error) => {
//     //     //     panic!("There was a problem opening the file: {:?}", error)
//     //     // }
//     //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
//     //         match File::create("hello.txt") {
//     //             Ok(fc) => fc,
//     //             Err(e) => {
//     //                 panic!("Tried to create file but there was a problem: {:?}", e)
//     //             }
//     //         }
//     //     },
//     //     Err(error) => {
//     //         panic!("There was a problem opening the file: {:?}", error)
//     //     }
//     // }
// }

use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();

loop {
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {}
}

pub strcut Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}