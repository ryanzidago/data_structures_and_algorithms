def insertion_sort(a)
    if a.length <= 1
        return
    end

    for i in 1..(a.length() - 1)
        j = i
        while j > 0 and a[j - 1] > a[j]
            a[j - 1], a[j] = a[j], a[j - 1]
            j -= 1
        end
    end
end
