from typing import List


def read_notes(filename: str) -> List[str]:

    with open(filename) as file:
        data = [line.rstrip().split(" | ") for line in file]
    return data


def count_digits(notes: List[str]) -> int:

    total = 0
    for note in notes:
        output = note[1].split(" ")
        counts = [1 for x in output if len(x) in [2, 4, 3, 7]]
        total += sum(counts)

    return total


if __name__ == "__main__":
    notes = read_notes("d8.txt")

    part1 = count_digits(notes)
    print(part1)
