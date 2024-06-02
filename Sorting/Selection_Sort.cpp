#include <iostream>
using namespace std;


void swap(int arr[], int index1, int index2) {
    
    /*** @brief Swaps two elements in an array.*/
     
    int temp = arr[index1];
    arr[index1] = arr[index2];
    arr[index2] = temp;

}


void selectionSort(int arr[], int n) {

    /**
     * @brief Sorts an array using the selection sort algorithm.
     * @param arr The array to sort.
     * @param n The number of elements in the array.
     */

    int minIndex ;

    for(int i = 0; i < n - 1; ++i) {

        minIndex  = i;

        for (int j = i + 1; j < n - 1; ++j) {

            if(arr[j] < arr[minIndex ]) {
                minIndex  = j;
            }
        }

        // Don't swap if the min is the first element
        if (minIndex  != i) {
            swap(arr, minIndex , i);
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

    int arr[] = {5, 3, 4, 8, 1, 2, 6};
    
    int n = sizeof(arr) / sizeof(arr[0]);

    printArray(arr, n);

    selectionSort(arr, n);

    printArray(arr, n);

    return 0;
}