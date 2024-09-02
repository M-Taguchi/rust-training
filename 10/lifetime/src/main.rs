use std::fmt::Display;

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

struct ImportantExceipt<'a> {
    part: &'a str,
    fn level(&self) -> i32 {
        3
    },
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attentuon please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExceipt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str where T: Display {
    println!("Annoucement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}