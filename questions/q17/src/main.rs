fn main() {
    
  let name = "harry";
  let greet = || println!("Hello my name is {}", name);

  greet();
  greet();


}

//  friendly robot that remembers your name and can say hello anytime!

// No parameters
// let greet = || println!("Hello!");

// // One parameter
// let greet = |name| println!("Hello, {}!", name);

// // Two parameters
// let greet = |first, last| println!("Hello, {} {}!", first, last);