fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(n != 0 && m!=0);
    while m !=0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    println!("Hello, world!");
    println!("GCD for 20 and 16 is {}", gcd(20, 16));
}
