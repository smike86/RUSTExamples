use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Please provide a number as argument.");
        return;
    }

    let i : u32 = args[1].parse().ok().expect("Please input integer.");
    println!("{:?} has {:?} Collatz steps", i, collatz(i));
}

fn collatz(n: u32) -> u32 {
    if n == 1 { return 0; }
    match n % 2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(n*3+1) }
    }
}

pub fn compute(cycle: u32) -> u32 {
    let mut n = 0;
    let mut counter : u32 = 0;
    while counter != cycle {
        n = n+1;
        counter = collatz(n);
        println!("collatz({}) = {}", n, counter);
    }
    println!("{}", n);
    n
}
