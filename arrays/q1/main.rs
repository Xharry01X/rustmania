// Write a function that takes an array of integers and returns the sum of all elements.

fn main(){

    let array = vec![1,2,3,4,5,6];

    let result = sum_array(&array);

    println!("The sum of the array: {}",result);

}

fn sum_array(array:&Vec<i32>) -> i32 {

  let result = array.iter().sum();

  result

}