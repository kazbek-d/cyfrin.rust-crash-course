fn main() {
    println!("Vector");

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector v:{:?}", v);

    let v: Vec<i32> = vec![1, 15, 64];
    println!("Vector v:{:?}", v);

    let v = vec![1i32, 7, 11];
    println!("Vector v:{:?}", v);

    let v: Vec<i32> = vec![0i32; 10];
    println!("Vector v:{:?}", v);

    let v: Vec<i32> = vec![1, 15, 64];
    println!("Vector v[0]:{:?}", v[0]);

    // Crush
    // println!("Vector v[100]:{:?}", v[100]);

    // Option(&i32)
    println!("Option Vector v[1]:{:?}", v.get(1)); // Some(i32)
    println!("Option Vector v[100]:{:?}", v.get(100)); // None

    // Update
    let mut v: Vec<i32> = vec![1i32, 7, 11];
    v[0] = 77;
    v[1] += 1;
    println!("Update Vector v:{:?}", v);

    let x1: Option<&i32> = v.get(1);
    let x2: Option<&i32> = v.get(100);
    println!("Option Vector v[1]:{:?}", x1); // Some(i32)
    println!("Option Vector v[100]:{:?}", x2); // None

    let mut v: Vec<i32> = vec![1, 2];
    let x1: Option<i32> = v.pop();
    let x2: Option<i32> = v.pop();
    let x3: Option<i32> = v.pop();
    println!("Vector POP. x1={:?}, x2={:?}, x3={:?},", x1, x2, x3);

    let v: Vec<i32> = vec![45, 12, 333, 78, 95];
    let s: &[i32] = &v[1..4]; // start index: 1 (including), end index: 4 (not including). Eq: [12,333,78] 
    println!("Vector slice. s={:?},", s);
}
