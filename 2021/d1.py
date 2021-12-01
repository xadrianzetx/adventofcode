import numpy as np


def read_sweep_report() -> np.array:

    with open("2021/d1.txt") as file:
        report = file.readlines()
    return np.array(report, dtype=np.int32)


def count_increases(report: np.array) -> int:

    diffs = report[1:] - report[:-1]
    return len(diffs[diffs > 0])


if __name__ == "__main__":

    report = read_sweep_report()
    increases = count_increases(report)
    print(increases)
