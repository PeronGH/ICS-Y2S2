1. **Tree**: A tree is a special kind of DAG (directed graph with no directed cycles) where there is exactly one path between any two nodes. This means that there's a single root node and each node (except the root) has exactly one parent.

   - **Root**: One node is distinguished as the root; it's the starting node of the tree.

   - **Parent and Child**: Every node other than the root is connected by an edge from exactly one other node, its parent. A node can have multiple children.
   - **Degree of a Node**: The degree of a node is the number of its children.
   - **Degree of a Tree**: The degree of a tree is the maximum degree of any node in the tree.
   - **Leaf Node**: Any node that has no children is called a leaf node.
   - **Height of a Node**: The height of a node is the length of the longest path from it to a leaf.
   - **Height of a Tree**: The height of a tree is the height of its root node, or equivalently, the depth of its deepest node.
   - **Depth of a Node**: The depth of a node is the length of the path from it to the root.

2. **Binary Tree**: A binary tree is a tree where each node has at most two children, conventionally referred to as the left child and the right child.

3. **Heap**: A heap is a binary tree with two additional properties:

   - **Shape property**: It's a complete binary tree, meaning all levels are fully filled except possibly for the last level, which is filled from left to right.
   - **Heap property**: Each node follows the min-heap or max-heap property with respect to its parent (in a min-heap, parent nodes are less than or equal to their children; in a max-heap, parent nodes are greater than or equal to their children).

   **Deletion**:

   1. **Remove the Root**: Remove the root node. This creates a "hole" at the top of the heap.
   2. **Move Last Element to Root**: Move the last element in the heap (the rightmost node on the bottom level) to the root position. This ensures the binary heap remains complete, but it may violate the heap property.
   3. **Heapify Down**: Restore the heap property by percolating the new root down to its appropriate position. Compare it with its children:
      - If it maintains the heap property with both children, you're done.
      - If not, swap it with its smaller child in a min-heap (or larger child in a max-heap), and repeat the process with the swapped child.

4. **Binary Search Tree (BST)**: A BST is a binary tree with an ordering property: for any node, all nodes in the left subtree are less than the node, and all nodes in the right subtree are greater.

   **Deletion**:

   1. **Locate the Node**: Start from the root and navigate through the tree to locate the node you wish to remove. If the node is not found, the removal is impossible and the structure remains unchanged.

   2. **Node Removal**: Once you've found the node, there are three scenarios to consider:

      a. **Leaf Node (no children)**: If the node is a leaf node (i.e., it doesn't have any children), you can remove it directly without affecting the rest of the tree.

      b. **One Child**: If the node has a single child, you can "bypass" the node to be deleted. Link its parent to its child, and then remove the node.

      c. **Two Children**: If the node has two children, you have to maintain the binary search tree property after the removal. The common practice is to find the node's in-order predecessor (the maximum value in its left subtree) or in-order successor (the minimum value in its right subtree), replace the node's value with the predecessor/successor's value, and then delete the predecessor/successor, which will fall into one of the first two cases.

   In a Binary Search Tree (BST), the time complexity for search, insert, and delete operations can degrade to O(n) in the worst case scenario when the tree becomes a linked list (skewed tree).

5. **AVL Tree**: An AVL Tree is a Binary Search Tree (BST) that maintains its balance through a property called the "Balance Factor." The balance factor of a node in an AVL tree is the height of its left subtree minus the height of its right subtree. The tree is considered balanced if every node has a balance factor of -1, 0, or 1. If the balance factor of any node in the tree is not in the range -1 to 1 inclusive, the tree is unbalanced, and it needs to be balanced again.

   **Rotation**:

   Rotation is a key operation in AVL trees that helps maintain their balance. There are four types of rotations: left, right, left-right (LR), and right-left (RL).

   - **Left Rotation**: Used when a right-heavy imbalance is detected. This rotates the subtree to the left, effectively moving the current root of the subtree down to the left, and its right child up to the root.
   - **Right Rotation**: Used when a left-heavy imbalance is detected. This rotates the subtree to the right, effectively moving the current root of the subtree down to the right, and its left child up to the root.
   - **Left-Right Rotation (LR)**: A combination of left rotation followed by right rotation. This is used when the left subtree is right-heavy.
   - **Right-Left Rotation (RL)**: A combination of right rotation followed by left rotation. This is used when the right subtree is left-heavy.

   **Insertion** and **Deletion**:

   1. **Perform Basic BST Operation**: For insertion, this means placing the new node at the correct location. For deletion, it involves removing the node and possibly replacing it with a successor or predecessor.
   2. **Update Heights**: After insertion or deletion, traverse back up the tree and update the height of each node. The height of a node is typically defined as 1 plus the maximum height of its children.
   3. **Check for Imbalances**: For each node visited, compute the balance factor (the difference in height between the left child and the right child). If at any node the balance factor is not -1, 0, or 1, then an imbalance has been detected at that node.
   4. **Rebalance the Tree**: If an imbalance is detected, perform rotations to restore balance. The type of rotation performed depends on whether the imbalance is left-heavy or right-heavy and also on the balance factor of the child causing the imbalance.

   In an AVL Tree, which is a self-balancing BST, the height of the tree is always logarithmic in the number of nodes, hence the time complexity for search, insert, and delete operations is guaranteed to be O(log n), even in the worst case.