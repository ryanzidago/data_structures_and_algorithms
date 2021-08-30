def rotate_right(array, k)
    (k % array.length).times do
        array.unshift(array.pop())
    end
end

