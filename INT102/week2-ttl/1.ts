// Write the algorithm
const algorithm = (m: number) => {
  let count = 0, x = 1;

  while (x <= m) {
    x = x * 2;
    count = count + 1;
  }

  return { m, count };
};

// Generate trace table
const traceTable = [];
for (let i = 2; i <= 32; i *= 2) {
  traceTable.push(algorithm(i));
}

console.table(traceTable);
