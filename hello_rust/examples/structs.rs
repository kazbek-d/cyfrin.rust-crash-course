#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Point3D(i32, i32, i32);
#[derive(Debug)]
struct Empty;

#[derive(Debug)]
struct Circle {
    radius: u32,
    center: Point,
}

fn main() {
    println!("Structs");
    let p: Point = Point { x: 1, y: 1 };
    println!("Point p={:?}", p);
    println!("Point p.x={:?}, p.y={:?} ", p.x, p.y);

    let p: Point3D = Point3D(0, 1, 2);
    println!("Point3D p={:#?}", p);
    println!("Point3D p.0={:?},p.1={:?},p.2={:?}", p.0, p.1, p.2);

    let e: Empty = Empty;
    println!("Empty e={:?}", e);

    let c: Circle = Circle {
        radius: 20,
        center: Point { x: 6, y: 7 },
    };
    println!("Circle c={:#?}", c);
    println!(
        "Circle c.radius={:?},c.center.x={:?},c.center.y={:?}",
        c.radius, c.center.x, c.center.y
    );

    // Shortcut
    let x: u32 = 1;
    let y: u32 = 10;
    let p: Point = Point { x: x, y: y };
    println!("Point p={:?}", p);
    let p: Point = Point { x, y };

    // Copy fields
    let p1: Point = Point { x: 5, ..p };
    println!("Copy fields: p1={:#?}", p1);

    // Update
    let mut p: Point = Point { x: 1, y: 1 };
    p.x = 15;
    p.y = 25;
    println!("Update Point p={:?}", p);

    // It results in a compile-time error because all struct fields must be explicitly initialized.
    // let p: Point = Point{};
}
