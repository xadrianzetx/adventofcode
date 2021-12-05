from collections import defaultdict
from typing import List, Tuple


def read_vents_coords(filename: str) -> List[Tuple[int]]:

    coord_pairs = []
    with open(filename) as file:
        for line in file:
            x1y1, x2y2 = line.rstrip().split(" -> ")
            x1, y1 = x1y1.split(",")
            x2, y2 = x2y2.split(",")
            pair = (int(x1), int(x2), int(y1), int(y2))
            coord_pairs.append(pair)

    return coord_pairs


def find_vents(coords: List[Tuple[int]], part2: bool = False) -> int:

    vents = defaultdict(int)
    for x1, x2, y1, y2 in coords:
        dx = 1 if x2 > x1 else -1
        dy = 1 if y2 > y1 else -1

        if x1 == x2 or y1 == y2:
            for x in range(abs(x1 - x2) + 1):
                for y in range(abs(y1 - y2) + 1):
                    c = (x1 + dx * x, y1 + dy * y)
                    vents[c] += 1

        else:
            if part2:
                for step in range(abs(x1 - x2) + 1):
                    c = (x1 + dx * step, y1 + dy * step)
                    vents[c] += 1

    counts = [1 for val in vents.values() if val > 1]
    return sum(counts)


if __name__ == "__main__":
    coords = read_vents_coords("d5.txt")

    part1 = find_vents(coords)
    print(part1)

    part2 = find_vents(coords, part2=True)
    print(part2)
