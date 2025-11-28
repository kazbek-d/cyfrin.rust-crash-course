#![allow(unused)]

// Constants
const NUM: u32 = 1;

fn add_with_return(x: u32, y: u32) -> (u32, bool) {
    return (x + y, true);
}

fn add_without_return(x: u32, y: u32) -> u32 {
    x + y
}

fn add_without_return1(x: u32, y: u32) -> u32 {
    x + y
}

fn printtt(s: String) {
    print!("{s}{s}{s}{s}{s}");
}

fn main() {
    println!("Hello world!");
    println!("Hello world!");
    println!("Hello world!");

    let mut x: i32 = 1;
    x += 2;

    // shadowing
    let y: i32 = 1;
    let y: bool = true;

    // type placeholder
    let z: _ = 1234;

    // Inline
    println!("x is {x}");
    // println
    println!("y is {}", y);
    // Positional arguments
    let a: i32 = 1;
    println!("{0} + {0} = {1}", a, a + a);
    // Debug
    println!("DEBUG: x {:#?}", x);
}
