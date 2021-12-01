import numpy as np


def read_sweep_report(filename: str) -> np.array:

    with open(filename) as file:
        report = file.readlines()
    return np.array(report, dtype=np.int32)


def count_increases(report: np.array) -> int:

    diffs = report[1:] - report[:-1]
    return len(diffs[diffs > 0])


def three_measurements(report: np.array) -> np.array:

    return report[2:] + report[1:-1] + report[:-2]


if __name__ == "__main__":

    report = read_sweep_report("d1.txt")
    part1 = count_increases(report)
    print(part1)

    part2 = count_increases(three_measurements(report))
    print(part2)
