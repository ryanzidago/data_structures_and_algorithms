def is_palindrome(string):
    left = 0
    right = len(string) - 1

    while left <= right:
        if not string[left].isalnum():
            left += 1
            continue

        if not string[right].isalnum():
            right -= 1
            continue

        if string[left].lower() != string[right].lower():
            return False
        else:
            left += 1
            right -= 1
    return True


string = "A man, a plan, a canal: Panama"
assert(is_palindrome(string))

string = "race a car"
assert(not is_palindrome(string))
