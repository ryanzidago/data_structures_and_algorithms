# returns the index of the target element 
# otherwise, returns -1
def binary_search(nums, target):
    left, right = 0, len(nums) - 1 

    while left <= right:
        pivot = left + (right - left) // 2

        if nums[pivot] == target:
            return pivot 
        if target < nums[pivot]:
            right = pivot - 1 
        else: 
            left = pivot + 1

    return - 1


arr = [-1, 0, 3, 5, 9, 12]
assert(4 == binary_search(arr, 9))