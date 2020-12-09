import collections
from typing import List

file = open("day9.txt", "r")
data = [int(x) for x in file.readlines()]


# O(N*K)
def weak_number(preamble: int, xmas_data: List[int]) -> int:
    sums = collections.defaultdict(int)
    for i in range(preamble):
        for j in range(i + 1, preamble):
            sums[xmas_data[i] + xmas_data[j]] += 1

    for i in range(preamble, len(xmas_data)):
        if xmas_data[i] not in sums:
            return xmas_data[i]

        for j in range(i - preamble, i):
            sums[xmas_data[i] + xmas_data[j]] += 1
        for j in range(i - preamble + 1, i + 1):
            to_removed_sum = xmas_data[i - preamble] + xmas_data[j]
            sums[to_removed_sum] -= 1
            if sums[to_removed_sum] == 0:
                del sums[to_removed_sum]
    assert False, "Cannot find the weak number®"


def encryption_weakness(target: int, xmas_data) -> int:
    for i in range(len(xmas_data)):
        current = xmas_data[i]
        for j in range(i + 1, len(xmas_data)):
            current += xmas_data[j]
            if current == target:
                return min(xmas_data[i: j + 1]) + max(xmas_data[i: j + 1])
    assert False, "Cannot find the encryption weakness"


test_input = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576]
test_output_part1 = weak_number(5, test_input)
assert test_output_part1 == 127

test_output_part2 = encryption_weakness(test_output_part1, test_input)
assert test_output_part2 == 62

part1 = weak_number(25, data)
print(part1)

part2 = encryption_weakness(part1, data)
print(part2)
