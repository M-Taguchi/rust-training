// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("confition was true");
//     } else {
//         println!("codition was false");
//     }
// }

// fn main() {
//     let number = 3;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {}", number);
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {}", count);
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
   
//     for element in a {
//         println!("the value is: {}", element);
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}