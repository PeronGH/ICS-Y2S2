import { Edge, Graph, Vertex } from "./Graph.ts";
import { UnionFind } from "./UnionFind.ts";

class MST<T> extends Graph<T> {
  primMST(start: Vertex<T>): Edge<T>[] {
    const mst: Edge<T>[] = [];
    const visited: Set<Vertex<T>> = new Set();

    if (this.vertices.size === 0) {
      return mst;
    }

    visited.add(start);

    while (visited.size < this.vertices.size) {
      let minEdge: Edge<T> | null = null;

      for (const vertex of visited) {
        const neighbors = this.getNeighbors(vertex);

        for (const edge of neighbors) {
          if (
            !visited.has(edge.end) &&
            (minEdge === null || edge.weight < minEdge.weight)
          ) {
            minEdge = edge;
          }
        }
      }

      if (minEdge !== null) {
        visited.add(minEdge.end);
        mst.push(minEdge);
      } else {
        break;
      }
    }

    return mst;
  }

  kruskalMST(): Edge<T>[] {
    const mst: Edge<T>[] = [];

    const edges = Array.from(this.edges).sort((a, b) => a.weight - b.weight);
    const vertexMap: Map<Vertex<T>, number> = new Map();
    let vertexIndex = 0;

    this.vertices.forEach((vertex) => {
      vertexMap.set(vertex, vertexIndex++);
    });

    const uf = new UnionFind(this.vertices.size);

    for (const edge of edges) {
      const startRoot = uf.find(vertexMap.get(edge.start)!);
      const endRoot = uf.find(vertexMap.get(edge.end)!);

      if (startRoot !== endRoot) {
        mst.push(edge);
        uf.union(startRoot, endRoot);
      }
    }

    return mst;
  }
}

const vertices = [0, 1, 2, 3, 4, 5].map((value) => new Vertex(value));

const graph = new MST(...vertices);

graph.addEdge(0, 1, 2);
graph.addEdge(0, 2, 4);
graph.addEdge(1, 2, 1);
graph.addEdge(1, 3, 7);
graph.addEdge(2, 4, 3);
graph.addEdge(2, 3, 3);
graph.addEdge(3, 4, 5);
graph.addEdge(3, 5, 6);
graph.addEdge(4, 5, 1);

console.log(graph.primMST(vertices[0]));
console.log(graph.kruskalMST());
