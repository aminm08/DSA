


fn insertion_sort(arr: &mut [i32]) {

    for i in 1..arr.len() {

        let mut j = i;
        let key = arr[i];

        while j > 0 && arr[j-1] > key {
            
            arr[j] = arr[j-1];
            j -= 1;

        }    

        arr[j] = key;
    }
}



fn main() {


    let mut arr: [i32; 7] = [9, 4, 3, 6, 7, 1, 0];
        
    insertion_sort(&mut arr);

    println!("{:?}", arr);

}
