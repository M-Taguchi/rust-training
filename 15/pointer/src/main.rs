// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     // let b = Box::new(5);
//     // println!("b = {}", b);
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Nil)))));
// }

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }

// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello(&m);
// }

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// fn main() {
//     let c = CustomSmartPointer { data: String::from("my stuff") };
//     let d = CustomSmartPointer { data: String::from("other stuff") };
//     println!("CustomSmartPointers created.");
//     drop(c);
//     println!("CustomSmartPointer dropped before the end of main.")
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&b));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after goes out of scope = {}", Rc::strong_count(&a));
}