fn main() {
    let mut arr = [6, 2, 3, 2, 5, 1, 9, -10, 111];

    for i in 0..arr.len() {
        let mut min_value = &arr[i];
        let mut min_index = i;

        for j in i..arr.len() {
            if &arr[j] < min_value {
                min_index = j;
                min_value = &arr[j];
            }
        }
        arr.swap(i, min_index);
    }
    println!("{:?}", arr);
}
