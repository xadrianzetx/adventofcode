import numpy as np
from typing import Union


def play_crab_cups(cups: list, n_iter: int,
                   part_one: bool = True) -> Union[str, int]:
    """
    Plays crab cups

    My original solution was too slow for part 2
    so switching to r/adventofcode genius sol.
    """

    max_cups = max(cups)
    src = cups[0]
    links = {}

    for a, b in zip(cups, cups[1:] + cups[:1]):
        # build initial map
        links[a] = b

    for _ in range(n_iter):
        a = links[src]
        b = links[a]
        c = links[b]

        # link the gap after
        # removing 3 cups
        links[src] = links[c]
        dest = src - 1

        while True:
            if dest < 1:
                dest = max_cups
            if dest in [a, b, c]:
                dest -= 1
            else:
                break

        links[c] = links[dest]  # link end of seq
        links[dest] = a  # link start of seq
        src = links[src]

    if part_one:
        k = 1
        res = ''
        while True:
            val = links[k]
            if val == 1:
                break
            res += str(val)
            k = val

    else:
        res = links[1] * links[links[1]]

    return res
