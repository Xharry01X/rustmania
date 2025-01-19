

fn main() {

    let multiplier = 10;
    let tax_rate = 0.2;

    let calculate_subtotal = | price: f64 | {
     
     let subtotal = price * multiplier as f64;

     let tax = subtotal * tax_rate;

     subtotal + tax
      

     
    };

    println!("{}",calculate_subtotal(5.0));


    
}


