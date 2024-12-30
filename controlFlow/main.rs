fn main(){
  
  println!("Basic function call");
  let result = add_numbers(5,3);
  println!("5 + 3 = {}", result);

// control fow if else
println!("Checking number");
let number = -7 + 1;
check_numbers(number);

// loops
println!("Loop example");
loop_example();





}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn check_numbers(num: i32) {
    if num > 0 {
        println!("{} is positive", num);
    }else if num < 0{
        println!("{} is nagative", num);
    }else {
        println!("Number is zero");
    }

    let description = if num % 2 == 0 {
        "even"
    }else {
        "odd"
    };
    println!("The number is {}", description);
}

fn loop_example(){
    // while loop
    let mut counter = 0;
    while counter < 3 {
        println!("While loop counter: {}", counter);
        counter +=1;
    }

    // for loop
   for i in 0..5 {
    println!("for loop count {}",i);
   }
  
   // loop with break and continue
   let mut sum = 0;
   'counting_up: loop {
    println!("Sum is {}", sum);
   
    let mut remaining = 10;

    loop {
        if remaining == 9 {
            break;
        }
        if sum >= 20 {
            break 'counting_up
        }
        remaining -=1;
        sum +=1;
    }

   }

}