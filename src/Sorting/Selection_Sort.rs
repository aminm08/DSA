


fn find_min_index(arr: &[i32]) -> usize {
    
    let mut min_index: usize = 0;
    for j in 1..arr.len() {
        if arr[j] < arr[min_index] {
        min_index = j;

        }
    }

    min_index
}


fn selection_sort(arr: &mut [i32]) {

    for i in 0..arr.len() {

        let min_index = find_min_index(&arr[i..]) + i;

        if min_index != i {

            arr.swap(i, min_index);

        }

    } 
}


fn main() {


    let mut arr: [i32; 6] = [5, 7, 8, 3, 4, 1];

    selection_sort(&mut arr);

    println!("{:?}", arr);


}
