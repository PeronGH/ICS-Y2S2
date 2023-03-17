let cmpCount = 0, swapCount = 0;

function bubbleSort(A: number[]) {
  // Sorts a given array by bubble sort
  // Input: An array A[0..n-1] of orderable elements
  // Output: Array A[0..n-1] sorted in ascending order

  const n = A.length;

  for (let i = 0; i < n - 1; i++) {
    for (let j = n - 1; j > i; j--) {
      cmpCount++;
      if (A[j] < A[j - 1]) {
        // Swap A[j] and A[j - 1]
        [A[j], A[j - 1]] = [A[j - 1], A[j]];
        swapCount++;
      }
    }
  }

  return A;
}

console.log(bubbleSort([6, 1, 2, 3, 4, 5]));
console.log(`cmpCount = ${cmpCount}, swapCount = ${swapCount}`);
