def bubble_sort(arr):
    cmp, swap = 0, 0
    n = len(arr)

    for i in range(n - 1):
        for j in range(n - 1, i, -1):
            cmp += 1
            if arr[j] < arr[j - 1]:
                swap += 1
                arr[j], arr[j - 1] = arr[j - 1], arr[j]

    return arr, cmp, swap


A = [3, 4, 5, 3, 4, 5]
sorted_array, cmp, swap = bubble_sort(A)
print(sorted_array, f'cmp: {cmp}, swap: {swap}')
