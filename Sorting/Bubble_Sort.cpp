#include <iostream>
using namespace std;


void swap(int arr[], int index1, int index2) {
    
    /*** @brief Swaps two elements in an array.*/
     
    int temp = arr[index1];
    arr[index1] = arr[index2];
    arr[index2] = temp;

}

void bubbleSort(int arr[], int n) {

    /**
     * @brief Sorts an array of integers using the Bubble Sort algorithm.
     * 
     * @param arr Array to be sorted.
     * @param n Size of the array.
     */

    bool swapped;

    for(int i = 0; i < n - 1; ++i) {

        swapped = false;

        for(int j = 0; j < n - i - 1; ++j) {

            if(arr[j] > arr[j+1]){
               swap(arr, j, j+1);
               swapped = true;
            }
        }
        
        // Stops if it is already sorted
        if (!swapped) {
            break;
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

    int arr[] = {2, 8, 5, 10, 7, 9, 1};

    int n = sizeof(arr) / sizeof(arr[0]); 

    printArray(arr, n);

    bubbleSort(arr, n);

    printArray(arr, n);

    return 0;
}