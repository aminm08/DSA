

fn merge(arr: &mut [i32], s: u32, m: u32, e: u32) {

    println!("Merging s: {}, e: {}, m: {}", s, e, m);
    
    // i = start
    // j = m +1
    // compare until member exists: i < m and j < e
    // put the smaller element in the array 
    // add one to the index of the smaller element(i or j)
    // put the rest of the existing array if there is anything left . for
    // situations like {2,3,4} and {1} which i runs out and 2, 3, 4 should be merged after that

}



fn merge_sort(arr: &mut [i32], s: u32, e: u32) {

    if s == e {
        return 

    }

    
    let m: u32 = (s + e) / 2;

    merge_sort(arr, s, m);
    merge_sort(arr, m+1, e);
    merge(arr, s, m, e);




}





fn main() {


    let mut arr: [i32; 9] = [4, 6, 8, 2, 1, 9, 10, -2, -5];


    merge_sort(&mut arr, 0, 8);

    println!("{:?}", arr);




}


