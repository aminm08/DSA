#include <iostream>

using namespace std;


bool linearSeach(int arr[], int n, int target) {

    /**
     * @brief Performs a linear search on the given array to find the target value. 
     * @param arr The array in which to search for the target value.
     * @param n The number of elements in the array.
     * @param target The value to search for in the array. 
     * @note Time complexity: O(n)
     */

    for (int i = 0; i < n; ++i) {
        if (arr[i] == target){
            return true;
        }
    }
    
    return false;
    
}


int main(){

    int arr[] = {5, 1, 8, 2, 5, 4, 9, 10};

    int n = sizeof(arr) / sizeof(arr[0]);
    
    int searchResult = linearSeach(arr, n, 14);


    if (searchResult){
        cout << "Item is in the list";
    }

    else {
        cout << "item is not in the list";
    }

    return 0;
}