fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];  // creating vector
    let result = some_operation(numbers);
    println!("{}", result);
}

fn some_operation(nums: Vec<i32>) -> i32 {
    nums.iter()
        .filter(|&&x| x % 2 == 0)  // keep only even numbers
        .map(|&x| x * 3)           // multiply each by 3
        .sum()                     // sum all numbers
}