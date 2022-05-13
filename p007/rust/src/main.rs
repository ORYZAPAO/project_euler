
fn prime(number : u32) -> bool{
    for i in 2..number-1 {
        if number % i == 0 {
            return false;
        }
    }

    return true;
}


fn main() {
    let mut num = 2u32;
    let mut ct  = 0u32;

    loop{
        if prime(num) {
            //println!("{} -> {}", ct, num);
            ct  += 1;

            if ct == 10001 {
                break;
            }
        }

        //
        num += 1;
    }

    println!("{} -> {}", ct, num);    
}
