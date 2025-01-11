fn main(){
    let number = 5;

    print_star(number);
}

fn print_star(n:i32){

  for i in 1..=n {
    for _ in 0..i {
        print!("*");
    }
    println!();
  }

}