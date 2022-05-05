//fn is_palindromic(val: u32) -> bool{
fn is_kaibun(_val: i32) -> bool {
    let mut val = _val;
    let mut num_list = Vec::new();

    // 桁数を求める
    let digits = f32::ceil(f32::log10(val as f32)) as i32;

    // 一桁ずつ分離
    let mut n = digits;
    while n > 0 {
        let num = val % 10;
        num_list.push(num);

        val = (val / 10) as i32;
        n -= 1;
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
