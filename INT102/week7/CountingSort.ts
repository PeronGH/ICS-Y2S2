function countingSort(arr: number[], maxValue: number): number[] {
  // Initialize the count array
  const countArray: number[] = new Array(maxValue + 1).fill(0);

  // Count the occurrences of each element in the input array
  for (const element of arr) {
    countArray[element]++;
  }

  console.log("Count Array:");
  console.table(countArray);

  // Calculate the cumulative sum of the count array
  for (let i = 1; i < countArray.length; i++) {
    countArray[i] += countArray[i - 1];
  }

  console.log("Cumulative Sum:");
  console.table(countArray);

  // Create an output array to store the sorted elements
  const outputArray: number[] = new Array(arr.length).fill(0);

  // Traverse the input array in reverse order and place the elements in the output array
  for (let i = arr.length - 1; i >= 0; i--) {
    console.log(
      "Placing",
      arr[i],
      "at index",
      countArray[arr[i]] - 1,
      `which is countArray[${arr[i]}] - 1`,
    );
    outputArray[countArray[arr[i]] - 1] = arr[i];
    countArray[arr[i]]--;
  }

  // Return the sorted output array
  return outputArray;
}

// Example usage
Deno.test("Counting Sort", () => {
  const arr: number[] = [4, 2, 2, 8, 3, 3, 1];
  const maxValue: number = Math.max(...arr);

  const sortedArray: number[] = countingSort(arr, maxValue);

  console.log("Sorted Array:", sortedArray);
});
