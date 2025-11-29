#[derive(Debug, PartialEq)]

enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32 },
}

fn main() {
    let cmd: Command = Command::Play;
    println!("Command: {:#?}", cmd);
    let cmd: Command = Command::Skip(10);
    println!("Command: {:#?}", cmd);
    let cmd: Command = Command::Resize {
        width: 100,
        height: 50,
    };
    println!("Command: {:#?}", cmd);

    let cmd0: Command = Command::Stop;
    let cmd1: Command = Command::Back(50);
    let cmd2: Command = Command::Back(10);
    let cmd3: Command = Command::Back(10);
    println!("Eq. of {:?} == {:?} = {:?}", cmd0, cmd1, cmd0 == cmd1);
    println!("Eq. of {:?} == {:?} = {:?}", cmd2, cmd1, cmd2 == cmd1);
    println!("Eq. of {:?} == {:?} = {:?}", cmd2, cmd3, cmd2 == cmd3);

    // Option<T> = Some<T> | None
    let x: Option<u32> = Some(1);
    println!("Option: {:?}", x);
    let x: Option<u32> = None;
    println!("Option: {:?}", x);

    // Result<T,E> = Ok(T) | Error(E)
    let x: Result<i32, String> = Ok(100);
    println!("Result: {:?}", x);
    let x: Result<i32, String> = Err("Some failier ...".to_string());
    println!("Result: {:?}", x);
}
