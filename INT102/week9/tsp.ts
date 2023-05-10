interface TspState {
  adj: number[][];
  currBound: number;
  currWeight: number;
  currPath: number[];
}

interface TSPResult {
  finalRes: number;
  finalPath: number[];
}

function findMins(adj: number[][], i: number): [number, number] {
  const others = adj[i].filter((_, j) => j !== i);
  const sorted = others.sort((a, b) => a - b);
  return [sorted[0], sorted[1] || Number.MAX_SAFE_INTEGER];
}

/**
 * Recursive function to find the shortest path
 * using depth-first search and branch-and-bound.
 *
 * @param state - An object that stores the current state of the search, including
 * the adjacency matrix, current lower bound of the path's cost, current weight of
 * the path, and the nodes visited in the current path.
 * @returns An object containing the minimum cost of the path and the final path.
 */
function TSPRec(state: TspState): TSPResult {
  const { adj, currPath } = state;
  let { currBound, currWeight } = state;
  let finalRes = Number.MAX_SAFE_INTEGER;
  let finalPath: number[] = [];

  // If all nodes have been visited, check if there is an edge from the last node in
  // the path back to the first node (to form a cycle). If so, compare the total weight
  // of this path with the current minimum, and update the minimum if necessary.
  if (currPath.length === adj.length) {
    if (adj[currPath[currPath.length - 1]][currPath[0]] !== 0) {
      const currRes = currWeight +
        adj[currPath[currPath.length - 1]][currPath[0]];
      if (currRes < finalRes) {
        finalPath = [...currPath, currPath[0]];
        finalRes = currRes;
      }
    }
    return { finalRes, finalPath };
  }

  // If not all nodes have been visited, try to add each unvisited node to the path.
  for (let i = 0; i < adj.length; i++) {
    // Check if there is an edge from the current node to node i and that node i has not been visited.
    if (adj[currPath[currPath.length - 1]][i] !== 0 && !currPath.includes(i)) {
      // Remember the current lower bound to restore later.
      const temp = currBound;
      // Add the weight of the edge to node i to the current weight.
      currWeight += adj[currPath[currPath.length - 1]][i];

      // Recalculate the lower bound of the path's cost.
      const [firstMin] = findMins(adj, i);
      if (currPath.length === 1) {
        currBound -=
          (findMins(adj, currPath[currPath.length - 1])[0] + firstMin) / 2;
      } else {
        currBound -=
          (findMins(adj, currPath[currPath.length - 1])[1] + firstMin) / 2;
      }

      // If the current lower bound plus the current weight of the path is less than
      // the current minimum, add node i to the path and recurse. If the result is better
      // than the current minimum, update the minimum.
      if (currBound + currWeight < finalRes) {
        currPath.push(i);
        const result = TSPRec({ adj, currBound, currWeight, currPath });
        if (result.finalRes < finalRes) {
          finalRes = result.finalRes;
          finalPath = result.finalPath;
        }
        currPath.pop();
      }

      // Restore the weight and the lower bound for the next iteration.
      currWeight -= adj[currPath[currPath.length - 1]][i];
      currBound = temp;
    }
  }

  return { finalRes, finalPath };
}

export function TSP(adj: number[][]): TSPResult {
  const currPath = [0];
  let currBound = 0;

  for (let i = 0; i < adj.length; i++) {
    const [firstMin, secondMin] = findMins(adj, i);
    currBound += firstMin + secondMin;
  }

  currBound = currBound % 2 === 1
    ? Math.floor(currBound / 2) + 1
    : currBound / 2;

  return TSPRec({ adj, currBound, currWeight: 0, currPath });
}

//Adjacency matrix for the given graph
const adj = [
  [0, 4, 5, 2, 1], // a
  [4, 0, 4, 3, 1], // b
  [5, 4, 0, 1, 8], // c
  [2, 3, 1, 0, 6], // d
  [1, 1, 8, 6, 0], // e
];

const result = TSP(adj);

console.log(`Minimum cost: ${result.finalRes}`);
console.log(`Path Taken: ${result.finalPath.join(" ")}`);
