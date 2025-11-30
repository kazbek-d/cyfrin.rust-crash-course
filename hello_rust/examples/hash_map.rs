use std::collections::HashMap;

fn main() {
    println!("HashMap");
    let scores: HashMap<String, u32> = HashMap::new();
    println!("HashMap scores={:?}", scores);

    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("Bob".to_string(), 7);
    scores.insert("Alice".to_string(), 11);
    scores.insert("John".to_string(), 55);
    println!("HashMap insert scores={:#?}", scores);

    // Get
    let bob: Option<&u32> = scores.get("Bob");
    let mike: Option<&u32> = scores.get("Mike");
    println!("HashMap get scores Bob={:?}, Mike={:?}", bob, mike);

    // Update
    let bob: &mut u32 = scores.entry("Bob".to_string()).or_default();
    *bob += 10;
    println!("HashMap update scores by Entry Bob={:?}", bob);

    let mike: &mut u32 = scores.entry("Mike".to_string()).or_default();
    *mike += 33;
    println!("HashMap update scores by Entry Mike={:?}", mike);
    println!("HashMap update scores by Entry scores={:#?}", scores);

    let sam: &mut u32 = scores.entry("Sam".to_string()).or_insert(99);
    println!("HashMap update scores by Entry Sam={:?}", sam);
    println!("HashMap update scores by Entry scores={:#?}", scores);
}
