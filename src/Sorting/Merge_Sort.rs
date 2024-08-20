

fn merge(arr: &mut [i32], s: usize, m: usize, e: usize) {

    println!("Merging s: {}, e: {}, m: {}", s, e, m);
    

    let mut i = s;
    let mut j = m + 1;
    let mut newarr : Vec<i32> = Vec::new();
    while i <= m && j <= e {
        println!("i = {}, j = {}", i, j);
        if arr[i] > arr[j] {
            
            newarr.push(arr[j]);
            j += 1;
        } else if arr[i] < arr[j] {

            newarr.push(arr[i]);
            i += 1;
        }

    }

    if i <= m {

        while i <= m {

            newarr.push(arr[i]);
            i += 1;
        }

    } else if j <= e {


        while j <= e {
            
            newarr.push(arr[j]);
            j += 1;
        }

    }


    println!("{:?}", newarr);

    



    // i = start
    // j = m +1
    // compare until member exists: i < m and j < e
    // put the smaller element in the array 
    // add one to the index of the smaller element(i or j)
    // put the rest of the existing array if there is anything left . for
    // situations like {2,3,4} and {1} which i runs out and 2, 3, 4 should be merged after that

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


    let mut arr: [i32; 3] = [8, 6, 4];


    merge_sort(&mut arr, 0, 2);

    println!("{:?}", arr);




}


