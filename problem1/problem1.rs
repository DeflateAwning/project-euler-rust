fn main() {
    let mut sum = 0;

    for number in 1..1000 {
        if number % 3 == 0 || number % 5 == 0 {
            sum += number;
        }
    }

    println!("The sum of all the multiples of 3 or 5 below 1000 is: {}", sum);
}
