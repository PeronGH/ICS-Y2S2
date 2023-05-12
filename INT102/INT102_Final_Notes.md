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
fn selection_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    for i in 0..arr.len() {
        if let Some((min_i, ..)) = arr.iter().enumerate().skip(i).min_by_key(|(.., v)| *v) {
            arr.swap(i, min_i);
        }
    }
    arr
}
```

#### Bubble Sort

- **Input**: an unsorted list of comparable elements.
- **Output**: the same list sorted in the desired order (ascending here).
- **Time complexity**: $O(n^2)$
- **Implementation**: repeatedly swaps adjacent elements if they are in the wrong order.

[Here](./review/src/brute_force.rs) is the code:

```rust
fn bubble_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    for _ in 0..arr.len() {
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
            }
        }
    }
    arr
}
```
