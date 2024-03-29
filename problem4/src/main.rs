fn is_number_base_10_palindrome(n: u128) -> bool {
    let log_val = (n as f64).log10().floor() as u32;
    for i in 0..(log_val / 2 + 1) {
        let left = (n / 10u128.pow(log_val - i)) % 10;
        let right = (n / 10u128.pow(i)) % 10;
        if left != right {
            return false;
        }
    }
    true
}

fn main() {
    let mut largest_palindrome = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if is_number_base_10_palindrome(product as u128) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    }
    println!("Largest palindrome product of two 3-digit numbers: {}", largest_palindrome);
}
