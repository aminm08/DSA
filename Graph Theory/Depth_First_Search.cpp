#include <iostream>
#include <vector>
#include <list>

using namespace std;


void DFS(vector<list<int>>& AL, vector<bool>& visited, int v){

    // Depth-First Search (DFS) function to traverse a graph represented by an adjacency list
    // Parameters:
    //   - AL: Adjacency list representing the graph
    //   - visited: Vector to keep track of visited vertices
    //   - v: Current vertex being visited

    visited[v] = true;
    cout << v << endl;

    for (int neighbour: AL[v]){
        
        if (!visited[neighbour]){
            DFS(AL, visited, neighbour);
        }
    }

}


int main(){

    int V = 3; // number of vertices

    vector<list<int>> AL(V);
    vector<bool> visited(V, false);

    AL = {{1, 2}, {0}, {0}};

    DFS(AL, visited, 0);

    return 0;


}