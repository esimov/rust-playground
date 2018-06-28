fn main() {
    let res = gdc(120, 48);
    println!("The greatest common devisor: {}", res)
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