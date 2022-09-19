
// 実行時間20min
//-->100000
//-->200000
//-->300000
//-->400000
//-->500000
//-->600000
//-->700000
//-->800000
//-->900000
//-->1000000
//-->1100000
//-->1200000
//-->1300000
//-->1400000
//-->1500000
//-->1600000
//-->1700000
//-->1800000
//-->1900000
//Ans = 142913828922



fn main() {
    let mut sum = 0_i64;

    for num in 2..200_0000i64{
    //for num in 2..30i64{

        // 素数判定
        let mut is_prime = true;
        'inner:for i in 2..(num/2)+1 {
            if num % i == 0 {
                is_prime = false;
                break 'inner;
            }
        }

        // 素数だったら、加算
        if is_prime == true {
            //println!("{}",num); 
            sum += num;
        }

        // 進捗表示
        if num % 100000 == 0{
            println!("-->{}",num);
        }
    }

    // 最終結果
    println!("Ans = {}",sum);
}



