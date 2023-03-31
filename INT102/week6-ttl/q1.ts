const edges = [
  "ab",
  "ae",
  "bc",
  "bf",
  "cd",
  "cg",
  "ef",
  "fg",
  "gh",
].flatMap(([a, b]) => [a + b, b + a]);
console.log(edges);

const nodes = new Set([...edges].flatMap(([a, b]) => [a, b]));
console.log(nodes);

// adjacency matrix and adjacency list
const adjList = new Map<string, Array<string>>(
  [...nodes].map((
    node,
  ) => [node, edges.flatMap(([s, e]) => s === node ? [e] : [])]),
);

console.log(adjList);

const adjMatrix: Array<Array<boolean>> = [...adjList].map(([_s, es]) =>
  [...nodes].map((e) => es.includes(e))
);

console.table(adjMatrix);

// incidence matrix and incidence list
const incList = new Map<string, Array<string>>(
  [...nodes].map((node) => [node, edges.filter(([s]) => s === node)]),
);
console.log(incList);
