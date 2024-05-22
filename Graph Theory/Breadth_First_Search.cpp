#include <iostream>
#include <vector>
#include <queue>

using namespace std;


void BFS(vector<vector<int>>& adjList, vector<bool>& visited, int v){

    /**
     * @brief Performs a breadth-first search (BFS) on a graph.
     * 
     * @param adjList Adjacency list representation of the graph.
     * @param visited Vector to keep track of visited nodes.
     * @param v Starting vertex for the BFS.
     */

    queue<int> q;

    visited[v] = true;

    q.push(v);

    while(!q.empty()){

        int node = q.front();
        q.pop();
        cout << node << endl;

        for (int neighbour : adjList[node]){

            if(!visited[neighbour]){
                visited[neighbour] = true;
                q.push(neighbour);
                
            }
        }
    }
}


int main(){

    int V = 3; // number of vertices

    vector<vector<int>> adjList = {{1, 2}, {0, 3, 4}, {0, 5}, {1}, {1}, {2}};
    vector<bool> visited(V, false);

    BFS(adjList, visited, 0);

    return 0;
}