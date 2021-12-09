from typing import Dict

ADJACENT = [1 + 0j, -1 + 0j, 0 + 1j, 0 - 1j]


def make_height_map(filename: str) -> Dict[complex, int]:

    location = 0 + 0j
    height_map = {}
    with open(filename) as file:
        for line in file:
            heights = list(line.rstrip())
            for h in heights:
                height_map[location] = int(h)
                location += 1
            location += -location.real + 1j

    return height_map


def find_low_points(hmap: Dict[complex, int]) -> int:

    risk = 0
    for coord, val in hmap.items():
        neighbors = []

        for offset in ADJACENT:
            neighbor = coord + offset
            if neighbor in hmap:
                neighbors.append(hmap.get(neighbor))
        is_low = all([val < n for n in neighbors])

        if is_low:
            risk += 1 + val

    return risk


if __name__ == "__main__":
    hmap = make_height_map("d9.txt")

    part1 = find_low_points(hmap)
    print(part1)
