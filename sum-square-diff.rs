fn main() {
    let mut sum_of_sq: i32 = 0;
    let mut sq_of_sum: i32 = 0;

    for i in 0..101 as i32 {
        sum_of_sq += i.pow(2);
        sq_of_sum += i;
    }

    sq_of_sum = sq_of_sum.pow(2);
    println!("{}", sq_of_sum-sum_of_sq);
}