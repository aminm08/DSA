#include <iostream>

using namespace std;



int main(){
    int targetNumber = -1;
    int arrayLength = 10;
    int myArray[arrayLength] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};


    int start = 0;
    int end = arrayLength - 1;
    int mid = 0;

    while (start <= end){

        mid = start + (start - end) / 2;


        if (myArray[mid] > targetNumber){
            end = mid - 1; // target array is in the left half
        }

        else if (myArray[mid] < targetNumber){
            start = mid + 1; // Target number is in the right half
        }

        else {
            cout << targetNumber; // Target number is found at mid index
            return 0;
        }
    }
    cout << "was not found";


    return 0;
}