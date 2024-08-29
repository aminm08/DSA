use std::fmt::Debug;

fn partition<T: Debug>(arr: &[T]) {

    println!("{:?}", arr);

}



fn quick_sort<T: Debug>(arr: &[T]) {
    println!("{:?}", arr);
    if arr.len() > 1 {
        let pivot = arr.len() / 2;
        quick_sort(&arr[..pivot]);
        quick_sort(&arr[pivot..]);

    }

}



fn main() {

    let arr = [5, 4, 1, 8, 2, 7, 3, 9];

    quick_sort(&arr);



}
