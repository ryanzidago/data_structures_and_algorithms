def get_majority_element(elements)
    candidate = nil 
    count = 0

    elements.each do |element|
        candidate = count == 0 ? candidate = element : candidate
        count += candidate == element ? +1 : -1
    end

    candidate
end

elements = [1, 1, 1, 1, 3, 5, 7, 6]
puts(get_majority_element(elements) == 1)