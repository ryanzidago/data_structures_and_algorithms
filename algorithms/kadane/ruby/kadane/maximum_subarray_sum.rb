def maximum_subarray_sum(nums)
    nums.length > 0 ? _maximum_subarray_sum(nums) : 0
end


def _maximum_subarray_sum(nums)
    current_sum, maximum_sum = nums[0], nums[0]
        
    nums[1..].each do |num|
        current_sum = num > current_sum + num ? num : current_sum + num
        maximum_sum = maximum_sum > current_sum ? maximum_sum : current_sum
    end

    maximum_sum
end

puts(maximum_subarray_sum([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6)
puts(maximum_subarray_sum([1]) == 1)
puts(maximum_subarray_sum([5, 4, -1, 7, 8]) == 23)
puts(maximum_subarray_sum([-2, 1]) == 1)
puts(maximum_subarray_sum([1, 2, -5, 2]) == 3)