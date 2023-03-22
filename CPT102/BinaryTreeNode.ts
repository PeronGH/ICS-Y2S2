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
