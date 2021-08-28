def rotate_right(array, k)
    k.times do
        array.unshift(array.pop())
    end
end

