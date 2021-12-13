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

    to_pop = []
    to_add = []

    if axis.real > 0:
        for key in dotmap.keys():
            if key.real > axis.real:
                offset = key.real - axis.real
                newx = complex(axis.real - offset, key.imag)
                to_add.append(newx)
                to_pop.append(key)

    else:
        for key in dotmap.keys():
            if key.imag > axis.imag:
                offset = key.imag - axis.imag
                newy = complex(key.real, axis.imag - offset)
                to_add.append(newy)
                to_pop.append(key)

    for key in to_pop:
        dotmap.pop(key)

    for key in to_add:
        dotmap[key] += 1

def visualize(dotmap: Dict[complex, int]) -> None:

    canvas = np.full((40, 40), ".")
    for coord in dotmap:
        canvas[int(coord.real), int(coord.imag)] = "#"

    canvas = np.flipud(canvas)
    for line in canvas:
        print("".join(line))


if __name__ == "__main__":
    dotmap = read_instructions("d13.txt")

    # fold along x=655
    # fold along y=447
    # fold along x=327
    # fold along y=223
    # fold along x=163
    # fold along y=111
    # fold along x=81
    # fold along y=55
    # fold along x=40
    # fold along y=27
    # fold along y=13
    # fold along y=6

    # instructions = [0+7j, 5+0j]
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
    for instruction in instructions:
        fold(dotmap, instruction)

    visualize(dotmap)
