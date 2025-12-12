use std::collections::HashMap;

fn main() {
    println!("Iter");

    // Iterators
    // - into_iter -> T
    // - iter -> &T
    // - iter_mut -> &mut T

    let v: Vec<i32> = vec![1, 6, 8, 9, 7];
    for i in v.iter() {
        println!("Iter. v_1:{}", i);
    }
    for i in v.iter() {
        println!("Iter. v_2:{}", i);
    }

    v.iter().for_each(|x: &i32| {
        let c = x + 9;
        println!("Iter. Map:{}", c);
    });

    let v2: Vec<i32> = v.iter().map(|x: &i32| x + 1).collect();
    println!("Iter. v2:{:?}", v2);

    let v3: Vec<(String, i32)> = v.iter().map(|x: &i32| (x.to_string(), x + 1)).collect();
    println!("Iter. Vec<(String, i32):{:?}", v3);
    let v4: HashMap<String, i32> = v3.into_iter().collect();
    let v3: HashMap<String, i32> = v.iter().map(|x: &i32| (x.to_string(), x + 1)).collect();
    println!("Iter. v4.HashMap<String, i32>:{:?}", v4);
    println!("Iter. v3.HashMap<String, i32>:{:?}", v3);

    let vf: Vec<&i32> = v
        .iter()
        .filter(|x: &&i32| **x > 2)
        .map(|x: &i32| x)
        .collect();
    println!("Iter. vf.iter:{:?}", vf);

    let vf: Vec<i32> = v
        .into_iter()
        .filter(|x: &i32| *x > 2)
        .map(|x: i32| x)
        .collect();
    println!("Iter. vf.into_iter:{:?}", vf);
}
