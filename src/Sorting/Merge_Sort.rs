

fn merge(arr: &mut [i32], s: usize, m: usize, e: usize) {



    let mut i = 0usize;
    let mut j = 0usize;
    let mut k = 0usize;

    let n1 = m - s + 1;
    let n2 = e - m;
    let mut left = vec![0; n1];
    let mut right = vec![0; n2];
    
    left[..].clone_from_slice(&arr[..n1]);
    println!("{:?}", left);
    if n2 + n1 == arr.len() {

        right[..].clone_from_slice(&arr[n1..]);
    } else {
        right[..].clone_from_slice(&arr[n1..(n2+1)]);
    }
    println!("{:?}", right);
    while i < n1 && j < n2 {

        if left[i] < right[j] {

            arr[k] = left[i];    
            i += 1;
        } else {

            arr[k] = right[j]; 
            j += 1;
        }

        k += 1;

    }


    while i < n1 {

        arr[k] = left[i];
        k += 1;
        i += 1;
    }

    while j < n2 {

        arr[k] = right[j];
        k += 1;
        j += 1;
    }

    

}



fn merge_sort(arr: &mut [i32], s: usize, e: usize) {

    if s == e {
        return 

    }

    
    let m: usize = (s + e) / 2;

    merge_sort(arr, s, m);
    merge_sort(arr, m+1, e);
    merge(arr, s, m, e);




}





fn main() {


    let mut arr: [i32; 9] = [5, 8,1, 0, 9, -10, 7, 4, -3];


    merge_sort(&mut arr, 0, 9);

    println!("{:?}", arr);




}


