from typing import Dict, Generator
from functools import lru_cache

# infinite recursive /!\ 
def fib1(n: int) -> int:
    return fib1(n - 1) + fib2(n - 2)

# recursive
# O(2^N)
def fib2(n: int) -> int:
    if n < 2:
        return n
    return fib2(n - 1) + fib2(n - 2)

# memoization 
# O(N)
def fib3(n: int, memo: Dict[int, int] = {0: 0, 1: 1}) -> int:
    print(memo)
    if n not in memo:
        memo[n] = fib3(n - 1) + fib3(n - 2) # memoization 
    return memo[n]

# LRU cachce
# O(N)
@lru_cache(maxsize=None)
def fib4(n: int) -> int:
    if n < 2:
        return n 
    return fib4(n - 1) + fib4(n - 2)

# iterative
# O(N)
def fib5(n):
    if n == 0: return n
    last = 0 
    next = 1
    for _ in range(1, n):
        last, next = next, last + next 
    return next

# yielding all values
def fib6(n):
    yield 0
    if n > 0:
        yield 1
    last = 0
    next = 1
    for _ in range(1, n):
        last, next = next, last + next 
        yield next
