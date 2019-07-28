use std::io::Write;
use std::str::FromStr;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        if b < a {
            let c = b;
            b = a;
            a = c;
        }
        b = b % a;
    }
    a
}


fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        println!("Get argument {}", arg);
        numbers.push(u64::from_str(&arg).expect("error parsing string argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "There is no argument");
    }

    let mut d = numbers[0];
    for n in &numbers[1..] {
        println!("gcd = {}", d);
        d = gcd(d, *n);
    }
    println!("gcd = {}", d);
    println!("Hello, world!");
}
