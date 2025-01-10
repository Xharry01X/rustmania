// Write code that takes a student's score (0-100) and prints their grade: A (90-100), B (80-89), C (70-79), D (60-69), and F (below 60).

fn get_grade(score: i32) -> char {
   if score < 0 || score > 100 {
    panic!("Score must be between 0 and 100");
   }

   match score {
    90..=100 => 'A',
    80..=89 => 'B',
    70..=79 => 'C',
    60..=69 => 'D',
    _ => 'F'
}
}

  fn main(){
   
   let test_score = vec![66, 24, 32, 90];

   for score in test_score {
    println!("Score: {}, Grade: {}", score, get_grade(score));
   }

  }