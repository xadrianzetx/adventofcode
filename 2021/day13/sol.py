from collections import defaultdict
from typing import Dict

import numpy as np


def read_instructions(filename: str) -> Dict[complex, int]:

    dotmap = defaultdict(int)
    with open(filename) as file:
        for line in file:
            pair = [int(x) for x in line.rstrip().split(",")]
            coord = complex(*pair)
            dotmap[coord] = 1

    return dotmap


def fold(dotmap: Dict[complex, int], axis: complex) -> None:

    pairs = []
    for coord in dotmap:
        if coord.real > axis.real > 0:
            offset = coord.real - axis.real
            newx = complex(axis.real - offset, coord.imag)
            pairs.append((coord, newx))

        elif coord.imag > axis.imag > 0:
            offset = coord.imag - axis.imag
            newy = complex(coord.real, axis.imag - offset)
            pairs.append((coord, newy))

    for rm, add in pairs:
        dotmap.pop(rm)
        dotmap[add] += 1


def visualize(dotmap: Dict[complex, int]) -> None:

    canvas = np.full((40, 40), ".")  # Big enough.
    for coord in dotmap:
        canvas[int(coord.real), int(coord.imag)] = "#"

    canvas = np.flipud(canvas)
    for line in canvas:
        print("".join(line))


if __name__ == "__main__":
    instructions = [
        655 + 0j,
        0 + 447j,
        327 + 0j,
        0 + 223j,
        163 + 0j,
        0 + 111j,
        81 + 0j,
        0 + 55j,
        40 + 0j,
        0 + 27j,
        0 + 13j,
        0 + 6j,
    ]

    checksum = True
    dotmap = read_instructions("d13.txt")
    for instruction in instructions:
        fold(dotmap, instruction)
        if checksum:
            checksum = False
            print(len(dotmap))  # Part 1.

    visualize(dotmap)  # Part 2.
