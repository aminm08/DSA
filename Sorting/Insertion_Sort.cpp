#include <iostream>

using namespace std;


void swap(int arr[], int index1, int index2) {
    
    /*** @brief Swaps two elements in an array.*/
     
    int temp = arr[index1];
    arr[index1] = arr[index2];
    arr[index2] = temp;

}


void insertionSort(int arr[], int n) {

    /**
     * @brief Sorts the array using the insertion sort algorithm.
     * @param arr The array to be sorted.
     * @param n The number of elements in the array.
     */


    for (int i = 1; i < n; i++) {
        
        int current = i; // Index of the member you want to sort

        // Swaps the current element with previous ones until it is in it's place
        while (arr[current-1] > arr[current] && current > 0){

            swap(arr, current, current-1);
            current --;

        }
    }
}


void printArray(const int arr[], int n) {
    
    for (int i = 0; i < n; ++i) {
        cout << arr[i] << " ";
    }
    cout << endl;
}


int main(){


    int arr[] = {5, 2, 7, 1, 9, 14, 3, 0, 4};

    int n = sizeof(arr) / sizeof(arr[0]);


    printArray(arr, n);

    insertionSort(arr, n);

    printArray(arr, n);

    return 0;
}