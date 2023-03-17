function findLargest(
  array: number[],
  start = 0,
  end = array.length - 1,
): number {
  if (start >= end) return end;

  const mid = Math.floor(start + (end - start) / 2);
  const leftLargest = findLargest(array, start, mid);
  const rightLargest = findLargest(array, mid + 1, end);

  return array[leftLargest] > array[rightLargest] ? leftLargest : rightLargest;
}

console.log(findLargest([1, 2, 3, 8, 4, 5, 34, 23, 1]));
