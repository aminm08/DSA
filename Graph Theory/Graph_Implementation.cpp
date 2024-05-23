#include <iostream>
#include <vector>
using namespace std;




void addEdge(vector<vector<int>>& adjList, int v, int w){
    adjList[v].push_back(w);
    adjList[w].push_back(v);
}

void printGraph(const vector<vector<int>>& adjList){

    for (int i = 0; i < adjList.size(); i++){
        cout << "vertex" << i << ":\n";
        for (int j : adjList[i])
            cout << j << ", ";
        cout << endl;
    }
}

int main(){

    int V = 3; // Number of vertices
    vector<vector<int>> adjList(V);
    addEdge(adjList, 0, 1);
    addEdge(adjList, 0, 2);


    printGraph(adjList);

    return 0;


}