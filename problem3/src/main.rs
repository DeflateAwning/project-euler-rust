fn calc_largest_prime_factor(target: u128) -> u128 {
    let mut n: u128 = target;
    let mut i: u128 = 2;

    while i * i <= n {
        if n % i == 0 {
            n /= i;
        } else {
            i += 1;
        }
    }

    n
}

fn main() {
    let targets: [u128; 3] = [13195, 600_851_475_143, 1_000_000_000_000];

    for target in targets.iter() {
        let largest_prime_factor = calc_largest_prime_factor(*target);
        println!("Largest prime factor of {}: {}", target, largest_prime_factor);
    }
}
