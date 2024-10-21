extern crate gui;
use gui::Draw;
use gui::{Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("MayBe"),
//                     String::from("No")
//                 ],
//             }),
//             Box::new(Button {
//                 width: 50,
//                 height: 10,
//                 label: String::from("OK"),
//             }),
//         ],
//     };

//     screen.run();
// }

extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());

    // post.approve();

    let post = post.request_review();

    let post = post.approve();
    
    assert_eq!("I ate a salad for lunch today", post.content());
}