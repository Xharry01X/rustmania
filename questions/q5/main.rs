


fn main() {
    for number in 1..=20 {
        check_number(number);
    }
}

fn check_number(n: i32) {
    
    if n > 0 {
        print!("number is positive: {}", n);
    }else if n == 0{
       print!("number is zero: {}", n);
    }else {
        print!("number is negative: {}", n);
    }

}