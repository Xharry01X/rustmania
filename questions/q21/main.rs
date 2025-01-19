

fn main() {

    let mut counter = 0;

    let mut counter_print = || {
         counter += 1;
         println!("{}", counter);

    };

    counter_print();
    counter_print();
    counter_print();



    

   

}