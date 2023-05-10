interface TspState {
  adj: number[][];
  currBound: number;
  currWeight: number;
  level: number;
  currPath: number[];
  visited: boolean[];
}

function findMins(adj: number[][], i: number): [number, number] {
  const others = adj[i].filter((_, j) => j !== i);
  const sorted = others.sort((a, b) => a - b);
  return [sorted[0], sorted[1]];
}

function TSPRec(state: TspState): { finalRes: number; finalPath: number[] } {
  let { adj, currBound, currWeight, level, currPath, visited } = state;
  let finalRes = Number.MAX_SAFE_INTEGER;
  let finalPath = Array(currPath.length + 1).fill(-1);

  if (level === currPath.length) {
    if (adj[currPath[level - 1]][currPath[0]] !== 0) {
      const currRes = currWeight + adj[currPath[level - 1]][currPath[0]];
      if (currRes < finalRes) {
        finalPath = [...currPath, currPath[0]];
        finalRes = currRes;
      }
    }
    return { finalRes, finalPath };
  }

  for (let i = 0; i < adj.length; i++) {
    if (adj[currPath[level - 1]][i] !== 0 && !visited[i]) {
      const temp = currBound;
      currWeight += adj[currPath[level - 1]][i];

      const [firstMin] = findMins(adj, i);
      if (level === 1) {
        currBound -= (findMins(adj, currPath[level - 1])[0] + firstMin) / 2;
      } else {
        currBound -= (findMins(adj, currPath[level - 1])[1] + firstMin) / 2;
      }

      if (currBound + currWeight < finalRes) {
        currPath[level] = i;
        visited[i] = true;
        const result = TSPRec({
          adj,
          currBound,
          currWeight,
          level: level + 1,
          currPath,
          visited,
        });
        if (result.finalRes < finalRes) {
          finalRes = result.finalRes;
          finalPath = result.finalPath;
        }
      }

      currWeight -= adj[currPath[level - 1]][i];
      currBound = temp;
      visited = visited.map((_, j) => j <= level - 1);
    }
  }

  return { finalRes, finalPath };
}

function TSP(adj: number[][]): { finalRes: number; finalPath: number[] } {
  const currPath = Array(adj.length).fill(-1);
  let currBound = 0;
  const visited = Array(adj.length).fill(false);

  for (let i = 0; i < adj.length; i++) {
    const [firstMin, secondMin] = findMins(adj, i);
    currBound += firstMin + secondMin;
  }

  currBound = currBound % 2 === 1 ? (currBound / 2) + 1 : currBound / 2;
  visited[0] = true;
  currPath[0] = 0;

  return TSPRec({ adj, currBound, currWeight: 0, level: 1, currPath, visited });
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
