fn main() {

    'loop_end: for a in 1..998{
        for b in 1..998{
            for c in 1..998{
                if a+b+c == 1000 {
                    if (a*a + b*b) == (c*c) {
                        println!("a={}  b={}  c={} ",a,b,c);

                        println!("  a + b + c = 1000");
                        println!("  a^2({}) + b^2({}) = c^2({})",a*a,b*b,c*c);
                        println!("  a * b * c = {}",a*b*c);
                        break 'loop_end;
                    }
                }

            }
        }
    }


}




