const LIMIT: usize = 2000000;

fn sieve() -> Vec<bool> {
    let mut primes: Vec<bool> = vec![true; LIMIT];

    primes[0] = false;
    primes[1] = false;

    for p in 2..LIMIT {
        if !primes[p] {
            continue;
        }

        for m in ((p*p)..LIMIT).step_by(p) {
            primes[m as usize] = false;
        }
    }

    primes
}

fn main() {
    let mut sum: u64 = 0;
    let primes = sieve();
    for i in 0..primes.len() {
        if primes[i] == true {
            sum += i as u64;
        }
    }
    println!("{sum}");
}