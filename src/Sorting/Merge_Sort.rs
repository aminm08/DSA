

fn merge(arr: &mut [i32], s: usize, m: usize, e: usize) {

    let left = arr[s..=m].to_vec();
    let right = arr[m+1..=e].to_vec();

    let (mut l, mut r, mut i) = (0, 0, s);


    while l < left.len() && r < right.len() {

        if left[l] < right[r] {

            arr[i] = left[l];    
            l += 1;
        } else {

            arr[i] = right[r]; 
            r += 1;
        }

        i += 1;

    }

    while l < left.len() {

        arr[i] = left[l];
        i += 1;
        l += 1;
    }

    while r < right.len() {

        arr[i] = right[r];
        i += 1;
        r += 1;
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

    let mut arr = [87, 56, -10, 2, 8, 19, 25, -9, -25, 1, 12];

    merge_sort(&mut arr, 0, 10);

    println!("{:?}", arr);

}


