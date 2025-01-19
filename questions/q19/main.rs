// takes one parameter and returns it double

fn main(){

    let double = |x| x + x;

    let result = double(5);

    println!("Doubled: {}", result);

    let tripple = |x| x * 3;

    let result = tripple(3);

    println!("{}", result);


}