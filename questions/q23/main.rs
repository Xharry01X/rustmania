// Example 2: Closure that takes a parameter

fn main(){

    let number = |x| println!("Number is {}",x);
    example2(print_number(number,5));

}


let print_number(f: impl Fn()){


    
}