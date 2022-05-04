fn main() {
    let mut sum = 0;

    for num in 0..1000 {
        sum += if ((num % 3) == 0) || ((num % 5) == 0) {
            num
        } else {
            0
        };
    }

    println!("abs {}", sum);
}
