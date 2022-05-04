use std::cmp::max;

/// SPF(Smallest Prime Factor)
///
/// 最小の素因数
///
fn spf(val: u64) -> u64 {
    let mut n: u64 = 2u64;

    loop {
        if val % n == 0 {
            break;
        }
        n += 1;

        if val == n {
            break;
        }
    }

    n
}

fn main() {
    let mut n: u64 = 600851475143u64;
    let mut spf_n: u64;

    loop {
        spf_n = spf(n);

        n = n / spf_n;
        if n == 1 {
            break;
        }

        println!("->{}", n);
    }
    println!("ans {}", spf_n);
}
