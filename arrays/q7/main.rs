fn rotate_array(arr: &mut [i32], k: usize) {
    let len = arr.len(); // Get array length
    if len == 0 {
        return;
    }

    // Handle cases where k is greater than the length of the array
    let k = k % len;
    arr.reverse();
    arr[..k].reverse();
    arr[k..].reverse();
}

fn main() {
    let mut array = vec![1, 2, 3, 4, 5];
    let k = 2;

    rotate_array(&mut array, k);
  
    println!("{:?}", array);
}
