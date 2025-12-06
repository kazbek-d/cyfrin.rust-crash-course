use hello_rust::my;

fn main() {
    println!("Module");

    my::print();
    my::b::print();

    let s: my::b::S = my::b::S { x: 1, y: 2 };
    println!("{:?}, x:{}, y:{}", s, s.x, s.y);
}
