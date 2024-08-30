use std::fmt::Debug;
use std::cmp::PartialOrd;
fn partition<T: Debug + PartialOrd + Copy>(arr: &mut [T]) -> usize {

    let mut l = 0;
    let mut h = arr.len() - 1;

    let pivot = arr[l];

    loop {

        while arr[l] < pivot {
            l += 1;
        }


        while arr[h] > pivot {
            
            h -= 1;
        }

        if l >= h {
            return h;
        }

        arr.swap(h, l);
        println!("{:?}", arr);
    }
}



//fn quick_sort<T: Debug + PartialOrd + Copy>(arr: &[T]) {
 //   if arr.len() > 1 {
   //     let pivot = partition(arr);
     //   quick_sort(arr[..pivot]);
       // quick_sort(arr[pivot..]);

    //}

//}



fn main() {

    let mut arr = [6, 2, 1, 3];

    let j = partition(&mut arr);
    

    println!("{:?}, {}", arr, j);


}
