def merge(left, right):
    merged = []
    left_idx, right_idx = 0, 0

    while left_idx < len(left) and right_idx < len(right):
        if left[left_idx] < right[right_idx]:
            merged.append(left[left_idx])
            left_idx += 1
        else:
            merged.append(right[right_idx])
            right_idx += 1

    if left_idx < len(left):
        merged.extend(left[left_idx:])
    if right_idx < len(right):
        merged.extend(right[right_idx:])

    return merged


def merge_sort_iterative(arr):
    if len(arr) <= 1:
        return arr

    step = 1
    while step < len(arr):  # outer loop complexity: ceil(log2(len(arr)))
        left = 0
        while left < len(arr):  # inner loop complexity: n / (2 * step)
            mid = min(left + step, len(arr))
            right = min(left + 2 * step, len(arr))
            print(f'{arr[left:mid]} + {arr[mid:right]} = ', end='')
            arr[left:right] = merge(arr[left:mid], arr[mid:right])
            print(arr[left:right])
            left += 2 * step

        print(f'--- Step of {step} finished ---')
        step *= 2

    return arr


# Test the iterative merge sort
arr = [64, 34, 25, 12, 22, 11, 90]
print("Unsorted array is:", arr)
sorted_arr = merge_sort_iterative(arr)
print("Sorted array is:", sorted_arr)
