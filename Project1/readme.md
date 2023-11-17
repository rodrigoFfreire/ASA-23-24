# Knapsack w/repetition


$$
K(W) = max(K(W) - w_i) + v_i \;\;\;\; \text if \; w \leq w_i
$$


First implementation:
```c++
int knapsack(int* weights, int* values, int max_weight) {
    int max = 0;
    for (i = 1; i <= W; i++) {
        if (weights[i] <= max_weight) {
            int aux = knapsack(weights, values, max_weight - weights[i])
            if (max < aux) {
                max = aux;
            }
        }
    }
    return max;
}
```
Time complexity: $ O(n^W) $ \
Space complexity: $ O(1) $
```diff
- This is a bad implementation
```
<br></br>

Second implementation:
```c++
// `W` is the knapsacks max weight
int knapsack(int* weights, int* values, int W) {
    int k[W + 1];
    memset(k, 0, W + 1);

    for (int w = 1; w <= W; w++) {
        for (int i = 1; values.len(); i++) {
            int add = k[w - weights[i]] + values[i];

            if (weights[i] <= w && k[w] < add) {
                k[w] = add;
            }
        }
    }
    return k[W];
}
```
Time complexity: $ Θ(W\cdot n) $ \
Space complexity: $ O(W) $

<br></br>

Third implementation (Memoization):
```c++
// got lazy :p
```
Time complexity: $ O(W \cdot n) $ \
Space complexity: $ O(W \cdot n)$

<br></br>

# 0-1 Knapsack (no repetition)
$$
K(W) = 
\begin{cases}
K(W, i - 1) & \quad \text{when } w_i > W \\
\\
max(K(W - w_i, i - 1) + v_i,\;\; K(W, i - 1)) & \quad \text{otherwise}
\end{cases}
$$

