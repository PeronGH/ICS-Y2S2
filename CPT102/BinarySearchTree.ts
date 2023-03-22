import { BinaryTreeNode } from "./BinaryTreeNode.ts";

export class BinarySearchTreeNode<T> extends BinaryTreeNode<T> {
  constructor(
    public value: T,
    public left: BinarySearchTreeNode<T> | null = null,
    public right: BinarySearchTreeNode<T> | null = null,
  ) {
    super(value, left, right);

    // validate that left is less than value
    // and right is greater than value

    if (left && left.value >= value) {
      throw new Error("Left node is greater than or equal to parent");
    }

    if (right && right.value <= value) {
      throw new Error("Right node is less than or equal to parent");
    }
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
