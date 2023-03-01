fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a*b)/gcd(a, b)
}

fn main() {
    let mut factor: u64 = 1;
    let mut lmp = 1;

    for i in 1..21 {
        lmp = lcm(i, lmp);
    }

    println!("{lmp}");
}