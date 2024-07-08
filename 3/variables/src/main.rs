use std::io;

const MAX_POINTS: u32 = 100_000;

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let x = 5;

//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in the innner scope is: {}", x);
//     }

//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let spaces = "   ";
//     let spaces = spaces.len();
// }

// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {}", y);

//     println!("The value of z is: {}", tup.2);
// }

// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     let b = [3; 5];

//     let first = a[0];
// }

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}