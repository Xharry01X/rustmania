// write a program to find a prime number, add a square root optimization.


fn main(){
    let number = 37;

    if prime_number(number){
        println!("{} is a prime number", number);
    }else {
        println!("{} is not a prime number", number);
    }


}

fn prime_number(n: u32) -> bool {

   if n < 2 {
    return false;
   }
   for i in 2..=((n as f64).sqrt() as u32){
    if n % i == 0 {
        return false;
    }
   }
   return true
}