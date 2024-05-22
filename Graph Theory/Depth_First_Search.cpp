#include <iostream>
#include <vector>

using namespace std;


void DFS(vector<vector<int>>& adjList, vector<bool>& visited, int v){

     /**
     * @brief Performs a depth-first search (DFS) on a graph.
     * 
     * @param adjList Adjacency list representation of the graph.
     * @param visited Vector to keep track of visited nodes.
     * @param v Starting vertex for the BFS.
     */

    visited[v] = true;
    cout << v << endl;

    for (int neighbour: adjList[v]){
        
        if (!visited[neighbour]){
            DFS(adjList, visited, neighbour);
        }
    }
}


int main(){

    int V = 3; // number of vertices

    vector<vector<int>> adjList = {{1, 2}, {0}, {0}};
    vector<bool> visited(V, false);

    

    DFS(adjList, visited, 0);

    return 0;

}