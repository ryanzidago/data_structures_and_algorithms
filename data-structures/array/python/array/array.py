# slow implementations
# do not pass leetcode's test cases: https://leetcode.com/problems/rotate-array/

def rotate_left(array, k):
    for _ in range(k % len(array)):
        _rotate_left(array)

def _rotate_left(array):
    n = len(array)
    temp = array[0]
    
    for i in range(n - 1):
        array[i] = array[i + 1]
    
    array[-1] = temp

def rotate_right(array, k):
    for _ in range(k % len(array)):
        _rotate_right(array)

def _rotate_right(array):
    n = len(array)
    temp = array[-1]

    for i in range(1, n):
        array[-i] = array[-i - 1]
    
    array[0] = temp


# rotate_left test 
array = [1, 2, 3]
k = 1
rotate_left(array, k)
assert(array == [2, 3, 1])


# rotate_right test
array = [-1, -100, 3, 99]
k = 2
rotate_right(array, k)

assert(array == [3, 99, -1, -100])