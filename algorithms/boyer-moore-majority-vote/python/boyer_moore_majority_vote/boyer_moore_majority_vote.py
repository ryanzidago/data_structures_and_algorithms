def get_majority_element(elements):
    candidate = None
    count = 0

    for element in elements:
        if count == 0:
            candidate = element

        if candidate == element:
            count += 1
        else:
            count -= 1

    return candidate


elements = [1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 4, 5, 6, 7, 8, 9, 2]
assert(get_majority_element(elements) == 2)
