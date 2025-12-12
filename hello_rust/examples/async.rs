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

#[tokio::main]
async fn main() {
    println!("Async");
    let h: Hamburger = make_humburger_seq().await;
    println!("Async. Tokio. Hamburger Seq: {:?}", h);

    let h: Hamburger = make_humburger().await;
    println!("Async. Tokio. Hamburger Join: {:?}", h);
}
