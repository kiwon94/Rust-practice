mod problem1;
mod problem2;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a problem number");
        return;
    }

    let problem_number: u32 = args[1]
        .trim()
        .parse()
        .expect("Please provide a valid number");

    match problem_number {
        1 => problem1::solve(),
        2 => problem2::solve(),
        _ => println!("Problem not found"),
    }
}
