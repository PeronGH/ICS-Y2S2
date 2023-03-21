import { Edge, Graph, Vertex } from "./Graph.ts";
import { UnionFind } from "./UnionFind.ts";

class MST<T> extends Graph<T> {
  primMST(start: Vertex<T>): Edge<T>[] {
    // Create an empty array to store edges of the minimum spanning tree
    const mst: Edge<T>[] = [];
    // Create a set to keep track of visited vertices
    const visited: Set<Vertex<T>> = new Set();

    // If the graph has no vertices, return the empty MST
    if (this.vertices.size === 0) {
      return mst;
    }

    // Add the starting vertex to the visited set
    visited.add(start);

    // Repeat until all vertices have been visited
    while (visited.size < this.vertices.size) {
      // Find the minimum weight edge that connects a vertex in the MST to a vertex not in the MST
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

      // If an edge was found, add the new vertex to the visited set and add the edge to the MST
      if (minEdge !== null) {
        visited.add(minEdge.end);
        mst.push(minEdge);
      } else {
        // If no edge was found, we have multiple disconnected components in the graph and we are done
        break;
      }
    }

    // Return the completed MST
    return mst;
  }

  /**
   * Finds the minimum spanning tree of the graph using Kruskal's algorithm.
   * @returns An array of edges that form the minimum spanning tree.
   */
  kruskalMST(): Edge<T>[] {
    const mst: Edge<T>[] = [];

    // Step 1: Sort the edges in increasing order of weight.
    const edges = [...this.edges].sort((a, b) => a.weight - b.weight);

    // Step 2: Create a mapping from each vertex to its index (0-based) in the UnionFind data structure.
    const vertexMap: Map<Vertex<T>, number> = new Map();
    let vertexIndex = 0;

    this.vertices.forEach((vertex) => {
      vertexMap.set(vertex, vertexIndex++);
    });

    // Step 3: Create a UnionFind data structure with one element for each vertex.
    const uf = new UnionFind(this.vertices.size);

    // Step 4: Iterate over the sorted edges and add them to the MST if they do not form a cycle.
    for (const edge of edges) {
      const startRoot = uf.find(vertexMap.get(edge.start)!);
      const endRoot = uf.find(vertexMap.get(edge.end)!);

      // If the start and end vertices of the edge belong to different sets, add the edge to the MST and merge the sets.
      if (startRoot !== endRoot) {
        mst.push(edge);
        uf.union(startRoot, endRoot);
      }
    }

    // Step 5: Return the MST.
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
