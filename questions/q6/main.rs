

fn main(){
    let number = 5;
    
    let result = factorial(number);

    println!("factorial of {} a number is {} ", number,result);
    
}

fn factorial(n:u32) -> u32 {
  
let mut result = 1;
for i in 1..=n {
    result *= i;
}
result

}