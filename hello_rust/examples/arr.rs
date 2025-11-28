#![allow(unused)]

fn main() {
    let arr1: [u32; 3] = [0, 1, 2];
    let arr2: [u32; 5] = [1; 5];

    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);

    // slice
    // first 3
    let s: &[u32] = &arr2[..3];
    println!("s first 3: {:?}", s);
    // last 3 (indexes 3,4,5)
    let s: &[u32] = &arr2[3..];
    println!("s last 3 (indexes 3,4,5): {:?}", s);
    // midle (indexes 3,4)
    let s: &[u32] = &arr2[3..5];
    println!("s midle (indexes 3,4): {:?}", s);
}
