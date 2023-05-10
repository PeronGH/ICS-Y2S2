interface TspState {
  adj: number[][];
  currBound: number;
  currWeight: number;
  currPath: number[];
}

function findMins(adj: number[][], i: number): [number, number] {
  const others = adj[i].filter((_, j) => j !== i);
  const sorted = others.sort((a, b) => a - b);
  return [sorted[0], sorted[1] || Number.MAX_SAFE_INTEGER];
}

function TSPRec(state: TspState): { finalRes: number; finalPath: number[] } {
  const { adj, currPath } = state;
  let { currBound, currWeight } = state;
  let finalRes = Number.MAX_SAFE_INTEGER;
  let finalPath: number[] = [];

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

  for (let i = 0; i < adj.length; i++) {
    if (adj[currPath[currPath.length - 1]][i] !== 0 && !currPath.includes(i)) {
      const temp = currBound;
      currWeight += adj[currPath[currPath.length - 1]][i];

      const [firstMin] = findMins(adj, i);
      if (currPath.length === 1) {
        currBound -=
          (findMins(adj, currPath[currPath.length - 1])[0] + firstMin) / 2;
      } else {
        currBound -=
          (findMins(adj, currPath[currPath.length - 1])[1] + firstMin) / 2;
      }

      if (currBound + currWeight < finalRes) {
        currPath.push(i);
        const result = TSPRec({ adj, currBound, currWeight, currPath });
        if (result.finalRes < finalRes) {
          finalRes = result.finalRes;
          finalPath = result.finalPath;
        }
        currPath.pop();
      }

      currWeight -= adj[currPath[currPath.length - 1]][i];
      currBound = temp;
    }
  }

  return { finalRes, finalPath };
}

function TSP(adj: number[][]): { finalRes: number; finalPath: number[] } {
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
