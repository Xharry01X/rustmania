// write a program that print all even number from 1 to 20 using for loop


fn main() {
   
   for n in 1..20 {
    if n % 2 == 0 {
     println!("All even number: {}", n);
    }else {
     println!("All are odd number: {}", n);
    }
   }
}