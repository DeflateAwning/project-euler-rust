fn main() {
    let mut a0: u128 = 1;
    let mut a1: u128 = 2;
    let mut sum: u128 = 0;

    let max_val: u128 = 4_000_000; // 4 million

    while a1 < max_val {
        if a1 % 2 == 0 {
            sum += a1;
        }

        let temp = a1;
        a1 = a0 + a1;
        a0 = temp;
    }

    println!("Sum of even Fibonacci numbers less than {}: {}", max_val, sum);
}
