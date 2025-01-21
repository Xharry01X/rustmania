// finding largest element in arrays

// In Rust, <T: PartialOrd> is a way of saying, "Hey, I want to compare things that can be ordered or compared by size." The T represents any toy (or thing), and PartialOrd is like saying, "This toy can be compared to others to see which one is bigger."
// In simple words: unwrap() is saying, "I’m sure this has a value," but if it doesn't, it will cause an error.
fn main() {
    let num = [1, 9, 7, 5, 3, 6, 10];

    let largest = *num.iter().max().unwrap();

    println!("The largest number inside the array: {}", largest);
}

