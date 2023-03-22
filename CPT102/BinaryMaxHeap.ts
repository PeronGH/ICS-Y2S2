import { BinaryTreeNode } from "./BinaryTreeNode.ts";

export class BinaryMaxHeap {
  private root: BinaryTreeNode<number> | null;

  constructor() {
    this.root = null;
  }

  insert(value: number): void {
    if (this.root === null) {
      this.root = new BinaryTreeNode(value);
    } else {
      this.insertNode(this.root, value);
    }
  }

  private insertNode(node: BinaryTreeNode<number>, value: number): void {
    if (node.left === null) {
      node.left = new BinaryTreeNode(value);
      this.heapifyUp(node.left);
    } else if (node.right === null) {
      node.right = new BinaryTreeNode(value);
      this.heapifyUp(node.right);
    } else {
      const leftSize = node.left.size;
      const rightSize = node.right.size;

      if (leftSize <= rightSize) {
        this.insertNode(node.left, value);
      } else {
        this.insertNode(node.right, value);
      }
    }
  }

  private heapifyUp(node: BinaryTreeNode<number>): void {
    if (node === this.root) return;

    const parent = this.findParent(this.root, node);

    if (parent && parent.value < node.value) {
      this.swapValues(parent, node);
      this.heapifyUp(parent);
    }
  }

  private findParent(
    node: BinaryTreeNode<number> | null,
    target: BinaryTreeNode<number>,
  ): BinaryTreeNode<number> | null {
    if (node === null || node.left === target || node.right === target) {
      return node;
    }

    const leftParent = this.findParent(node.left, target);
    return leftParent !== null
      ? leftParent
      : this.findParent(node.right, target);
  }

  private swapValues(
    node1: BinaryTreeNode<number>,
    node2: BinaryTreeNode<number>,
  ): void {
    [node1.value, node2.value] = [node2.value, node1.value];
  }

  toString(): string {
    return this.root?.toString() || "";
  }
}

Deno.test("BinaryMaxHeap", () => {
  const heap = new BinaryMaxHeap();

  heap.insert(5);
  heap.insert(3);
  heap.insert(17);
  heap.insert(10);
  heap.insert(84);
  heap.insert(19);
  heap.insert(6);
  heap.insert(22);
  heap.insert(9);

  console.log(heap.toString());
});
