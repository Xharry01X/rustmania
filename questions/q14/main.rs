// Make a function that finds the sum of all numbers that are multiples of both 3 and 5 up to a given number.

fn main(){

    let number = 5;

   let result =  sum_number(number);
   println!("The sum is {}", result);

}


fn sum_number(n: i32) -> i32{
    let mut sum = 0;
   
    for i in 1..=n {
        sum += i;
    }

    return sum;


}
