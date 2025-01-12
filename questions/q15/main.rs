// // Make a function that finds the sum of all numbers that are multiples of both 3 and 5 up to a given number. 

fn main (){
    let limit = 1000;
  
   let sum = sum_of_multiples(limit);

   println!("The sum of multiples of 3 and 5 upto {} is {}", limit, sum);

}

fn sum_of_multiples(n: i32) -> i32 {
    let mut sum = 0;

    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            sum += i;
        }
    }
    return sum;
}
