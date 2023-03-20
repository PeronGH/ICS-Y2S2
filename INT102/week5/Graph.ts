export class Vertex<T> {
  constructor(public value: T) {}

  toJSON() {
    return this.value;
  }

  toString() {
    return `${this.value}`;
  }
}

export class Edge<T> {
  constructor(
    public start: Vertex<T>,
    public end: Vertex<T>,
    public weight: number,
  ) {}

  toString() {
    return `${this.start} -- ${this.end} (${this.weight})`;
  }
}

export class Graph<T> {
  readonly vertices: Map<T, Vertex<T>> = new Map();
  readonly edges: Set<Edge<T>> = new Set();

  constructor(...initialVertices: Vertex<T>[]) {
    for (const vertex of initialVertices) {
      this.vertices.set(vertex.value, vertex);
    }
  }

  addVertex(vertex: Vertex<T> | T): Vertex<T> {
    if (!(vertex instanceof Vertex)) vertex = new Vertex(vertex);
    if (!this.vertices.has(vertex.value)) {
      this.vertices.set(vertex.value, vertex);
    }
    return this.vertices.get(vertex.value)!;
  }

  removeVertex(vertex: Vertex<T>): void {
    this.vertices.delete(vertex.value);

    for (const edge of this.edges) {
      if (edge.start === vertex || edge.end === vertex) {
        this.edges.delete(edge);
      }
    }
  }

  addEdge(start: Vertex<T> | T, end: Vertex<T> | T, weight: number): void {
    if (!(start instanceof Vertex)) start = this.addVertex(start);
    if (!(end instanceof Vertex)) end = this.addVertex(end);

    if (!this.edgeExists(start, end)) {
      const edge = new Edge(start, end, weight);
      this.edges.add(edge);
    }
  }

  removeEdge(start: Vertex<T>, end: Vertex<T>): void {
    for (const edge of this.edges) {
      if (
        (edge.start === start && edge.end === end) ||
        (edge.start === end && edge.end === start)
      ) {
        this.edges.delete(edge);
        break;
      }
    }
  }

  getNeighbors(vertex: Vertex<T>): Edge<T>[] {
    const neighbors: Edge<T>[] = [];
    for (const edge of this.edges) {
      if (edge.start === vertex) {
        neighbors.push(edge);
      } else if (edge.end === vertex) {
        neighbors.push(new Edge(vertex, edge.start, edge.weight));
      }
    }
    return neighbors;
  }

  toJSON() {
    return {
      vertices: [...this.vertices.values()],
      edges: [...this.edges],
    };
  }

  private edgeExists(start: Vertex<T>, end: Vertex<T>): boolean {
    for (const edge of this.edges) {
      if (
        (edge.start === start && edge.end === end) ||
        (edge.start === end && edge.end === start)
      ) {
        return true;
      }
    }
    return false;
  }
}
