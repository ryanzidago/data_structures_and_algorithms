# O(2^N)
def fib1(n)
    return n if n < 2
    return fib1(n - 2) + fib1(n - 1)
end

# memoization
# (2 * N) - 1 
# O(N)
def fib2(n, memo = {0 => 0, 1 => 1})
    return n if n < 2
    
    if memo[n].nil?
        memo[n] = fib2(n - 2, memo) + fib2(n - 1, memo)
    end

    return memo[n]
end

# iterative 
# O(N)
def fib3(n)
    return 0 if n == 0
    current_n = 0 
    next_n = 1

    for _ in 1..(n - 1)
        current_n, next_n = next_n, current_n + next_n
    end

    return next_n
end

puts(fib2(100))
puts(fib3(100))