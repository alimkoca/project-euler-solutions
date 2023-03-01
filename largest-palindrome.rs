fn check_palindrome(num: u32) -> bool {
    let mut num_u = num;
    let mut reversed = 0;
    let mut ld = 0;

    while num_u > 0 {
        ld = num_u % 10;
        reversed = (reversed*10) + ld;
        num_u /= 10;
    }

    //println!("{reversed}, {num}");
    if reversed == num {
        true
    } else {
        false
    }
}

fn main() {
    let mut largest_palindrome = 0;

    for i in 100..1000 {
        for j in 100..1000 {
            if check_palindrome(i*j) && i*j > largest_palindrome {
                largest_palindrome = i*j;
            }
        }
    }

    println!("{largest_palindrome}");
}