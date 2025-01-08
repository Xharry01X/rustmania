fn main() {
    println!("Basic function call");
    let result = add_numbers(5, 3);
    println!("5 + 3 = {}", result);

    // Control flow if else
    println!("\nChecking numbers:");
    let number = -7 + 1;
    check_numbers(number);

    // Loops
    println!("\nLoop examples:");
    loop_example();

    // vector operations
    println!("Working with vectors");
    work_with_vectors();
   
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn check_numbers(num: i32) {
    if num > 0 {
        println!("{} is positive", num);
    } else if num < 0 {
        println!("{} is negative", num);
    } else {
        println!("Number is zero");
    }

    let description = if num % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    println!("The number is {}", description);
}

fn loop_example() {
    // While loop
    let mut counter = 0;
    while counter < 3 {
        println!("While loop counter: {}", counter);
        counter += 1;
    }

    // For loop
    println!("\nFor loop:");
    for i in 0..5 {
        println!("For loop count: {}", i);
    }

    // Loop with break and continue
    println!("\nCounting up loop:");
    let mut sum = 0;
    'counting_up: loop {
        println!("Sum is {}", sum);
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if sum >= 20 {
                break 'counting_up;
            }
            remaining -= 1;
            sum += 1;
        }
    }
}

fn work_with_vectors() {
    // Creating and modifying vector
    let mut numbers = vec![1, 2, 3, 4, 5];

    numbers.push(6);
    println!("number is added: {:?}", numbers);

    let popped = numbers.pop();
    println!("popped value: {:?}", popped);
    println!("number aftef pop: {:?}", numbers);

    // accessing elements
    println!("First element: {}", numbers[0]);
    println!("Length: {}", numbers.len());

    // using 
    match numbers.get(3) {
        Some(value) => println!("Second elemnt: {}", value),
        None => println!("No element found"),
    }
}

