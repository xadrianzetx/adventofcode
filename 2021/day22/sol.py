from collections import defaultdict
from typing import Dict, List

Steps = List[List[int]]


def read_reboot_steps(filename: str) -> Steps:

    steps = []
    with open(filename) as file:
        for line in file:
            valid = True
            cmd, ranges = line.rstrip().split()
            cmd = 1 if cmd == "on" else 0
            step = [cmd]
            for rng in ranges.split(","):
                bounds = rng[2:].split("..")
                bounds = [int(b) for b in bounds]
                if max(bounds) > 50 or min(bounds) < -50:
                    valid = False
                step.append(bounds)
            if valid:
                steps.append(step)

    return steps


def _reboot_cube(reactor, x, y, z, l, h, val):

    if l <= x <= h and l <= y <= h and l <= z <= h:
        reactor[(x, y, z)] = val


def reboot(steps: Steps) -> int:

    reactor = defaultdict(int)
    for step in steps:
        for x in range(step[1][0], step[1][1] + 1):
            for y in range(step[2][0], step[2][1] + 1):
                for z in range(step[3][0], step[3][1] + 1):
                    # print(x, y, z)
                    _reboot_cube(reactor, x, y, z, -50, 50, step[0])

    return sum(reactor.values())


if __name__ == "__main__":
    step = read_reboot_steps("d22.txt")
    # print(step)

    part1 = reboot(step)
    print(part1)
