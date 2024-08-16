

fn swap(arr: &mut [i32], index1: usize, index2: usize) {

    let temp = arr[index1];

    arr[index1] = arr[index2];
    arr[index2] = temp;
    

}


fn bubble_sort(arr: &mut [i32]) {
    

    let n = arr.len();

    for i in 0..(n-1) {
        
        let mut swapped = false;
        
        for j in 0..(n-i-1) {

           if arr[j] > arr[j+1] {

                swap(arr, j, j+1);
                swapped = true;

           } 

        }


        if swapped == false {

            break;

        }

    }

}




fn main() {


    let mut arr: [i32; 8] = [1, 5, 8, 2, 3, 4, 8, -5];


    bubble_sort(&mut arr);
    
    println!("{:?}", arr);



}
