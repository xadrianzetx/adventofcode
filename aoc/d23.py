import numpy as np


def play_crab_cups(cups: np.array, n_iter: int) -> str:
    """Plays crab cups"""

    n_cups = len(cups)
    max_cups = max(cups)
    min_cups = min(cups)
    current = 0

    for i in range(n_iter):
        offsets = np.array([1, 2, 3]) + current
        pick_idx = (offsets) % n_cups
        rolled = np.sum(offsets != pick_idx)
        dest = cups[current] - 1

        while True:
            if dest < min_cups:
                dest = max_cups
            if dest in cups[pick_idx]:
                dest -= 1
            if dest in cups and dest not in cups[pick_idx]:
                break

        picks = cups[pick_idx]
        cups = cups[~np.in1d(cups, picks)]
        dest_idx = np.argwhere(cups == dest)[0][0]
        cups = np.insert(cups, dest_idx + 1, picks)

        if dest_idx < current:
            # need to roll back left
            # in this case
            # 3 - n rolled back
            rol = 3 - rolled
            rback = cups[:rol]
            cups = cups[rol:]
            cups = np.append(cups, rback)

        current += 1

        if current >= n_cups:
            current = 0

    onepos = np.argwhere(cups == 1)[0][0]
    print(cups)
    wrap = cups[:onepos]
    cups = cups[onepos:]
    cups = np.append(cups, wrap)
    res = ''.join(cups[1:].astype('str'))

    return res
