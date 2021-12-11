from typing import Tuple

import numpy as np

OFFSETS = [[1, 0], [1, -1], [0, -1], [-1, -1], [-1, 0], [-1, 1], [0, 1], [1, 1]]


def read_energy_levels(filename: str) -> np.array:

    data = []
    with open(filename) as file:
        for line in file:
            data.append(list(line.rstrip()))

    return np.array(data, dtype=np.uint8)


def model_flashes(energy: np.array) -> Tuple[int, int]:

    n_flashes = 0
    iter_count = 0
    while True:
        if np.sum(energy) == 0:
            return n_flashes, iter_count

        energy += 1
        while True:
            flashing = np.argwhere(energy > 9)
            if len(flashing) == 0:
                break

            for pos in flashing:
                set_neighbors(pos, energy)
                energy[pos[0], pos[1]] = 0
                if iter_count < 100:
                    n_flashes += 1

        iter_count += 1


def set_neighbors(pos: np.array, energy: np.array) -> None:

    h, w = energy.shape
    for offset in OFFSETS:
        neigh = pos + offset
        if 0 <= neigh[0] < h and 0 <= neigh[1] < w:
            if energy[neigh[0], neigh[1]] != 0:
                energy[neigh[0], neigh[1]] += 1


if __name__ == "__main__":
    energy = read_energy_levels("d11.txt")

    part1, part2 = model_flashes(energy)
    print(part1)
    print(part2)
