// write a program takes a number and prints postive if it's greater than 0, "Negative" if it less than and zero if it's equal to zero


fn main(){
   

    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
}