// Write a function that finds all numbers between 1 and N that are divisible by 4 and 6. Return their product instead of their sum.



fn main() {
    let number = 20;
    let result = product_numbers(number);
    println!("The product of numbers divisible by 4 and 6 up to {} is {}", number, result);
   
}

fn product_numbers(n: i64) -> i64 {
    let mut product = 1;
    
    for i in 1..=n {
        
        if (i % 4 == 0) && (i % 6 == 0) {
            product *= i;
        }
    }
    
    product
}

