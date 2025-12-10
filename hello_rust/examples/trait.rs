struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
    fn help(&self) -> String {
        "Good luck!".to_string()
    }
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("splc {}", file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {}", file_path)
    }
}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}
fn compile1(lang: impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

// Generic Traits
trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }
    fn first(&self) -> &T {
        &self[0]
    }
}

// Trait's bound

// max can be implemented only for Types where Partial Order is applicable
fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y { y } else { x }
}

trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl B for i32 {}
impl C for i32 {}

fn a<T: A>(x: T) {}
fn ab<T: A + B>(x: T) {}
fn w<T, U>(x: T, y: U)
where
    T: A + B,
    U: B + C,
{
}

fn main() {
    println!("Trait");
    let sol: Solidity = Solidity {
        version: "0.8".to_string(),
    };
    let vy: Vyper = Vyper {
        version: "0.4".to_string(),
    };

    println!("sol compile, version-{}: {}", sol.version, sol.help());
    println!("vyper compile, version-{}: {}", vy.version, vy.help());

    println!("sol compile: {}", compile(&sol, "hello.sol"));
    println!("vyper compile: {}", compile(&vy, "hello.vy"));

    println!("sol compile1: {}", compile1(sol, "hello.sol"));
    println!("vyper compile1: {}", compile1(vy, "hello.vy"));

    let v: Vec<u32> = vec![11, 45, 77];
    println!(
        "Generic Trait. v.count:{}, v.first:{}",
        v.count(),
        v.first()
    );

    println!("Trait's bound. x:{}, y:{}, max(x,y):{}", 5, 7, max(5, 7));

    let u: u32 = 1;
    let i: i32 = -1;

    a(u);
    // a(i); - err here because Trait A implemented only for u32
    // ab(i); - same err here because Trait A implemented only for u32, even having Trait B as well
    w(u, i);
}
