fn main() {
    println!("Control Flow");

    let x: i32 = 10;
    if x > 5 {
        println!("x > 5");
    } else if x == 5 {
        println!("x eq. 5");
    } else {
        println!("x < 5");
    }

    let z: i32 = if x > 0 {
        println!("x > 0");
        1
    } else if x == 0 {
        println!("x eq. 0");
        0
    } else {
        println!("x < 0");
        -1
    };
    println!("IF/ELSE return value: {}", z);

    let mut i: u32 = 0;
    loop {
        if i > 5 {
            break;
        }
        println!("Loop. i={i}");
        i += 1;
    }

    i = 0;
    while i <= 5 {
        println!("While Loop. i={i}");
        i += 1;
    }

    for i in 0..6 {
        println!("For Loop. i={i}");
    }

    let arr: [i32; 5] = [1, 5, 6, 7, 9];
    let n: usize = arr.len();
    for i in 0..n {
        println!("For Loop. arr[{i}]={}", arr[i]);
    }

    for item in arr {
        println!("For Loop in arr. item={item}");
    }

    let v: Vec<i32> = vec![10, 55, 77];
    for item in v.into_iter() {
        // that is eq to
        //     for item in v.into_iter()
        // and will not run twise
        println!("For Loop in vector. v={item}");
    }

    let v: Vec<i32> = vec![10, 55, 77];
    // v.iter will allow to run it multiple times
    for item in v.iter() {
        println!("For Loop in vector. Run 1. v={item}");
    }
    for item in v.iter() {
        println!("For Loop in vector. Run 2. v={item}");
    }

    let mut x: u32 = 0;
    let loop_return_value: String = loop {
        x += 1;
        if x > 5 {
            break format!("End with x={x}");
        }
    };
    println!("Loop returns value: {loop_return_value}");

    let i: i32 = 5;
    match i {
        1 => println!("Match One"),
        2 => println!("Match Two"),
        _ => println!("Match Others"),
    }
    match i {
        4 | 5 | 6 => println!("Match 4 or 5 or 6"),
        _ => println!("Match Others"),
    }
    match i {
        3..5 => println!("Match from 3 to 5 (excluding the last index) "),
        4..=5 => println!("Match from 4 to 5 (including the last index)"),
        _ => println!("Match Others"),
    }
    match i {
        item @ 0..=10 => println!("Match from 0 to 10. item={item}"),
        _ => println!("Match Others"),
    }

    let ov: Option<u32> = Some(35);
    match ov {
        Some(val) => println!("Match Option. Some({val})"),
        None => println!("Match Option. None"),
    }

    let res: Result<u32, String> = Ok(55);
    match res {
        Ok(val) => println!("Match Result. Ok({val})"),
        Err(val) => println!("Match Result. Err({val})"),
    }
    let res: Result<u32, String> = Err("Some error happend".to_string());
    match res {
        Ok(val) => println!("Match Result. Ok({val})"),
        Err(val) => println!("Match Result. Err({val})"),
    }

    let res: Result<u32, String> = Ok(77);
    let ret: u32 = match res {
        Ok(val) => val,
        Err(_) => 0,
    };
    println!("Match return value={ret}");

    let ov: Option<u32> = Some(99);
    match ov {
        Some(val) => println!("Match Option. Some({val})"),
        None => {}
    }
    // if only first condition is to consider, it's the same as:
    if let Some(val) = ov {
        println!("Match Option. Consider only some({val})");
    }
}
