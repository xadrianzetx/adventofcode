from typing import List, Tuple

import numpy as np
from scipy.ndimage.filters import generic_filter


def read_image(filename: str) -> Tuple[np.array, List[int]]:

    with open(filename) as file:
        data = file.read()
        lut, img = data.split("\n\n")
        mapper = lambda c: 1 if c == "#" else 0
        lut = [mapper(e) for e in lut.rstrip()]
        img = [[mapper(e) for e in line.rstrip()] for line in img.split()]

    # As usual, "infinite" just means "big enough".
    img = np.array(img, dtype=np.uint8)
    img = np.pad(img, ((50, 50), (50, 50)), constant_values=0)

    return img, lut


def enhance(img: np.array, lut: List[int]):
    def _kernel(arr: np.array) -> int:
        bin_idx = "".join([str(int(x)) for x in arr])
        return lut[int(bin_idx, 2)]

    # Since first and last bits in image enhancement algorithm
    # are respectively 1 and 0, whole grid will switch to light every
    # other iteration. Edges have to account for this.
    cval = 1 if img[0, 0] == 1 else 0
    return generic_filter(img, _kernel, size=3, mode="constant", cval=cval)


if __name__ == "__main__":
    img, lut = read_image("d20.txt")
    for i in range(50):
        img = enhance(img, lut)
        if i in [1, 49]:
            # Parts 1 and 2.
            print(img.sum())
