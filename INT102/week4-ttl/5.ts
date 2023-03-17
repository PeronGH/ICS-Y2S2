function findMinMax(
  arr: number[],
  start = 0,
  end = arr.length - 1,
): [number, number] {
  if (start >= end) return [end, end];

  const mid = Math.floor(start + (end - start) / 2);
  const [leftMin, leftMax] = findMinMax(arr, start, mid);
  const [rightMin, rightMax] = findMinMax(arr, mid + 1, end);

  return [
    arr[leftMin] < arr[rightMin] ? leftMin : rightMin,
    arr[leftMax] > arr[rightMax] ? leftMax : rightMax,
  ];
}

console.log(findMinMax([-9, 2, 3, 8, 4, 5, 34, 23, 1]));
