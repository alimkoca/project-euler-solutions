pub fn nth_prime(n: u32) -> Option<u64> {
    if n < 1 {
        return None;
    }

    // The prime counting function is pi(x) which is approximately x/ln(x)
    // A good upper bound for the nth prime is ceil(x * ln(x * ln(x)))
    let x = if n <= 10 { 10.0 } else { n as f64 };
    let limit: usize = (x * (x * (x).ln()).ln()).ceil() as usize;
    let mut sieve = vec![true; limit];
    let mut count = 0;

    // Exceptional case for 0 and 1
    sieve[0] = false;
    sieve[1] = false;

    for prime in 2..limit {
        if !sieve[prime] {
            continue;
        }
        count += 1;
        if count == n {
            return Some(prime as u64);
        }

        for multiple in ((prime * prime)..limit).step_by(prime) {
            sieve[multiple] = false;
        }
    }
    None
}

fn main() {
    let _prime = nth_prime(10001).unwrap();
    println!("{_prime}");
}