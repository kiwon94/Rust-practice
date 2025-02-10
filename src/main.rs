mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;
mod problem9;
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
        3 => problem3::solve(),
        4 => problem4::solve(),
        5 => problem5::solve(),
        6 => problem6::solve(),
        7 => problem7::solve(),
        8 => problem8::solve(),
        9 => problem9::solve(),
        _ => println!("Problem not found"),
    }
}
