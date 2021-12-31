from functools import lru_cache
from typing import List


def read_population(filename: str) -> List[int]:

    with open(filename) as file:
        data = file.read().rstrip().split(",")

    return [int(d) for d in data]


def simulate_lanternfish(start_days: List[int], n_days: int = 80) -> int:

    total_population = 0
    for start_day in start_days:
        total_population += spawn(start_day + 1, n_days, fforward=False)

    return total_population


@lru_cache()
def spawn(day: int, total_days: int, fforward: bool = True) -> int:

    if fforward:
        # Initial incubation period.
        day += 9

    days_left = total_days - day
    if days_left < 0:
        return 1

    children = 0
    for offset in range(days_left + 1):
        if offset % 7 == 0:
            children += spawn(day + offset, total_days)

    return children + 1


if __name__ == "__main__":
    start_days = read_population("d6.txt")

    part1 = simulate_lanternfish(start_days)
    print(part1)

    part2 = simulate_lanternfish(start_days, n_days=256)
    print(part2)
