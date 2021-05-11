# Time complexity of insertion sort:

# worst case: O(n²)
# average case: O(n²)
# best case: O(n)

# Space complexity of insertion sort:
# O(1) (in place)
def insertion_sort(a):
    for i in range(1, len(a)):
        j = i 
        while j > 0 and a[j - 1] > a[j]:
            a[j - 1], a[j] = a[j], a[j - 1]
            j -= 1

# test 
a = [5, 2, 4, 6, 1, 3]
insertion_sort(a)
assert a == [1, 2, 3, 4, 5, 6]

# test 
a = [4, 2, 7, 1, 3]
insertion_sort(a)
assert a == [1, 2, 3, 4, 7]