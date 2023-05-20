# INT102 Final Notes

## Asymptotic Idea

### Big-O Notation

#### Definition of Big-O Notation

Big-O describes the upper bound in the worst-case scenario.

#### Formal Definition of Big-O Notation

Given two functions $f(n)$ and $g(n)$, we say that $f(n)$ is $O(g(n))$ if there exist constants $c > 0$ and $n_0 \geq 0$ such that for all $n \geq n_0$, we have $f(n) \leq c \cdot g(n)$.

This means that $f(n)$ grows no faster than $g(n)$, up to a constant multiple, for large enough values of $n$.

#### Big O Complexity Hierarchy

1. $O(1)$ - Constant time complexity.
2. $O(\log n)$ - Logarithmic time complexity.
3. $O(n)$ - Linear time complexity.
4. $O(n \log n)$ - Linearithmic time complexity.
5. $O(n^2)$ - Quadratic time complexity.
6. $O(n^3)$ - Cubic time complexity.
7. $O(n^k)$ for some constant $k$ - Polynomial time.
8. $O(2^n)$ - Exponential time complexity.
9. $O(n!)$ - Factorial time complexity.

### Computational Complexity Theory

#### Complexity Reduction

A **reduction** from problem A to problem B is an algorithm that transforms any instance of problem A into an instance of problem B. The solution to the instance of problem B can then be used to solve the original instance of problem A.

If we can reduce problem A to problem B in polynomial time, it means that problem B is at least as hard as problem A.

#### Decision/Optimization problems

- A **decision problem** is a problem with a yes/no answer.
- An **optimization problem** is a problem of finding the best solution from all feasible solutions.
- **From Optimization to Decision**: Any optimization problem can be converted into a related decision problem. This is often done by adding a parameter k and asking if the optimal solution is less than, equal to, or greater than k.

#### Complexity Classes

- The class **P** consists of decision problems that are solvable in polynomial time.
  - Examples: Connectivity problem, Shortest Path problem, etc.

- The class **NP** consists of decision problems to which a proposed solution can be checked quickly.
  - Examples: Hamiltonian Circuit problem, decision version of 0/1 Knapsack problem, Circuit-SAT, etc.

- **NP-Complete** is a subset of NP, the set of problems to which all problems in NP can be reduced in [polynomial time](#big-o-complexity-hierarchy).
  - Examples: decision version of Vertex Cover problem, etc.

- **NP-Hard** is the set of problems to which all problems in NP can be reduced in polynomial time. NP-hard problems need not be in NP; i.e., they need not have solutions verifiable in polynomial time and can be either decision problems or optimization problems.
  - Examples: The optimization version of the TSP, etc.

#### P vs NP Problem

Whether P = NP or not is one of the most important open questions in theoretical computer science.

![Euler diagram for P, NP, NP-complete, and NP-hard set of problems (excluding the empty language and its complement, which belong to P but are not NP-complete)](https://upload.wikimedia.org/wikipedia/commons/thumb/a/a0/P_np_np-complete_np-hard.svg/2880px-P_np_np-complete_np-hard.svg.png)

## Brute Force

### Brute-Force Sorting

#### Selection Sort

- **Input**: an unsorted list of comparable elements.
- **Output**: the same list sorted in the desired order (ascending here).
- **Time complexity**: $O(n^2)$
- **Implementation**: repeatedly selects the smallest element for each index.

[Here](./review/src/brute_force.rs) is the code:

```rust
fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        if let Some((min_i, _)) = arr.iter().enumerate().skip(i).min_by_key(|(_, e)| *e) {
            arr.swap(i, min_i)
        }
    }
}
```

#### Bubble Sort

- **Input**: an unsorted list of comparable elements.
- **Output**: the same list sorted in the desired order (ascending here).
- **Time complexity**: $O(n^2)$
- **Implementation**: repeatedly swaps adjacent elements if they are in the wrong order.

[Here](./review/src/brute_force.rs) is the code:

```rust
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 1..arr.len() - i {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
}
```

#### Insertion Sort

- **Input**: an unsorted list of comparable elements.
- **Output**: the same list sorted in the desired order (ascending here).
- **Time complexity**: $O(n^2)$
- **Implementation**: repeatedly inserts the next element into the sorted part of the list.

[Here](./review/src/brute_force.rs) is the code:

```rust
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
```

### Brute-Force String

#### Linear Search

- **Input**: a list of comparable elements and a target list.
- **Output**: the index of the first occurrence of the target list in the list of elements, or None if not found.
- **Time complexity**: $O(nm)$, where $n$ is the length of the list of elements and $m$ is the length of the target list.
- **Implementation**: repeatedly compares the target list with a sublist of the list of elements.

[Here](./review/src/brute_force.rs) is the code:

```rust
fn linear_search<T: Eq>(haystack: &[T], needle: &[T]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .enumerate()
        .find_map(|(i, current)| if current == needle { Some(i) } else { None })
}
```

### Brute-Force Graph

#### Breadth-First Search

- **Input**: a graph (or tree) and a start node.
- **Output**: a set of all nodes reachable from the start node.
- **Time complexity**: $O(V + E)$, where $V$ is the number of vertices and $E$ is the number of edges.
- **Implementation**: explores the graph level by level, starting from the given node.

[Here](./review/src/brute_force.rs) is the code:

```rust
fn bfs<N, E>(graph: &Graph<N, E>, start: NodeIndex) -> HashSet<NodeIndex> {
    let mut deque = VecDeque::from([start]);
    let mut visited = HashSet::from([start]);

    while let Some(node) = deque.pop_front() {
        for neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                deque.push_back(neighbor);
            }
        }
    }

    visited
}
```

#### Depth-First Search

- **Input**: a graph (or tree) and a start node.
- **Output**: a set of all nodes reachable from the start node.
- **Time complexity**: $O(V + E)$, where $V$ is the number of vertices and $E$ is the number of edges.
- **Implementation**: explores as far as possible along each branch before backtracking.

[Here](./review/src/brute_force.rs) is the code:

```rust
fn dfs<N, E>(graph: &Graph<N, E>, start: NodeIndex, visited: &mut HashSet<NodeIndex>) {
    visited.insert(start);

    for neighbor in graph.neighbors(start) {
        if !visited.contains(&neighbor) {
            dfs(graph, neighbor, visited);
        }
    }
}
```

## Divide and Conquer

### Divide-and-Conquer Sorting

#### Merge Sort

- **Input**: an unsorted list of comparable elements.
- **Output**: the same list sorted in the desired order (ascending here).
- **Time complexity**: $O(n \log n)$
- **Implementation**: repeatedly splits the list into two halves, sorts them, and merges them

[Here](./review/src/divide_and_conquer.rs) is the code:

```rust
fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    match arr.len() {
        0 | 1 => arr.to_vec(),
        _ => {
            let (left, right) = arr.split_at(arr.len() / 2);
            merge(&merge_sort(left), &merge_sort(right))
        }
    }
}

fn merge<T: Ord + Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i].clone());
            i += 1;
        } else {
            merged.push(arr2[j].clone());
            j += 1;
        }
    }

    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);

    merged
}
```

### Divide-and-Conquer Searching

#### Binary Search

- **Input**: a sorted list of comparable elements and a target element.
- **Output**: boolean indicating whether the target element is in the list.
- **Time complexity**: $O(\log n)$, where $n$ is the length of the list.
- **Implementation**: repeatedly divides the list in half until the target is found or the subarray size becomes 0.

[Here](./review/src/divide_and_conquer.rs) is the code:

```rust
fn binary_search<T: Ord>(arr: &[T], target: &T) -> bool {
    if arr.is_empty() {
        return false;
    }
    let mid = arr.len() / 2;
    match arr[mid].cmp(target) {
        Equal => true,
        Less => binary_search(&arr[mid + 1..], target),
        Greater => binary_search(&arr[..mid], target),
    }
}
```

## Dynamic Programming

### Dynamic Programming String

#### Longest Common Subsequence (LCS)

- **Input**: Two strings $X$ and $Y$.

- **Output**: The longest common subsequence of $X$ and $Y$.

- **Time complexity**: $O(mn)$, where $m$ and $n$ are the lengths of $X$ and $Y$ respectively.

- **Implementation**:

  1. Create a 2D DP table of size $(m+1) \times (n+1)$, where $m$ and $n$ are the lengths of $X$ and $Y$ respectively. Let's denote this table as $dp$.

  2. Initialize all cells of the DP table, $dp[i][j]$, to 0.

  3. Fill the DP table using:

  $$
  \begin{align*}
  dp[i][j] =
  \begin{cases}
  0 & \text{if } i = 0 \text{ or } j = 0, \\
  dp[i-1][j-1] + 1 & \text{if } X[i-1] = Y[j-1], \\
  \max(dp[i-1][j], dp[i][j-1]) & \text{otherwise}
  \end{cases}
  \end{align*}
  $$

  4. Backtrack the DP table to find the LCS. Starting from the cell $dp[m, n]$, backtrack until $dp[0, 0]$ is reached:

  $$
  \begin{align*}
  \text{{backtrack}}(dp, X, Y, i, j) =
  \begin{cases}
  \text{{empty string}} & \text{if } i = 0 \text{ or } j = 0,\\
  X[i-1] + \text{{backtrack}}(dp, X, Y, i-1, j-1) & \text{if } X[i-1] = Y[j-1], \\
  \text{{backtrack}}(dp, X, Y, i-1, j) & \text{if } X[i-1] \neq Y[j-1] \text{ and } dp[i-1][j] \geq dp[i][j-1],\\
  \text{{backtrack}}(dp, X, Y, i, j-1) & \text{if } X[i-1] \neq Y[j-1] \text{ and } dp[i][j-1] > dp[i-1][j].
  \end{cases}
  \end{align*}
  $$

The result is the longest common subsequence of $X$ and $Y$. Note that there may be multiple LCSs if there are multiple ways to reach the same length. This algorithm finds one of them.

[Here](./review/src/dynamic_programming.rs) is the code:

```rust
fn lcs(str1: &str, str2: &str) -> String {
    let m = str1.len();
    let n = str2.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    backtrack_lcs(&dp, str1, str2, m, n)
}

fn backtrack_lcs<T: Ord>(dp: &Vec<Vec<T>>, str1: &str, str2: &str, i: usize, j: usize) -> String {
    if i == 0 || j == 0 {
        String::new()
    } else if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
        format!(
            "{}{}",
            backtrack_lcs(dp, str1, str2, i - 1, j - 1),
            str1.chars().nth(i - 1).unwrap()
        )
    } else {
        if dp[i - 1][j] > dp[i][j - 1] {
            backtrack_lcs(dp, str1, str2, i - 1, j)
        } else {
            backtrack_lcs(dp, str1, str2, i, j - 1)
        }
    }
}
```

#### Pairwise Sequence Alignment (Global - Needleman-Wunsch algorithm)

- **Input**: Two sequences $A$ and $B$.

- **Output**: An optimal global alignment of $A$ and $B$.

- **Time complexity**: $O(mn)$, where $m$ and $n$ are the lengths of $A$ and $B$ respectively.

- **Implementation**:

  1. Create a 2D DP table of size $(m+1) \times (n+1)$, where $m$ and $n$ are the lengths of $A$ and $B$ respectively. Let's denote the matrix as $M$.

  2. Initialize the first row and the first column of the DP table as multiples of the gap penalty.

  3. Fill the DP table using:
  
  $$
  \begin{align*}
  M[i, j] = \max \begin{cases}
  M[i-1, j-1] + S(A_i, B_j) \\
  M[i-1, j] + G \\
  M[i, j-1] + G \\
  \end{cases}
  \end{align*}
  $$

  where:
  - $S(A_i, B_j)$ is the score of aligning residue $A_i$ with residue $B_j$. This would usually be a match score if $A_i$ and $B_j$ are the same, and a mismatch score if they are different.
  - $G$ is the gap penalty.

  4. Backtrack the DP table to find the optimal alignment. Starting from the cell $M[m, n]$, backtrack until $M[0, 0]$ is reached:

  $$
  \begin{align*}
  \text{{backtrack}}(M, A, B, i, j) =
  \begin{cases}
  \text{{empty alignment}} & \text{if } i = 0 \text{ and } j = 0,\\
  (A_i, -) \, \text{{concat}} \, \text{{backtrack}}(M, A, B, i-1, j) & \text{if } M[i, j] = M[i-1, j] + G,\\
  (-, B_j) \, \text{{concat}} \, \text{{backtrack}}(M, A, B, i, j-1) & \text{if } M[i, j] = M[i, j-1] + G,\\
  (A_i, B_j) \, \text{{concat}} \, \text{{backtrack}}(M, A, B, i-1, j-1) & \text{if } M[i, j] = M[i-1, j-1] + S(A_i, B_j).
  \end{cases}
  \end{align*}
  $$

  The result is a pairwise alignment of $A$ and $B$ that maximizes the total alignment score.
  
  [Here](./review/src/dynamic_programming.rs) is the code:
  
  ```rust
  fn global_alignment(
      str1: &str,
      str2: &str,
      gap_penalty: i64,
      score: fn(char, char) -> i64,
  ) -> (i64, (String, String)) {
      let m = str1.len();
      let n = str2.len();
  
      let mut dp = vec![vec![0; n + 1]; m + 1];
  
      // Initialize the DP table
      for i in 0..=m {
          dp[i][0] = i as i64 * gap_penalty;
      }
      for j in 0..=n {
          dp[0][j] = j as i64 * gap_penalty;
      }
  
      // Fill the DP table
      for i in 1..=m {
          for j in 1..=n {
              let match_score = dp[i - 1][j - 1]
                  + score(
                      str1.chars().nth(i - 1).unwrap(),
                      str2.chars().nth(j - 1).unwrap(),
                  );
              let delete = dp[i - 1][j] + gap_penalty;
              let insert = dp[i][j - 1] + gap_penalty;
              dp[i][j] = match_score.max(delete).max(insert);
          }
      }
  
      // Backtrack to find the optimal alignment
      let alignment = backtrack_global_alignment(&dp, str1, str2, m, n, gap_penalty, score);
  
      (dp[m][n], alignment)
  }
  
  fn backtrack_global_alignment(
      dp: &Vec<Vec<i64>>,
      str1: &str,
      str2: &str,
      i: usize,
      j: usize,
      gap_penalty: i64,
      score: fn(char, char) -> i64,
  ) -> (String, String) {
      if i == 0 && j == 0 {
          (String::new(), String::new())
      } else if i > 0 && dp[i][j] == dp[i - 1][j] + gap_penalty {
          let (x, y) = backtrack_global_alignment(dp, str1, str2, i - 1, j, gap_penalty, score);
          (
              format!("{}{}", x, str1.chars().nth(i - 1).unwrap()),
              format!("{}-", y),
          )
      } else if j > 0 && dp[i][j] == dp[i][j - 1] + gap_penalty {
          let (x, y) = backtrack_global_alignment(dp, str1, str2, i, j - 1, gap_penalty, score);
          (
              format!("{}-", x),
              format!("{}{}", y, str2.chars().nth(j - 1).unwrap()),
          )
      } else {
          let (x, y) = backtrack_global_alignment(dp, str1, str2, i - 1, j - 1, gap_penalty, score);
          (
              format!("{}{}", x, str1.chars().nth(i - 1).unwrap()),
              format!("{}{}", y, str2.chars().nth(j - 1).unwrap()),
          )
      }
  }
  ```
  
#### Pairwise Sequence Alignment (Local - Smith-Waterman algorithm)

- **Input**: Two sequences $A$ and $B$.

- **Output**: An optimal local alignment of $A$ and $B$.

- **Time complexity**: $O(mn)$, where $m$ and $n$ are the lengths of $A$ and $B$ respectively.

- **Implementation**:

  1. Create a 2D DP table of size $(m+1) \times (n+1)$, where $m$ and $n$ are the lengths of $A$ and $B$ respectively. Let's denote the matrix as $M$.

  2. Initialize the first row and the first column of the DP table to 0.

  3. Fill the DP table using:
  
  $$
  \begin{align*}
  M[i, j] = \max \begin{cases}
  0 \\
  M[i-1, j-1] + S(A_i, B_j) \\
  M[i-1, j] + G \\
  M[i, j-1] + G \\
  \end{cases}
  \end{align*}
  $$

  where:
  - $S(A_i, B_j)$ is the score of aligning residue $A_i$ with residue $B_j$. This would usually be a match score if $A_i$ and $B_j$ are the same, and a mismatch score if they are different.
  - $G$ is the gap penalty.

  4. Backtrack the DP table to find the optimal alignment. Starting from the cell with the maximum value, backtrack until a cell with a value of 0 is reached:

  $$
  \begin{align*}
  \text{{backtrack}}(M, A, B, i, j) =
  \begin{cases}
  \text{{empty alignment}} & \text{if } M[i, j] = 0,\\
  (A_i, -) \, \text{{concat}} \, \text{{backtrack}}(M, A, B, i-1, j) & \text{if } M[i, j] = M[i-1, j] + G,\\
  (-, B_j) \, \text{{concat}} \, \text{{backtrack}}(M, A, B, i, j-1) & \text{if } M[i, j] = M[i, j-1] + G,\\
  (A_i, B_j) \, \text{{concat}} \, \text{{backtrack}}(M, A, B, i-1, j-1) & \text{if } M[i, j] = M[i-1, j-1] + S(A_i, B_j).
  \end{cases}
  \end{align*}
  $$

  The result is a pairwise alignment of $A$ and $B$ that maximizes the total alignment score over all possible local alignments.
  
  [Here](./review/src/dynamic_programming.rs) is the code:
  
  ```rust
  fn local_alignment(
      str1: &str,
      str2: &str,
      gap_penalty: i64,
      score: fn(char, char) -> i64,
  ) -> (i64, (String, String)) {
      let m = str1.len();
      let n = str2.len();
  
      let mut dp = vec![vec![0; n + 1]; m + 1];
  
      let mut max_i = 0;
      let mut max_j = 0;
      let mut max_score = 0;
  
      // Fill the DP table
      for i in 1..=m {
          for j in 1..=n {
              let match_score = dp[i - 1][j - 1]
                  + score(
                      str1.chars().nth(i - 1).unwrap(),
                      str2.chars().nth(j - 1).unwrap(),
                  );
              let delete = dp[i - 1][j] + gap_penalty;
              let insert = dp[i][j - 1] + gap_penalty;
              dp[i][j] = 0.max(match_score.max(delete).max(insert));
  
              if dp[i][j] > max_score {
                  max_score = dp[i][j];
                  max_i = i;
                  max_j = j;
              }
          }
      }
  
      // Backtrack to find the optimal alignment
      let alignment = backtrack_local_alignment(&dp, str1, str2, max_i, max_j, gap_penalty, score);
  
      (max_score, alignment)
  }
  
  fn backtrack_local_alignment(
      dp: &Vec<Vec<i64>>,
      str1: &str,
      str2: &str,
      i: usize,
      j: usize,
      gap_penalty: i64,
      score: fn(char, char) -> i64,
  ) -> (String, String) {
      if dp[i][j] == 0 {
          (String::new(), String::new())
      } else if i > 0 && dp[i][j] == dp[i - 1][j] + gap_penalty {
          let (x, y) = backtrack_local_alignment(dp, str1, str2, i - 1, j, gap_penalty, score);
          (
              format!("{}{}", x, str1.chars().nth(i - 1).unwrap()),
              format!("{}-", y),
          )
      } else if j > 0 && dp[i][j] == dp[i][j - 1] + gap_penalty {
          let (x, y) = backtrack_local_alignment(dp, str1, str2, i, j - 1, gap_penalty, score);
          (
              format!("{}-", x),
              format!("{}{}", y, str2.chars().nth(j - 1).unwrap()),
          )
      } else {
          let (x, y) = backtrack_local_alignment(dp, str1, str2, i - 1, j - 1, gap_penalty, score);
          (
              format!("{}{}", x, str1.chars().nth(i - 1).unwrap()),
              format!("{}{}", y, str2.chars().nth(j - 1).unwrap()),
          )
      }
  }
  ```

### Dynamic Programming Graph

#### Floyd's Algorithm

- **Input**: A weighted graph $G$ with $n$ vertices, represented by an adjacency matrix.

- **Output**: A matrix $D$ where each cell $D[i][j]$ represents the shortest distance from vertex $i$ to vertex $j$ in graph $G$.

- **Time complexity**: $O(n^3)$, where $n$ is the number of vertices in the graph.

- **Implementation**:

  1. Create a 2D matrix $D$ such that $D[i][j]$ contains the direct distance from node $i$ to node $j$ (or infinity if there is no direct link).

  2. For each node $k$, consider each pair of nodes $i$ and $j$ and check if going from $i$ to $j$ through the node $k$ improves the current shortest path from $i$ to $j$. If it does, update the cell $D[i][j]$ with the new shorter distance. This can be formalized as:

  $$
  D[i][j] = \min 
  \begin{cases}
  D[i][j], \\
  D[i][k] + D[k][j]
  \end{cases}
  $$
  
  3. Repeat the above step until all nodes have been considered as intermediate nodes.

At the end, $D[i][j]$ will contain the shortest distance from node $i$ to node $j$ in graph $G$. Note that Floyd's algorithm works for both positive and negative edge weights, but cannot handle negative cycles.

[Here](./review/src/dynamic_programming.rs) is the code:

```rust
fn floyd_warshall(adj_matrix: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = adj_matrix.len();
    let mut dist = adj_matrix.clone();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != i64::MAX && dist[k][j] != i64::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    dist
}
```

#### Warshall's Algorithm

- **Input**: A directed graph $G$ with $n$ vertices, represented by an adjacency matrix.

- **Output**: A matrix $R$ where each cell $R[i][j]$ is true if there is a path from vertex $i$ to vertex $j$ in graph $G$, and false otherwise.

- **Time complexity**: $O(n^3)$, where $n$ is the number of vertices in the graph.

- **Implementation**:

  1. Create a 2D matrix $R$ such that $R[i][j]$ is true if there is a direct edge from node $i$ to node $j$ in the graph (or false if there is no direct edge).

  2. For each node $k$, consider each pair of nodes $i$ and $j$. If there is a path from $i$ to $k$ and a path from $k$ to $j$ (i.e., if $R[i][k]$ and $R[k][j]$ are both true), then there is a path from $i$ to $j$. So, set $R[i][j]$ to true. This can be formalized as:

  $$
  \begin{align*}
  R[i][j] = R[i][j] \lor (R[i][k] \land R[k][j])
  \end{align*}
  $$
  
  3. Repeat the above step until all nodes have been considered as intermediate nodes.

At the end, $R[i][j]$ will be true if there is a path from node $i$ to node $j$ in graph $G$, and false otherwise. Note that Warshall's Algorithm is a variant of Floyd's Algorithm and is used to find the transitive closure of a directed graph.

#### Assembly Line Scheduling Algorithm

- **Input**: Two tuples of sequences representing the processing times at each station `a = (a1, a2)` and the transfer times `t = (t1, t2)`. Two tuples of values `e = (e1, e2)` and `x = (x1, x2)` represent entry and exit times for each line, respectively.

- **Output**: The minimum time required to go through the assembly process.

- **Time complexity**: $O(mn)$ if there are "m" assembly lines with "n" stations each.

- **Implementation**:
1. Initialize two sequences `f = (f1, f2)` to store the fastest times to reach each station in the respective lines.
  
2. Set the initial values of `f1` and `f2` as `e1 + a1[0]` and `e2 + a2[0]`, respectively.
  
3. For each subsequent station `i` from `1` to `n-1`:
     - Compute `f1[i]` as the minimum between `f1[i-1] + a1[i]` and `f2[i-1] + t2[i-1] + a1[i]`.
     - Compute `f2[i]` as the minimum between `f2[i-1] + a2[i]` and `f1[i-1] + t1[i-1] + a2[i]`.
  
4. The minimum total time through the assembly line is the minimum between `f1[n-1] + x1` and `f2[n-1] + x2`.

[Here](./review/src/dynamic_programming.rs) is the code:

```rust
pub fn assembly_line_scheduling(
    a: (&[u64], &[u64]),
    t: (&[u64], &[u64]),
    e: (u64, u64),
    x: (u64, u64),
) -> u64 {
    let n = a.0.len();
    let mut f = (vec![0; n], vec![0; n]);

    f.0[0] = e.0 + a.0[0];
    f.1[0] = e.1 + a.1[0];

    for i in 1..n {
        f.0[i] = (f.0[i - 1] + a.0[i]).min(f.1[i - 1] + t.1[i - 1] + a.0[i]);
        f.1[i] = (f.1[i - 1] + a.1[i]).min(f.0[i - 1] + t.0[i - 1] + a.1[i]);
    }

    (f.0[n - 1] + x.0).min(f.1[n - 1] + x.1)
}
```

