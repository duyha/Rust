fn main() {
    println!("Hello, world!");
}

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

#[test]
fn test_gcd() {
    assert_eq!(gcd(2,3), 1);
    assert_eq!(gcd(14, 20), 2);
    assert_eq!(gcd(16, 20), 4);
}
