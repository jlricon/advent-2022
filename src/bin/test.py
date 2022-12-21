from typing import List


def isBadVersion(p):
    return p >= 1


def firstBadVersion(n: int) -> int:
    min_n = 1
    max_n = n
    while (max_n - min_n) >= 1:
        # for _ in range(6):
        pivot = (min_n + max_n) // 2
        print(f"pivot: {pivot}")
        if isBadVersion(pivot):
            max_n = pivot
        else:
            min_n = pivot
        print(min_n, max_n)
    return max_n


print(firstBadVersion(2))
