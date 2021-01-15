import numpy as np
from itertools import combinations


def find_xmas_entrypoint(arr: list, preamble: int = 25) -> int:
    """
    The first step of attacking the weakness in the XMAS data
    is to find the first number in the list (after the preamble)
    which is not the sum of two of the 25 numbers before it.
    What is the first number that does not have this property?
    """

    idxa = 0
    idxb = preamble
    invalid = -1

    for num in arr[preamble:]:
        lead = arr[idxa:idxb]
        sums = [sum(c) for c in combinations(lead, r=2) if c[0] != c[1]]

        if num not in sums:
            invalid = num
            break

        idxa += 1
        idxb += 1

    return invalid


def exploit_xmas_entrypoint(arr: np.array, ep: int) -> np.array:
    """
    To find the encryption weakness, add together the smallest
    and largest number in this contiguous range;
    """

    arr = np.array(arr)
    itemidx = np.where(arr == ep)[0][0]
    arr = arr[:itemidx]

    for kernel in range(3, len(arr) - 1):
        conv = np.convolve(arr, np.ones(kernel, dtype='int'), 'valid')

        try:
            # found entrypoint in convolved arr
            hit = np.where(conv == ep)[0][0]
            break

        except IndexError:
            # no hits for this kernel size
            continue

    cont = arr[hit:hit + kernel - 1]

    return min(cont) + max(cont)
