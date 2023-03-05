fn main() {
    let limit = 1000;

    'outer: for a in 0..limit as i32 {
        for b in 1..limit as i32 {
            for c in 2..limit as i32 {
                if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == 1000 && c > b && b > a {
                    println!("{a}, {b}, {c}");
                    break 'outer;
                }
            }
        }
    }
}