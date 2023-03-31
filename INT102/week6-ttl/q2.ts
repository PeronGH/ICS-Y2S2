function Question2(n: number, cache = Array<bigint>(n + 1)): bigint {
  if (cache[n]) return cache[n]!;

  let answer: bigint;

  if (0 <= n && n <= 3) answer = 1n;
  else {answer = Question2(n - 1, cache) +
      Question2(n - 2, cache) +
      Question2(n - 3, cache) +
      Question2(n - 4, cache);}

  console.log(n, answer);
  cache[n] = answer;
  return answer;
}

Question2(100); // O(n)

function Q2BottomUp(n: number): bigint {
  const cache = Array<bigint>(n + 1);
  cache.fill(1n, 0, 4);
  for (let i = 4; i <= n; i++) {
    cache[i] = cache[i - 1] + cache[i - 2] + cache[i - 3] + cache[i - 4];
  }

  return cache[n];
}

console.log(Q2BottomUp(100)); // O(n)
