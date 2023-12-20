#include <iostream>
#include <stdlib.h>
#include <unordered_set>
#include <vector>
#include <stack>

using namespace std;

int iterativeDfs(const vector<vector<int>>& adj, vector<vector<int>>& adjCopy, 
                vector<int>& maxJumps, vector<bool>& visited, vector<bool>& insertFlag,
                vector<bool>& isCycleStart, vector<int>& cycleId, 
                stack<int>& nodeStack, vector<int>& processedChildren, int source)
{

    nodeStack.push(source);

    while (!nodeStack.empty()) {
        int curr = nodeStack.top();

        insertFlag[curr] = true;
        uint childAmount = adj[curr].size();
        for (uint i = 0; i < childAmount; i++) {
            if (adjCopy[curr][i] < 0)
                continue;

            int neighbour = adj[curr][i];
            if (!visited[neighbour] && insertFlag[neighbour]) {
                cycleId[neighbour]++;
                cycleId[curr] += cycleId[neighbour];
                isCycleStart[neighbour] = true;
                maxJumps[curr] = max(maxJumps[curr], maxJumps[neighbour]);

                adjCopy[curr][i] = -1;
                processedChildren[curr]++;
            } else if (!visited[neighbour]) {
                nodeStack.push(neighbour);
            } else {
                if (cycleId[neighbour]) {
                    if (isCycleStart[curr]) {
                        cycleId[curr] = abs(cycleId[neighbour] - cycleId[curr]);
                    } else {
                        cycleId[curr] += cycleId[neighbour];
                    }
                    maxJumps[curr] = max(maxJumps[curr], maxJumps[neighbour]);
                } else {
                    maxJumps[curr] = max(maxJumps[curr], 1 + maxJumps[neighbour]);
                }
                adjCopy[curr][i] = -1;
                processedChildren[curr]++;
            }
        }
        if (processedChildren[curr] == (int)childAmount) {
            visited[curr] = true;
            nodeStack.pop();
        }
    }
    return maxJumps[source];
}

int findLongestPath(vector<vector<int>>& adj, vector<vector<int>>& adjCopy, int n)
{
    vector<bool> visited(n, false);
    vector<int> maxJumps(n, 0);

    vector<bool> insertFlag(n, false);
    vector<bool> isCycleStart(n, false);
    vector<int> cycleId(n, 0);
    vector<int> processedChildren(n, 0);
    stack<int> nodeStack;
    
    int ans = 0;

    for (int i = 0; i < n; i++) {
        if (!visited[i]) {
            fill(insertFlag.begin(), insertFlag.end(), false);
            fill(isCycleStart.begin(), isCycleStart.end(), false);
            fill(cycleId.begin(), cycleId.end(), 0);
            fill(processedChildren.begin(), processedChildren.end(), 0);

            ans = max(ans, iterativeDfs(adj, adjCopy, maxJumps, visited, insertFlag, 
                        isCycleStart, cycleId, nodeStack, processedChildren, i));
        }
    }
    return ans;
}

void addConnection(vector<vector<int>>& adjList, vector<vector<int>>& adjCopy, int p1, int p2) {
    adjList[p1 - 1].push_back(p2 - 1);
    adjCopy[p1 - 1].push_back(p2 - 1);

}

int main() {
    int n, m; // Number of individuals, Number of relations
    int result = 0;

    scanf("%d %d", &n, &m);

    if (n >= 2 && m >= 0) {
        vector<vector<int>> adjList(n);
        vector<vector<int>> adjCopy(n);
        int p1, p2;

        for (; m > 0; m--) {
            scanf("%d %d", &p1, &p2);
            addConnection(adjList, adjCopy, p1, p2);
        }
        result = findLongestPath(adjList, adjCopy, n);
    }

    printf("%d\n", result);
    return 0;
}
