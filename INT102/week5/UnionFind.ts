export class UnionFind {
  private parent: number[] = []; // an array to store the parent of each element
  private rank: number[] = []; // an array to store the rank (depth) of each tree

  // The constructor initializes both arrays, with each element initially being its own parent and having rank 0.
  constructor(size: number) {
    for (let i = 0; i < size; i++) {
      this.parent[i] = i;
      this.rank[i] = 0;
    }
  }

  /**
   * The `find` method takes an element and returns the root of the tree it belongs to.
   * It uses path compression to optimize future calls to `find`.
   * @param x The element whose root we want to find
   * @returns the root of the tree that x belongs to
   */
  find(x: number): number {
    if (this.parent[x] !== x) { // path compression
      this.parent[x] = this.find(this.parent[x]);
    }
    return this.parent[x];
  }

  /**
   * The `union` method takes two elements and merges the trees they belong to.
   * It uses union by rank to keep the trees balanced.
   * @param x An element in the first set
   * @param y An element in the second set
   */
  union(x: number, y: number): void {
    const rootX = this.find(x);
    const rootY = this.find(y);

    if (rootX === rootY) { // if two elements already belong to the same set, `union` does nothing.
      return;
    }

    if (this.rank[rootX] > this.rank[rootY]) { // union by rank
      this.parent[rootY] = rootX;
    } else if (this.rank[rootX] < this.rank[rootY]) {
      this.parent[rootX] = rootY;
    } else { // if two trees have the same rank, `union` chooses one to be the new root and increments its rank.
      this.parent[rootY] = rootX;
      this.rank[rootX]++;
    }
  }
}
