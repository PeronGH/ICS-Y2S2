let cmpCount = 0;

function mergeSort(A: number[]) {
  const n = A.length;

  if (n > 1) {
    const mid = Math.floor(n / 2);
    const B = A.slice(0, mid);
    const C = A.slice(mid);

    mergeSort(B);
    mergeSort(C);
    merge(B, C, A);
  }

  return A;
}

function merge(B: number[], C: number[], A: number[]) {
  let i = 0, j = 0, k = 0;
  const p = B.length;
  const q = C.length;

  while (i < p && j < q) {
    cmpCount++;
    if (B[i] <= C[j]) {
      A[k] = B[i];
      i++;
    } else {
      A[k] = C[j];
      j++;
    }
    k++;
  }

  if (i === p) {
    A.splice(k, p + q - k, ...C.slice(j));
  } else {
    A.splice(k, p + q - k, ...B.slice(i));
  }
}

console.log(mergeSort([6, 1, 2, 3, 4, 5]));
console.log(`cmpCount = ${cmpCount}`);
