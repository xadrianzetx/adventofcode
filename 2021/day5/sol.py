from collections import defaultdict
from typing import List, Tuple

import numpy as np


def read_vents_coords(filename: str) -> List[Tuple[int]]:

    coords = []
    with open(filename) as file:
        for line in file:
            x1y1, x2y2 = line.rstrip().split(" -> ")
            x1, y1 = x1y1.split(",")
            x2, y2 = x2y2.split(",")
            coord = (int(x1), int(x2), int(y1), int(y2))
            coords.append(coord)

    return coords


def find_vents(coords: List[Tuple[int]], nodiag: bool = True) -> int:

    vents = defaultdict(int)
    for x1, x2, y1, y2 in coords:
        dx = np.sign(x2 - x1)
        dy = np.sign(y2 - y1)

        if dx != 0 and dy != 0 and nodiag:
            continue

        rng = x1 - x2 or y1 - y2
        for step in range(abs(rng) + 1):
            c = (x1 + dx * step, y1 + dy * step)
            vents[c] += 1

    counts = [1 for val in vents.values() if val > 1]
    return sum(counts)


if __name__ == "__main__":
    coords = read_vents_coords("d5.txt")

    part1 = find_vents(coords)
    print(part1)

    part2 = find_vents(coords, nodiag=False)
    print(part2)
