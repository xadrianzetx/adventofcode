from typing import Dict

import numpy as np


def read_template(filename: str) -> Dict[str, str]:

    data = {}
    with open(filename) as file:
        for line in file:
            key, val = line.rstrip().split(" -> ")
            data[key] = val

    return data


def grow_polymer(sequence: str, template: Dict[str, str], iter: int) -> int:

    for _ in range(iter):
        newseq = sequence[0]
        for l, r in zip(sequence[:-1], sequence[1:]):
            inserted = template[f"{l}{r}"]
            newseq += f"{inserted}{r}"
        sequence = newseq

    _, counts = np.unique(list(newseq), return_counts=True)
    return max(counts) - min(counts)


if __name__ == "__main__":
    template = read_template("d14.txt")

    part1 = grow_polymer("PBVHVOCOCFFNBCNCCBHK", template, 10)
    print(part1)
