fn largest_prime(mut num: u64) -> u64 {
    let mut c = 2;
    let mut max: u64 = 0;

    while num > 1 {
        while num % c == 0 {
            if c > max {
                max = c;
            }
            num /= c;
        }
        c += 1;
    }

    max
}
fn main() {
    let max_prime = largest_prime(600851475143);
    println!("{max_prime:?}");
}