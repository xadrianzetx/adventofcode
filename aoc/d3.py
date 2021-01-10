import numpy as np


def traverse_forest(data: list, mv_right: int = 3, mv_down: int = 1) -> int:
    """
    Starting at the top-left corner of your map and following
    a slope of right 3 and down 1, how many trees would you encounter?
    """

    idx = 0
    vertical = mv_down - 1
    max_idx = len(list(data[0]))  # map width
    trees = 0

    for datum in data:
        vertical += 1
        if vertical != mv_down:
            continue
        vertical = 0
        chrs = list(datum)
        if chrs[idx] == '#':
            trees += 1
        if idx + mv_right >= max_idx:
            idx = idx + mv_right - max_idx
        else:
            idx += mv_right
    return trees


def super_traverse_forest(data: list) -> int:
    """
    Determine the number of trees you would encounter if,
    for each of the following slopes, you start at the
    top-left corner and traverse the map all the way to the bottom:

    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
    """

    policies = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ]
    trees = [traverse_forest(data, *policy) for policy in policies]
    prod = np.multiply.reduce(trees, dtype='int64')
    return prod
