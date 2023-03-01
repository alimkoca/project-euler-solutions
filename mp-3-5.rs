fn main() {
    let mut multiples = Vec::new();
    
    for i in 1..1000 {
        match i {
            i if i % 3 == 0 && i % 5 == 0 => multiples.push(i),
            i if i % 3 == 0 => multiples.push(i),
            i if i % 5 == 0 => multiples.push(i),
            _ => continue
        }
    }
    let sum: u32 = multiples.iter().sum();
    println!("{}", sum);
}