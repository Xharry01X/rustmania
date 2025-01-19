
// <F> - This is a generic type parameter. F will be the type of our closure

fn run_this(f: impl Fn()){
    f();
}


fn main() {

  let say_hi = || println!("Hello");

  run_this(say_hi);


 }