// 回文数の判定
//fn is_palindromic(val: u32) -> bool{
fn is_kaibun(_val: i32) -> bool {
    let mut val = _val;
    let mut num_list = Vec::new();
    
    // 数値桁
    let mut digits = 0; // 数値桁数のカウンタ
    while val > 0 {
        let num = val % 10;
        num_list.push(num);

        val = (val / 10) as i32;
        digits += 1;
    }


    //
    let mut high = (digits - 1) as usize;
    let mut low = 0usize;

    let mut result = true;
    loop {
        if high < low { break; }
        if ((high - low) == 0){ break; };

        if &num_list[high] != &num_list[low] {
            result = false;
            break;
        };

        high -= 1;
        low += 1;
    }
    
    result
}

fn main() {
    let mut max = 0;
    
    for num0 in 100..999{
        for num1 in 100..999{
            let kazu = num0 * num1;
            if is_kaibun(kazu) {
                if( max < kazu ){ max = kazu; }

                println!("{} x {}  == {}",num0, num1, kazu);
            }
        }
    }

    println!("Result {}",max);
}
