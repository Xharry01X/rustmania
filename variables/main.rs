
fn main(){

// default immubility
let x = 20;
println!("Immutbility example : {x}");


// checking mutablity
let mut y = 16;
println!("value before y changes: {y}");

y =20;
println!("value after adding mut: {y}");


// Constants require type annotation and are computed at compile time
const MAX_POINTS: u32 = 100_000;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
println!("Constants - Max Points: {MAX_POINTS}, Three Hours: {THREE_HOURS_IN_SECONDS}");

// Basic shadowing
let z = 5;
println!("value of original: {z}");

let z = z * 2;
println!("shadowed: {z}");

let z = z * 3;
println!("shadowed again: {z}");

// shadowing  with scope
let a = 5;
println!("outside the value: {a}");

{
    let a = a * 2;  // shadowing within the scope
    println!("Inner a is: {a}");

    let a = a + 1;
    println!("Modified inner a is : {a}");

}

println!("outer value of a : {a}");


// shadowing with type change
let spaces = "    ";  // spaces is a string
    println!("Spaces as string: '{spaces}'");
    
    let spaces = spaces.len();  
    println!("Spaces as length: {spaces}");

    // The following would NOT work with mut:
    // let mut spaces_mut = "   ";
    // spaces_mut = spaces_mut.len();  // Error: mismatched types


    // complex shadow
    let value = "heyy there";
    println!("orignal value: {value}");

    let value = 42;
    println!("Shadowed to a number: {value}");

    let value = true;
    println!("Shadowed to a number: {value}");

    {
        let value = "back to string"; 
        println!("Inner scope value: {value}");
        
        let value = value.len();  
        println!("Inner scope length: {value}");
    }
    
    println!("Outer scope value still boolean: {value}");

}