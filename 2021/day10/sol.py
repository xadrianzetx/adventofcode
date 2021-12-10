from collections import deque
from typing import List, Optional, Tuple

import numpy as np

OPENING = ["(", "[", "{", "<"]
CLOSING = {"(": ")", "[": "]", "{": "}", "<": ">"}
PENALTIES = {")": 3, "]": 57, "}": 1197, ">": 25137}
POINTS = {")": 1, "]": 2, "}": 3, ">": 4}


def read_lines(filename: str) -> List[str]:

    with open(filename) as file:
        data = [line.rstrip() for line in file]

    return data


def check_lines(lines: List[str]) -> int:

    score = 0
    for line in lines:
        score += check_line(line)[0]

    return score


def autocomplete(lines: List[str]) -> int:

    scores = []
    for line in lines:
        _, stack = check_line(line)
        if stack is not None:
            score = 0
            stack.reverse()

            for char in stack:
                score *= 5
                score += POINTS[CLOSING[char]]

            scores.append(score)

    return np.median(scores)


def check_line(line: str) -> Tuple[int, Optional[deque]]:

    stack = deque()
    for char in line:
        if char in OPENING:
            stack.append(char)

        else:
            opening = stack.pop()
            if char != CLOSING[opening]:
                return PENALTIES[char], None

    return 0, stack


if __name__ == "__main__":
    lines = read_lines("d10.txt")

    part1 = check_lines(lines)
    print(part1)

    part2 = autocomplete(lines)
    print(part2)
