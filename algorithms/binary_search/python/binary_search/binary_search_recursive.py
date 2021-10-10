# returns the index of the target element 
# otherwise, returns -1
def binary_search(nums, target):
    return _binary_search(nums, target, 0, len(nums) - 1)

def _binary_search(nums, target, left, right):
    pivot = left + (right - left) // 2
 
    if nums[pivot] == target:
        return pivot 

    if nums[pivot] < target:
        return _binary_search(nums, target, pivot + 1, right)

    if nums[pivot] > target:
        return _binary_search(nums, target, left, pivot - 1)    


arr = [-1, 0, 3, 5, 9, 12]
assert(4 == binary_search(arr, 9))