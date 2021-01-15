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
