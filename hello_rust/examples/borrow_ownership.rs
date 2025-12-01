fn main() {
    println!("Borrow ownership");

    // Immutable borrow
    let s: String = "some string".to_string();
    let s1: &String = &s;
    let s2: &String = &s;
    let s3: &String = s2;
    println!(
        "s1,s2 and s3 are immutable references of s: {}_{}_{}",
        s1, s2, s3
    );

    // Mutable borrow
    let mut s: String = String::from("Some Mutable String");
    // it can be only one mutable reference. s2 will not compile ... BUT actually compiling ...
    let s1: &mut String = &mut s;
    let s2: &mut String = &mut s;

    // It should be either mutable or imutable borrowing, but not both in same time ... BUT actually compiling ...
    let mut s: String = String::from("Some Mutable String");
    let s11: &String = &s;
    let s21: &mut String = &mut s;

    // Refernce should not outlive the value
    let s: String = String::from("Rust");
    let s1: &String = &s;
    {
        let s2: String = s;
    } // s and s2 not longet exists
    // this code will crush:  println!("Refernce should not outlive the value. {}", s1);
}
