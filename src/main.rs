use std::io;
use std::io::Write;
use collatz_lib::generate_collatz_sequence;

fn main() {
    print!("Start Number: ");
    io::stdout().flush().unwrap();
    let mut start = String::new();
    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read line");
    const MAX_WIDTH: usize = 1000;
    let start: usize = start.trim().parse().expect("Please type a number!");
    println!("====================================");

    let (seq, step) = generate_collatz_sequence(start).unwrap_or_else(|err| {
        println!("{err}");
        std::process::exit(1);
    });
    println!("Generated {} steps.", step);

    for val in seq {
        let val = val.min(MAX_WIDTH);
        println!("{:>width$}:{val}", "*", width = val)
    }

    println!("====================================");
    println!("Without exception.");
}

