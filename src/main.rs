mod problem1;
mod problem10;
mod problem11;
mod problem12;
mod problem13;
mod problem14;
mod problem15;
mod problem16;
mod problem17;
mod problem18;
mod problem19;
mod problem2;
mod problem20;
mod problem21;
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
        10 => problem10::solve(),
        11 => problem11::solve(),
        12 => problem12::solve(),
        13 => problem13::solve(),
        14 => problem14::solve(),
        15 => problem15::solve(),
        16 => problem16::solve(),
        17 => problem17::solve(),
        18 => problem18::solve(),
        19 => problem19::solve(),
        20 => problem20::solve(),
        21 => problem21::solve(),
        _ => println!("Problem not found"),
    }
}
