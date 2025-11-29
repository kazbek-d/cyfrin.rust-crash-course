fn main() {
    // String - when you'll need ownership or mutability
    // &str = string slice, when you'll need read-only

    let s: String = String::from("Hello Rust");
    println!("s: {s}");

    let s: String = "Hello Rust".to_string();
    println!("s: {s}");

    let length: usize = s.len();
    println!("length: {length}");

    let s: &str = &s[0..5];
    println!("slice: {s}");

    let s: &str = "Hello Rust";
    let s: String = s.to_string();
    println!("String: {s}");

    let mut s: String = String::from("Hello");
    s += " Rust";
    println!("String mutation: {s}");

    let a: String = "Rust".to_string();
    let s: String = format!("Hello {}", a);
    println!("String Interpolation: {s}");
}
