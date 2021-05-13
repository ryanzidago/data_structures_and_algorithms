def merge_sort(a):
    # an array of length 1 is already sorted
    if len(a) > 1:
        # for an array greater than length 1
        # split the array in half
        # call merge_sort on the first and second halves
        mid = len(a) // 2
        left = a[:mid]
        right = a[mid:]

        merge_sort(left)
        merge_sort(right)

        # then, merge the two halves together
        merge(left, right, a)


def merge(left, right, a):
    # two pointers, one for left, one for right
    # if the value from left pointer in left array is less than the value from right pointer in right array 
    left_index = 0
    right_index = 0

    # indexes must not be greater than their containers

    # for as long as the indexes are less
    # than the length of the containers they point to
    while left_index < len(left) and right_index < len(right):
        # then add the value from left pointer into the array `a` being sorted
        # and increment the left pointer by one 
        # otherwise, we do the same, but for right pointer
        if left[left_index] < right[right_index]:
            a[left_index + right_index] = left[left_index]
            left_index += 1
        else:
            a[left_index + right_index] = right[right_index]
            right_index += 1

    # put the leftovers back into the array being sorted
    for left_index in range(left_index, len(left)):
        a[left_index + right_index] = left[left_index]

    for right_index in range(right_index, len(right)):
        a[left_index + right_index] = right[right_index]


# test merge_sort
a = [4, 2, 3, 1]
merge_sort(a)
assert a == [1, 2, 3, 4]

a = [6, 2, 4, 3, 5, 1]
merge_sort(a)
assert a == [1, 2, 3, 4, 5, 6]
