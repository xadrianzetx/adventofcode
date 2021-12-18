import itertools
import math
from typing import List, Tuple

Equation = Tuple[List[int], List[int]]


def read_homework(filename: str) -> List[Equation]:

    numbers = []
    with open(filename) as file:
        for line in file:
            line = parse_numbers(line.rstrip())
            numbers.append(line)

    return numbers


def parse_numbers(line: str) -> Equation:

    nums = []
    nest_lvls = []
    nest_lvl = 0
    chrbuff = ""
    for char in line:
        if char == ",":
            if len(chrbuff) > 0:
                nums.append(int(chrbuff))
                nest_lvls.append(nest_lvl)
                chrbuff = ""
        elif char == "[":
            nest_lvl += 1
        elif char == "]":
            if len(chrbuff) > 0:
                nums.append(int(chrbuff))
                nest_lvls.append(nest_lvl)
                chrbuff = ""
            nest_lvl -= 1
        else:
            chrbuff += char

    return nums, nest_lvls


def explode(num: List[int], nest: List[int], idx: int) -> Equation:

    left = num[: idx + 1]
    right = num[idx + 1 :]

    lpop = left.pop()
    if len(left) > 0:
        left[-1] += lpop

    rpop = right.pop(0)
    if len(right) > 0:
        right[0] += rpop
    right.insert(0, 0)

    nest[idx + 1] -= 1
    del nest[idx]

    return left + right, nest


def split(num: List[int], nest: List[int], idx: int) -> Equation:

    ladd, radd = math.floor(num[idx] / 2), math.ceil(num[idx] / 2)
    left, right = num[:idx], num[idx:]
    left.append(ladd)
    right[0] = radd
    nest_lvl = nest[idx]
    lnest, rnest = nest[:idx], nest[idx:]
    lnest.append(nest_lvl + 1)
    rnest[0] += 1

    return left + right, lnest + rnest


def apply_reduction(numbers: List[int], nest_levels: List[int]) -> Equation:

    idx = 0
    should_explode = False
    should_split = False

    while True:
        # To reduce a snailfish number, you must repeatedly do
        # the first action in this list that applies to the snailfish number:
        # - If any pair is nested inside four pairs, the leftmost such pair explodes.
        # - If any regular number is 10 or greater, the leftmost such regular number splits.

        # In other words - all explodes first, then splits. sheesh.

        for i in range(len(numbers)):
            if nest_levels[i] > 4:
                should_explode = True
                idx = i
                break

        if not should_explode:
            for i in range(len(numbers)):
                if numbers[i] >= 10:
                    should_split = True
                    idx = i
                    break

        if should_explode:
            numbers, nest_levels = explode(numbers, nest_levels, idx)
            should_explode = False

        elif should_split:
            numbers, nest_levels = split(numbers, nest_levels, idx)
            should_split = False

        else:
            break

    return numbers, nest_levels


def total_maginitude(numbers: List[int]) -> int:

    nums, nests = numbers[0]
    for new_nums, new_nests in numbers[1:]:
        nums += new_nums
        nests += new_nests
        nests = [n + 1 for n in nests]
        nums, nests = apply_reduction(nums, nests)

    return calc_magnitude(nums, nests)[0]


def largest_magnitude(numbers) -> int:

    max_mag = 0
    for a, b in itertools.permutations(numbers, 2):
        nums = a[0] + b[0]
        nests = a[1] + b[1]
        nests = [n + 1 for n in nests]
        nums, nests = apply_reduction(nums, nests)
        mag, valid = calc_magnitude(nums, nests)
        if valid:
            max_mag = max(mag, max_mag)

    return max_mag


def calc_magnitude(nums: List[int], nest: List[int]) -> Tuple[int, bool]:

    idx = 0
    counter = 0
    valid = True

    while len(nums) > 1:
        if idx + 1 == len(nums):
            idx = 0
            continue

        if nest[idx] == nest[idx + 1]:
            left = nums[idx]
            right = nums[idx + 1]
            nest_level = nest[idx]
            del nums[idx: idx + 2]
            del nest[idx: idx + 2]
            mag = (3 * left) + (2 * right)
            nums.insert(idx, mag)
            nest.insert(idx, nest_level - 1)

        idx += 1
        counter += 1
        if idx >= len(nums):
            idx = 0

        if counter == 100:
            # It was hanging sometimes ¯\_(ツ)_/¯
            valid = False
            break

    return nums[0], valid


if __name__ == "__main__":
    # Barely works, but works ¯\_(ツ)_/¯
    homework = read_homework("d18.txt")
    part1 = total_maginitude(homework)
    print(part1)

    homework = read_homework("d18.txt")
    part2 = largest_magnitude(homework)
    print(part2)
