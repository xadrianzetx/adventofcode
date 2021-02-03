import numpy as np
from functools import reduce


def pick_bus(depart: int, schedule: str) -> int:
    """
    What is the ID of the earliest bus you can
    take to the airport multiplied by the number of
    minutes you'll need to wait for that bus?
    """

    running = [int(b) for b in schedule.split(',') if b != 'x']
    delays = [b - depart % b for b in running]
    return running[np.argmin(delays)] * np.min(delays)


def chinese_remainder(n: np.array, a: np.array) -> int:
    sum = 0
    prod = reduce(lambda a, b: a * b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod


def mul_inv(a: int, b: int) -> int:
    b0 = b
    x0, x1 = 0, 1
    if b == 1:
        return 1
    while a > 1:
        q = a // b
        a, b = b, a % b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0:
        x1 += b0
    return x1


def earliest_timestamp(schedule: str) -> int:
    """
    https://rosettacode.org/wiki/Chinese_remainder_theorem
    """

    schedule = schedule.split(',')
    offsets = np.arange(len(schedule))
    offsets = np.array([o for o, b in zip(offsets, schedule) if b != 'x']).astype('int64')
    n = np.array([int(b) for b in schedule if b != 'x']).astype('int64')
    a = n - offsets
    res = chinese_remainder(n, a)
    return res
