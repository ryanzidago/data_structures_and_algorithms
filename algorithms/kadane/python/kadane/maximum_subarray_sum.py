def maximum_subarray_sum(nums):
    return _maximum_subarray_sum(nums) if len(nums) > 0 else 0


def _maximum_subarray_sum(nums):
    current_sum, maximum_sum = nums[0], nums[0]

    for num in nums[1:]:
        current_sum = max(num, current_sum + num)
        maximum_sum = max(maximum_sum, current_sum)

    return maximum_sum


assert(maximum_subarray_sum([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6)
assert(maximum_subarray_sum([1]) == 1)
assert(maximum_subarray_sum([5, 4, -1, 7, 8]) == 23)
assert(maximum_subarray_sum([-2, 1]) == 1)
assert(maximum_subarray_sum([1, 2, -5, 2]) == 3)
