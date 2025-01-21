// You only need Vec if you're working with dynamically-sized arrays
// Generic Function (<T>): The function is generic, so it works for any type of array ([T]), as long as the type T implements Copy.


fn main(){
    // mut number is because it is going to change

    let mut number = [1,2,3,4,5];

    reverse_array(&mut number);
    println!("The reverse array is: {:?}", number);

    let mut chars = ['a','b','c','d'];

    chars_array(&mut chars);

    println!("Reversed character are: {:?}",chars)


}

fn reverse_array<T>(array:&mut [T]){

    let mut left = 0;
    let mut right =array.len() - 1;

    while left < right {
        array.swap(left, right);

        left += 1;
        right -= 1;
    }

}

fn chars_array<T>(array:&mut [T]){
    let mut left = 0;
    let mut right = array.len() - 1;

    while left < right {
        array.swap(left,right);
        left += 1;
        right -= 1;

    }
}


