use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers: Vec<u64> = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gdc(d, *m);
    }
    println!("The greatest common devisor of {:?} is {}", numbers, d);
}

fn gdc(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t
        }
        m = m % n
    }
    return n
}

#[test]
fn test_gdc() {
    assert_eq!(gdc(12, 8), 4);
}