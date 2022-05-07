
// 最初の10個の自然数について, その二乗の和は,
//   12 + 22 + ... + 102 = 385
//
// 最初の10個の自然数について, その和の二乗は,
//   (1 + 2 + ... + 10)2 = 3025
//
// これらの数の差は 3025 - 385 = 2640 となる.
//
// 同様にして, 最初の100個の自然数について二乗の和と和の二乗の差を求めよ.


fn sum_pow_2(n : u32) -> u32{
    // n^2乗の総和の公式
    let sum = n * (n + 1) * ((2*n) + 1) / 6;

    sum
}


fn sum_pow_1(n : u32) -> u32{
    // nの総和の公式
    let sum = n * (n + 1) / 2;

    // 2乗(sum square)
    sum.pow(2)
}

fn main() {
    let a = sum_pow_2(10);
    let b = sum_pow_1(10);    
    println!("(sum n)**2  {}", a);
    println!("(sum n**2)  {}", b);
    println!("diff        {}", b-a);
    println!();
        
    let a = sum_pow_2(100);
    let b = sum_pow_1(100);
    println!("(sum n)**2  {}", a);
    println!("(sum n**2)  {}", b);
    println!("diff        {}", b-a);
    println!();
    println!("ans         {}", b-a);
}
