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

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&b));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&c));
//     }
//     println!("count after goes out of scope = {}", Rc::strong_count(&a));
// }

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use List::{Cons, Nil};
// use std::rc::Rc;
// use std::cell::RefCell;

// fn main() {
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

//     let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

//     *value.borrow_mut() += 10;

//     println!("a after = {:?}", a);
//     println!("b after = {:?}", b);
//     println!("c after = {:?}", c);
// }

// #[dervie(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match *self {
//             Cons(_, ref item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
    
//     println!("a next item = {:?}", a.tail());
    
//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
    
//     println!("b initial rc count = {}", Rc::strong_count(&b));

//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));

//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     //println!("a next item = {:?}", a.tail());
// }

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak_count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!("leaf strong = {}, weak_count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak_count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}