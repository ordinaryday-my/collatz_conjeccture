use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    print!("Start Number: ");
    io::stdout().flush().unwrap();
    let mut start = String::new();
    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read line");
    let start: usize = start.trim().parse().expect("Please type a number!");
    println!("====================================");
    let mut num_1_cnt = 0;
    let mut cur = start;
    loop {
        cur = if cur % 2 != 0 {
            3 * cur + 1
        } else {
            cur / 2
        };

        if num_1_cnt > 40 {
            break;
        }

        print!("{text:>width$}:{num}", text="*", width=cur, num=cur);
        if cur == 1 {
            num_1_cnt += 1;
            if num_1_cnt > 3 {
                print!("        Endless cycle..................")
            }
        }
        println!();
        thread::sleep(Duration::from_millis(100));
    }

    println!("====================================");
    println!("Without exception.");
}
