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
    if (!root) {
      return true;
    }

    const queue: Array<BinaryTreeNode<T> | null> = [root];
    let flag = false;

    while (queue.length > 0) {
      const current = queue.shift()!;

      if (current === null) {
        flag = true;
      } else {
        if (flag) {
          return false;
        }

        queue.push(current.left);
        queue.push(current.right);
      }
    }

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
