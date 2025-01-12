// print upto 20 prime number 

fn main(){

  let number = 20;

  print_first_n_primes(number);

}

fn print_first_n_primes(n: i32) {
 let mut prime_found = 0; // counter for prime number found
 let mut candidate = 2;


 while prime_found < n {
    if is_prime(candidate){
        println!("{}", candidate);
        prime_found += 1;
    }
    candidate += 1;
 }




}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32){
        if n % i == 0 {
            return false;
        }
    }
    return true;
}



