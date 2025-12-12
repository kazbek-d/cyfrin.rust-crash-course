use std::time::Duration;
use tokio::{join, select};

#[derive(Debug)]
struct Tomato;
#[derive(Debug)]
struct Lettuce;
#[derive(Debug)]
struct Cheese;
#[derive(Debug)]
struct Patty;
#[derive(Debug)]
struct Bun;

#[derive(Debug)]
struct Hamburger {
    pub tomato: Tomato,
    pub lettuce: Lettuce,
    pub cheese: Cheese,
    pub patty: Patty,
    pub bun: Bun,
}

async fn toast_bun() -> Bun {
    Bun
}

async fn cook_patty() -> Patty {
    Patty
}

async fn get_veggies() -> (Tomato, Lettuce) {
    (Tomato, Lettuce)
}

async fn get_cheese() -> Cheese {
    Cheese
}

async fn make_humburger_seq() -> Hamburger {
    let bun: Bun = toast_bun().await;
    let patty: Patty = cook_patty().await;
    let (tomato, lettuce) = get_veggies().await;
    let cheese: Cheese = get_cheese().await;

    Hamburger {
        tomato,
        lettuce,
        cheese,
        patty,
        bun,
    }
}

async fn make_humburger() -> Hamburger {
    let (bun, patty, (tomato, lettuce), cheese) =
        join!(toast_bun(), cook_patty(), get_veggies(), get_cheese());

    Hamburger {
        tomato,
        lettuce,
        cheese,
        patty,
        bun,
    }
}

// join!
// - Polls multiple Futures concurently
// - Waits for all of them to complete
// - Returns a tuple of their results

// select!
// - Polls multiple Futures concurently
// - Returns as soon as one of them completes
// - Cancels the rest (drops them)

// Simulate an async task that completes after 'dt' milliseconds
async fn make(val: &'static str, dt: u64) -> &'static str {
    tokio::time::sleep(Duration::from_millis(dt)).await;
    val
}

#[tokio::main]
async fn main() {
    println!("Async");
    let h: Hamburger = make_humburger_seq().await;
    println!("Async. Tokio. Hamburger Seq: {:?}", h);

    let h: Hamburger = make_humburger().await;
    println!("Async. Tokio. Hamburger Join: {:?}", h);

    let (res1, res2, res3) = join!(
        make("coffee", 100),
        make("green tea", 50),
        make("lemonade", 20)
    );
    println!("Results Res1:{}", res1);
    println!("Results Res2:{}", res2);
    println!("Results Res3:{}", res3);

    let res = select! {
    val = make("coffee", 20) => {
        println!("coffee");
        val
    },
    val = make("green tea", 20) => {
        println!("green tea");
        val
    },
    val = make("lemonade", 20) => {
        println!("lemonade");
        val
    },
    };
    println!("Results Res:{}", res);
}
