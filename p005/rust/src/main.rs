/// Problem 5 「最小の倍数」 
/// 2520 は 1 から 10 の数字の全ての整数で割り切れる数字であり, そのような数字の中では最小の値である.
/// 
/// では, 1 から 20 までの整数全てで割り切れる数字の中で最小の正の数はいくらになるか

fn mod_chk(val : u32) -> bool{
    let max = 20;

    for i in 0..max {
        if val % (max-i) != 0 { return false; }
    }

    true
}

fn main() {
    let mut num = 1;

    loop{
        if mod_chk(num) {
            break;
        }
        //print!("{} ", num);
        num += 1;
    }

    println!("ans = {}", num);
    
}
