export class BinaryTreeNode<T> {
  constructor(
    public value: T,
    public left: BinaryTreeNode<T> | null = null,
    public right: BinaryTreeNode<T> | null = null,
  ) {}

  get leftmostNode(): BinaryTreeNode<T> {
    return this.left ? this.left.leftmostNode : this;
  }

  get rightmostNode(): BinaryTreeNode<T> {
    return this.right ? this.right.rightmostNode : this;
  }

  get isLeaf(): boolean {
    return !this.left && !this.right;
  }

  get size(): number {
    return 1 + (this.left?.size ?? 0) + (this.right?.size ?? 0);
  }

  get isEmpty(): boolean {
    return this.size === 0;
  }

  static isComplete<T>(root: BinaryTreeNode<T> | null): boolean {
    // If the root is null, the tree is considered complete
    if (!root) {
      return true;
    }

    // Initialize a queue to traverse the tree using level order traversal
    const queue: Array<BinaryTreeNode<T> | null> = [root];
    // A flag to indicate whether a null node has been encountered or not
    let flag = false;

    // Loop until the queue is empty
    while (queue.length > 0) {
      // Dequeue the first element from the queue
      const current = queue.shift()!;

      // If the current node is null, set the flag to true
      if (current === null) {
        flag = true;
      } else {
        // If a non-null node is found after a null node, the tree is not complete
        if (flag) {
          return false;
        }

        // Enqueue the left and right children of the current node
        queue.push(current.left);
        queue.push(current.right);
      }
    }

    // If the loop completes without returning false, the tree is complete
    return true;
  }

  get isComplete(): boolean {
    return BinaryTreeNode.isComplete(this);
  }

  collect(order: "in" | "pre" | "post" = "in", result: T[] = []): T[] {
    switch (order) {
      case "in":
        this.left?.collect(order, result);
        result.push(this.value);
        this.right?.collect(order, result);
        break;
      case "pre":
        result.push(this.value);
        this.left?.collect(order, result);
        this.right?.collect(order, result);
        break;
      case "post":
        this.left?.collect(order, result);
        this.right?.collect(order, result);
        result.push(this.value);
        break;
    }

    return result;
  }

  clone(): this {
    return this.constructor(
      this.value,
      this.left?.clone(),
      this.right?.clone(),
    );
  }

  toString(level = 0, prefix = "M:"): string {
    const valueString = "" + this.value;
    const indent = " ".repeat(level * 2);

    // Create the string representation of the node
    let nodeString = `${indent}${prefix}${valueString}\n`;

    // Add the left subtree, if it exists
    if (this.left !== null) {
      nodeString += this.left.toString(level + 1, "L:");
    }

    // Add the right subtree, if it exists
    if (this.right !== null) {
      nodeString += this.right.toString(level + 1, "R:");
    }

    return nodeString;
  }
}
