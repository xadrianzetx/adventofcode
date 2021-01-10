import itertools
import numpy as np


def report_pair(arr: np.array) -> int:
    """
    Find the two entries that sum to 2020;
    what do you get if you multiply them together?
    """

    pairs = itertools.combinations(arr, r=2)
    reduced = [p[0] * p[1] for p in pairs if p[0] + p[1] == 2020]
    return reduced[0]
