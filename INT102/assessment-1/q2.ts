export function F(n: number, cache = Array<bigint>(n + 1)): bigint {
  if (cache[n]) return cache[n]!;

  let answer: bigint;

  if (0 <= n && n <= 3) answer = 1n;
  else {answer = F(n - 1, cache) +
      F(n - 2, cache) +
      F(n - 3, cache) +
      F(n - 4, cache);}

  cache[n] = answer;
  return answer;
}
