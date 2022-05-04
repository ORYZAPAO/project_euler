fn main() {
    let mut n0: u32 = 1;
    let mut n1: u32 = 2;
    let mut fib: u32 = 0;

    let mut sum: u32 = 2;

    loop {
        fib = n0 + n1;

        n0 = n1;
        n1 = fib;

        if fib >= 400_0000 {
            break;
        }

        sum += if (fib % 2) == 0 { fib } else { 0 };
    }

    println!("ans {} ", sum);
}
