// write a program that reverse string




fn main(){
    let text = "Hello world";

    let reversed = reverse_string(text);

    println!("Original: {}", text);
    println!("Reversed: {}", reversed);
}

fn reverse_string(input: &str) -> String {
    // String::with_capacity(n) creates a new String with space pre-allocated for n characters
    let mut result = String::with_capacity(input.len());
    for ch in input.chars().rev() {
        result.push(ch)
    }

    return result
}
