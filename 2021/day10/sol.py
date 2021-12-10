from collections import deque
from typing import List, Optional

import numpy as np

OPENING = ["(", "[", "{", "<"]
CLOSING = {"(": ")", "[": "]", "{": "}", "<": ">"}
PENALTIES = {")": 3, "]": 57, "}": 1197, ">": 25137}
REWARDS = {")": 1, "]": 2, "}": 3, ">": 4}


def read_lines(filename: str) -> List[str]:

    with open(filename) as file:
        data = [line.rstrip() for line in file]

    return data


def check_syntax(lines: List[str]) -> int:

    score = 0
    for line in lines:
        stack = deque()
        err = check_line(line, stack)
        if err is not None:
            score += PENALTIES[err]

    return score


def autocomplete(lines: List[str]) -> int:

    scores = []
    for line in lines:
        stack = deque()
        err = check_line(line, stack)
        if err is None:
            score = 0
            stack.reverse()

            for char in stack:
                score *= 5
                score += REWARDS[CLOSING[char]]

            scores.append(score)

    return np.median(scores).astype(np.uint32)


def check_line(line: str, stack: deque) -> Optional[str]:

    for char in line:
        if char in OPENING:
            stack.append(char)

        else:
            opening = stack.pop()
            if char != CLOSING[opening]:
                return char

    return None


if __name__ == "__main__":
    lines = read_lines("d10.txt")

    part1 = check_syntax(lines)
    print(part1)

    part2 = autocomplete(lines)
    print(part2)
