use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    // test collect to make a HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // from here, try to use field_name&field_value will have a compile failure
    // but here try references will be OK
    println!("{}-{}", field_name, field_value);
    let another_field_name = 34;
    let another_field_value = String::from("Pink");
    let mut another_map = HashMap::new();
    another_map.insert(another_field_name, another_field_value);
    // another_field_name is fine as it implements Copy trait
    println!("{:?}", another_field_name);

    // try get wit result of Option
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // for loop
    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }

    // test to overwrite old value of the key
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // test entry API to check existing key
    scores.entry(String::from("Yellow")).or_insert(10);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}
