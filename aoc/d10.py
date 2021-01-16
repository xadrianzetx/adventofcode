import numpy as np


def chain_adapters(adapters: np.array) -> int:
    """
    Find a chain that uses all of your adapters to connect
    the charging outlet to your device's built-in adapter and
    count the joltage differences between the charging outlet,
    the adapters, and your device. What is the number of 1-jolt
    differences multiplied by the number of 3-jolt differences?
    """

    adapters = np.sort(adapters)
    jdiffs = adapters[1:] - adapters[:-1]
    jdiffs = np.insert(jdiffs, 0, adapters[0])  # diff between wall and first adapter
    jdiffs = np.append(jdiffs, 3)  # const built-in adapter
    _, jcounts = np.unique(jdiffs, return_counts=True)

    return np.product(jcounts), jdiffs


def combine_adapters(arr: np.array) -> int:
    """
    max length of consecutive nums is 5

    lengths of 3 have 2 combos
    lengths of 4 have 2^2=4 combos
    lengths of 5 have 2^3-1=7 combos
    """

    data = np.sort(arr)
    data = np.insert(data, 0, 0)
    data = np.append(data, max(data) + 3)
    seq = np.split(data, np.where(np.diff(data) != 1)[0] + 1)
    seq_len = np.array([len(s) for s in seq])

    a = 7 ** len(seq_len[seq_len == 5])
    b = 4 ** (len(seq_len[seq_len == 4]))
    c = 2 * len(seq_len[seq_len == 3])
    combos = a * b * c

    return combos
