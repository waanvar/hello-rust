use std::collections::HashMap;

fn main() {
    let s: String = "สวัสดี".to_string();
    println!("Thai hello say : {}", s);

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    for (key, value) in &scores {
        println!("{key} : {value}");
    }
}
