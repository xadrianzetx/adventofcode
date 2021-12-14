from collections import defaultdict
from typing import Dict


def read_template(filename: str) -> Dict[str, str]:

    data = {}
    with open(filename) as file:
        for line in file:
            key, val = line.rstrip().split(" -> ")
            data[key] = val

    return data


def grow_polymer(sequence: str, template: Dict[str, str], iter: int) -> int:

    pairs, counts = defaultdict(int), defaultdict(int)
    for left, right in zip(sequence[:-1], sequence[1:]):
        pairs[f"{left}{right}"] += 1
        counts[left] += 1
    counts[right] += 1

    for _ in range(iter):
        pcopy = pairs.copy()
        for pair, insert in template.items():
            count = pcopy[pair]
            pairs[pair] -= count
            counts[insert] += count
            left, right = list(pair)
            pairs[f"{left}{insert}"] += count
            pairs[f"{insert}{right}"] += count

    return max(counts.values()) - min(counts.values())


if __name__ == "__main__":
    template = read_template("d14.txt")

    part1 = grow_polymer("PBVHVOCOCFFNBCNCCBHK", template, 10)
    print(part1)

    part2 = grow_polymer("PBVHVOCOCFFNBCNCCBHK", template, 40)
    print(part2)
