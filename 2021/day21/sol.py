from typing import Generator, Tuple

from numba import jit

MULTIPLIERS = tuple([0, 0, 0, 1, 3, 6, 7, 6, 3, 1])


def deterministic_die() -> int:

    counter = 0
    while True:
        roll = 0
        for _ in range(3):
            roll += (counter % 100) + 1
            counter += 1
        yield roll


def play_game(p1: int, p2: int, die: Generator) -> int:

    roll_count = 0
    s1, s2 = 0, 0
    while True:
        p1 = (p1 + next(die) - 1) % 10 + 1
        s1 += p1
        roll_count += 3
        if s1 >= 1000:
            return s2 * roll_count

        p2 = (p2 + next(die) - 1) % 10 + 1
        s2 += p2
        roll_count += 3
        if s2 >= 1000:
            return s1 * roll_count


@jit(nopython=True)
def play_dirac(p1: int, p2: int, s1: int, s2: int, p1_mv: bool) -> Tuple[int, int]:

    if s1 >= 21:
        return 1, 0

    elif s2 >= 21:
        return 0, 1

    w1, w2 = 0, 0
    for roll in [3, 4, 5, 6, 7, 8, 9]:
        if p1_mv:
            p1d = (p1 + roll - 1) % 10 + 1
            s1d = s1 + p1d
            p2d = p2
            s2d = s2

        else:
            p1d = p1
            s1d = s1
            p2d = (p2 + roll - 1) % 10 + 1
            s2d = s2 + p2d

        a, b = play_dirac(p1d, p2d, s1d, s2d, not p1_mv)
        w1 += a * MULTIPLIERS[roll]
        w2 += b * MULTIPLIERS[roll]

    return w1, w2


if __name__ == "__main__":
    die = deterministic_die()
    part1 = play_game(4, 1, die)
    print(part1)

    part2 = max(play_dirac(4, 1, 0, 0, True))
    print(part2)
