fn main() {
    let word = "education";

    let count = count_vowels(word);

    println!("The word '{}' contains {} vowels.", word, count);
}

fn count_vowels(word: &str) -> usize{
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    word.chars().filter(|c| vowels.contains(c)).count()

}