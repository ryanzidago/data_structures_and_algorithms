def merge_sort(a)
    if a.length > 1
        mid = a.length / 2
        left = a[..(mid -1)]
        right = a[mid..]

        merge_sort(left)
        merge_sort(right)
        merge(left, right, a)
    end
end

private

def merge(left, right, a)
    left_index = 0
    right_index = 0

    while left_index < left.length and right_index < right.length
        if left[left_index] < right[right_index]
            a[left_index + right_index] = left[left_index]
            left_index += 1
        else
            a[left_index + right_index] = right[right_index]
            right_index += 1
        end
    end

    for left_index in left_index..(left.length - 1)
        a[left_index + right_index] = left[left_index]
    end

    for right_index in right_index..(right.length - 1)
        a[left_index + right_index] = right[right_index]
    end
end
