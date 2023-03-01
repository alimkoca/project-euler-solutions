fn main() {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut n3 = 0;
    let mut fib_sum: u64 = 0;

    while n3 < 4000000 {
        n3 = n2 + n1;
        n1 = n2;
        n2 = n3;

        if n3 % 2 == 0 {
            fib_sum += n3;
        }
    }

    println!("{fib_sum}");
}