import numpy as np
from scipy.ndimage import label
from scipy.ndimage.filters import generic_filter


def make_height_map(filename: str) -> np.array:

    data = []
    with open(filename) as file:
        for line in file:
            heights = list(line.rstrip())
            data.append(heights)

    return np.array(data, dtype=np.int32)


def find_low_points(hmap: np.array) -> int:
    def _kernel(arr: np.array) -> int:
        neighbors = arr[1::2]
        val = arr[4]
        is_low = all([val < n for n in neighbors])
        return 1 if is_low else 0

    minima = generic_filter(hmap, _kernel, size=3, mode="constant", cval=np.inf)
    risk = np.sum(hmap[minima == 1] + 1)

    return risk


def find_basins(hmap: np.array) -> int:

    hmap += 1
    hmap[hmap == 10] = 0

    clusters, _ = label(hmap)
    counts = np.bincount(clusters.flatten())
    largest = np.prod(sorted(counts[1:])[-3:])

    return largest


if __name__ == "__main__":
    hmap = make_height_map("d9.txt")

    part1 = find_low_points(hmap)
    print(part1)

    part2 = find_basins(hmap)
    print(part2)
