export class UnionFind {
  private parent: number[] = []; // an array to store the parent of each element
  private rank: number[] = []; // an array to store the rank (depth) of each tree

  constructor(size: number) { // constructor initializes the parent array and the rank array
    for (let i = 0; i < size; i++) {
      this.parent[i] = i; // initially, each element is its own parent
      this.rank[i] = 0; // initially, each tree has depth 0
    }
  }

  /**
   * Find the root of the tree that x belongs to
   * @param x The element whose root we want to find
   * @returns the root of the tree that x belongs to
   */
  find(x: number): number {
    if (this.parent[x] !== x) { // if x is not the parent of itself
      this.parent[x] = this.find(this.parent[x]); // recursively find the parent of x and update the parent of x to the root
    }
    return this.parent[x]; // return the parent of x (which is now the root of the tree)
  }

  /**
   * Merge the sets containing x and y
   * @param x An element in the first set
   * @param y An element in the second set
   */
  union(x: number, y: number): void {
    const rootX = this.find(x); // find the root of the tree containing x
    const rootY = this.find(y); // find the root of the tree containing y

    if (rootX === rootY) { // if x and y are already in the same set, do nothing
      return;
    }

    if (this.rank[rootX] > this.rank[rootY]) { // if the tree containing x is deeper than the tree containing y
      this.parent[rootY] = rootX; // make the root of the tree containing y a child of the root of the tree containing x
    } else if (this.rank[rootX] < this.rank[rootY]) { // if the tree containing y is deeper than the tree containing x
      this.parent[rootX] = rootY; // make the root of the tree containing x a child of the root of the tree containing y
    } else { // if the trees containing x and y have the same depth
      this.parent[rootY] = rootX; // make the root of the tree containing y a child of the root of the tree containing x
      this.rank[rootX]++; // increment the depth of the tree containing x
    }
  }
}
