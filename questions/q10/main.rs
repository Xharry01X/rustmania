// print number like this 
// 1
// 22
// 333
// 4444
// 55555

fn main(){

    let number = 5;

    print_numbers(number);

}

fn print_numbers(n:i32) {
    for i in 1..=n {
        for _ in 0..i {
            print!("{}",i);
        }
        println!();
    }
}

