fn main() {
    
   // Regular function - can't access "user_name"

   fn regular_tweet() {
    // this wouldn't not work: println!("Hello,{}", user_name);
   }

   // closure helps to access "user_name"
   let user_name = "Alice";
   let closure_greet = || println!("Hello, {}", user_name);

   closure_greet();

}
