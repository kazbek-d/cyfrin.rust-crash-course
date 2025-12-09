#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

#[derive(Debug)]
struct PointF {
    x: f32,
    y: f32,
}
impl PointF {
    // Static method (assosiatet to type)
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // Methods (assosiatet to instance)
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    println!("Generics");
    let p: Point<u32> = Point { x: 4, y: 5 };
    println!("Generics. Point: {:?}, x:{}, y:{}", p, p.x, p.y);

    let swap_result = swap(3u32, 6);
    println!("Generics. Funcs: {:?}", swap_result);

    let mut point_f: PointF = PointF { x: 0.0, y: 0.0 };
    println!("Generics. PointF: {:?}", point_f);
    point_f.move_to(45.7, 66.45);
    println!("Generics. PointF. Method.move_to: {:?}", point_f);

    let point_f: PointF = PointF::new(0.0, 100.0);
    println!("Generics. PointF, Static Method: {:?}", point_f);
}
