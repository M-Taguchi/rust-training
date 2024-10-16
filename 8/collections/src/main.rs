fn main() {

    // let v: Vec<i32> = Vec::new();
    
    // let v = vec![1, 2, 3];
    
    // let mut v = Vec::new();
    
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    
    // {
    //     let v = vec![1, 2, 3, 4];
    // }
    
    // let v = vec![1, 2, 3, 4, 5];
    
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);
    
    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
    
    // let mut v = vec![1, 2, 3, 4, 5];
    
    // let first = &v[0];
    
    // v.push(6);
    
    // println!("The first element is: {}", first);

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }

    // let mut v = vec![100, 32, 57];

    // for i in &mut v {
    //     *i += 50;
    // }

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ]

    // let mut s = String::new();
    
    // let data = "initial contents";

    // let s = data.to_string();

    // let s = "initial contents".to_string();

    // let s = String::from("initial contents");

    // let hello = String::from("你好");

    // let mut s = String::from("foo");
    // s.push_str("bar");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {}", s2);

    // let s1 = String::from("Hello, ");
    // let s2 = string::from("world!");
    // let s3 = s1 + &s2;

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{}-{}-{}", s1, s2, s3);

    // let hello = "Здравствуйте";

    // let s = &hello[0..4];

    // for c in "नमस्ते".chars() {
    //     println!("{}", c);
    // }

    // for b in "नमस्ते".bytes() {
    //     println!("{}", b);
    // }

    use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // // println!("{}", score.unwrap());
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }
    scores.insert(String::from("Blue"), 10);

    // scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}