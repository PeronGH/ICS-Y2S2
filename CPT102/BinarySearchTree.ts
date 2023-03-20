export class BinarySearchTreeNode<T> {
  constructor(
    public value: T,
    public left: BinarySearchTreeNode<T> | null = null,
    public right: BinarySearchTreeNode<T> | null = null,
  ) {
    // validate that left is less than value
    // and right is greater than value

    if (left && left.value >= value) {
      throw new Error("Left node is greater than or equal to parent");
    }

    if (right && right.value <= value) {
      throw new Error("Right node is less than or equal to parent");
    }
  }

  get leftmostNode(): BinarySearchTreeNode<T> {
    return this.left ? this.left.leftmostNode : this;
  }

  get rightmostNode(): BinarySearchTreeNode<T> {
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

  find(value: T): BinarySearchTreeNode<T> | null {
    if (value === this.value) {
      return this;
    }

    if (value < this.value && this.left) {
      return this.left.find(value);
    }

    if (value > this.value && this.right) {
      return this.right.find(value);
    }

    return null;
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

  remove(value: T): BinarySearchTreeNode<T> | null {
    if (value < this.value) {
      if (this.left) {
        this.left = this.left.remove(value);
      }
    } else if (value > this.value) {
      if (this.right) {
        this.right = this.right.remove(value);
      }
    } else {
      // Case 1: Node with no children (leaf node)
      if (this.isLeaf) {
        return null;
      }

      // Case 2: Node with only one child
      if (!this.left) {
        return this.right;
      }
      if (!this.right) {
        return this.left;
      }

      // Case 3: Node with two children
      // Replace the current node's value with the value of its in-order predecessor
      // (the rightmost node in its left subtree) and remove the in-order predecessor
      // Note `rightmostInLeftSubtree.value` is the largest value that is smaller than `this.value`
      const rightmostInLeftSubtree = this.left.rightmostNode;
      this.value = rightmostInLeftSubtree.value;
      this.left = this.left.remove(this.value);
    }

    return this;
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

export class BinarySearchTree<T> extends BinarySearchTreeNode<T> {
  private static insertToNode<T>(
    node: BinarySearchTreeNode<T> | null,
    value: T,
  ) {
    if (!node) {
      return new BinarySearchTreeNode(value);
    }

    if (value < node.value) {
      node.left = BinarySearchTree.insertToNode(node.left, value);
    } else if (value > node.value) {
      node.right = BinarySearchTree.insertToNode(node.right, value);
    }

    return node;
  }

  insert(value: T) {
    return BinarySearchTree.insertToNode(this, value);
  }
}

Deno.test("binary search tree", () => {
  const tree = new BinarySearchTree(5);
  tree.insert(3);
  tree.insert(7);
  tree.insert(2);
  tree.insert(4);
  tree.insert(6);
  tree.insert(8);
  tree.insert(1);
  tree.insert(9);
  tree.insert(0);

  console.log(tree.size);
  console.log(tree.collect("in"));
  console.log(tree.collect("pre"));
  console.log(tree.collect("post"));
  console.log(tree.find(7)?.toString());
  console.log(tree.toString());
  tree.remove(5);
  console.log(tree.toString());
  console.log(tree.collect());
});

Deno.test("remove", () => {
  const tree = new BinarySearchTree(5);
  tree.insert(4);
  tree.insert(3);
  tree.insert(2);
  tree.insert(1);

  console.log(tree.toString());
  tree.remove(4);
  console.log(tree.toString());
});