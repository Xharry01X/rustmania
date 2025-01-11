fn main(){
let number = 5;

 multiplication(number);



}

fn multiplication(n: i32) {
   for i in 1..=10 {
    println!("{} x {} = {}", n , i, n * i);
   }
}

