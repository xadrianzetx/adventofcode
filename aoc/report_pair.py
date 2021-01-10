import numpy as np


def report_pair(arr: np.array) -> int:
    """
    Find the two entries that sum to 2020;
    what do you get if you multiply them together?
    """
    # shit way to do this lol
    for i in arr:
        for j in arr:
            for k in arr:
                if i + j +k == 2020:
                    return i, j, k, i * j * k
