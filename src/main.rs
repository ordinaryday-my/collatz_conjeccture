use std::io;
use std::io::Write;

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

fn generate_collatz_sequence(start: usize) -> Result<(Vec<usize>, usize), String> {
    #[derive(Clone, Copy)]
    struct State {
        current: usize,
        one_count: usize,
    }

    let mut seq_iter = std::iter::from_fn({
        let mut state: State = State {
            current: start,
            one_count: 0,
        };
        let mut overflowed = false;

        move || {
            if overflowed {
                return None;
            }

            let next_val = if state.current % 2 != 0 {
                3usize
                    .checked_mul(state.current)
                    .and_then(|x| x.checked_add(1))
            } else {
                Some(state.current / 2)
            };

            match next_val {
                Some(val) => {
                    let new_count_1 = if val == 1 {
                        state.one_count + 1
                    } else {
                        state.one_count
                    };
                    if new_count_1 > 40 {
                        None
                    } else {
                        state.one_count = new_count_1;
                        state.current = val;
                        Some(Ok(val))
                    }
                }
                None => {
                    overflowed = true;
                    Some(Err("Overflow occurred".to_string()))
                },
            }
        }
    });

    seq_iter.try_fold((Vec::new(), 0usize), |(mut vec, count), val| {
        match val {
            Ok(v) => {
                vec.push(v);
                Ok((vec, count + 1))
            }
            Err(e) => Err(e),
        }
    })
}
