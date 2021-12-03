from copy import deepcopy

import numpy as np


def read_bitmap(filename: str) -> np.array:

    with open(filename) as file:
        data = [list(line.rstrip()) for line in file.readlines()]
    return np.array(data, dtype=np.int8)


def calc_power_consumption(bitmap: np.array) -> int:

    gamma_arr = np.round(np.mean(bitmap, axis=0))
    gamma = int("".join([str(el) for el in gamma_arr.astype(int)]), 2)

    return gamma * (2 ** bitmap.shape[1] + ~gamma)


def calc_oxy(bitmap: np.array) -> int:

    bitmapcp = deepcopy(bitmap)
    for idx in range(bitmap.shape[1]):
        common = np.round(np.mean(bitmapcp, axis=0) + 1e-10)
        bitmapcp = bitmapcp[bitmapcp[:, idx] == common[idx]]

    return int("".join([str(int(el)) for el in bitmapcp[0]]), 2)


def calc_co2(bitmap: np.array) -> int:

    bitmapcp = deepcopy(bitmap)
    for idx in range(bitmap.shape[1]):
        common = np.round(np.mean(bitmapcp, axis=0) + 1e-10)
        bitmapcp = bitmapcp[bitmapcp[:, idx] != common[idx]]
        if len(bitmapcp) == 1:
            break

    return int("".join([str(int(el)) for el in bitmapcp[0]]), 2)


if __name__ == "__main__":
    bitmap = read_bitmap("d3.txt")

    part1 = calc_power_consumption(bitmap)
    print(part1)

    # maybe refactor
    part2 = calc_oxy(bitmap) * calc_co2(bitmap)
    print(part2)
