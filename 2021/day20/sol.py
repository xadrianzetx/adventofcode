from typing import List, Tuple

import numpy as np
from numba import jit
from scipy.ndimage.filters import generic_filter


def read_image(filename: str) -> Tuple[np.array, Tuple[int]]:

    with open(filename) as file:
        data = file.read()
        lut, img = data.split("\n\n")
        mapper = lambda c: 1 if c == "#" else 0
        lut = [mapper(e) for e in lut.rstrip()]
        img = [[mapper(e) for e in line.rstrip()] for line in img.split()]

    # As usual, "infinite" just means "big enough".
    img = np.array(img, dtype=np.uint8)
    img = np.pad(img, ((50, 50), (50, 50)), constant_values=0)

    return img, tuple(lut)


def enhance(img: np.array, lut: List[int], n_iter: int) -> List[int]:
    @jit(nopython=True)
    def _kernel(arr: np.ndarray) -> int:
        idx = 0
        for bit in arr:
            idx = 2 * idx + int(bit)
        return lut[idx]

    lit = []
    for i in range(n_iter):
        # Since first and last bits in image enhancement algorithm
        # are respectively 1 and 0, whole grid will switch to light every
        # other iteration. Edges have to account for this.
        cval = 0 if i % 2 == 0 else 1
        img = generic_filter(img, _kernel, size=3, mode="constant", cval=cval)
        lit.append(img.sum())

    return lit


if __name__ == "__main__":
    img, lut = read_image("d20.txt")

    lit = enhance(img, lut, 50)
    print(lit[1])  # Part 1
    print(lit[-1])  # Part 2
