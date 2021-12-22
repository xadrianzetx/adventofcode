from dataclasses import dataclass
from typing import List, Optional

Steps = List[List[int]]


@dataclass
class Cube:
    x: List[int]
    y: List[int]
    z: List[int]
    val: int

    def intersect(self, other: "Cube") -> Optional["Cube"]:

        x = [max(self.x[0], other.x[0]), min(self.x[1], other.x[1])]
        y = [max(self.y[0], other.y[0]), min(self.y[1], other.y[1])]
        z = [max(self.z[0], other.z[0]), min(self.z[1], other.z[1])]
        if x[1] >= x[0] and y[1] >= y[0] and z[1] >= z[0]:
            return Cube(x, y, z, 0 - self.val)
        return None

    def on(self) -> bool:

        return self.val == 1

    def volume(self) -> int:

        return (
            (self.x[1] - self.x[0] + 1)
            * (self.y[1] - self.y[0] + 1)
            * (self.z[1] - self.z[0] + 1)
            * self.val
        )


def read_reboot_steps(filename: str, limit: bool = True) -> Steps:

    steps = []
    with open(filename) as file:
        for line in file:
            cmd, ranges = line.rstrip().split()
            cmd = 1 if cmd == "on" else -1
            step = [cmd]
            include = True

            for rng in ranges.split(","):
                bounds = rng[2:].split("..")
                bounds = [int(b) for b in bounds]
                if (bounds[0] < -50 or bounds[1] > 50) and limit:
                    include = False
                step.append(bounds)

            if include:
                steps.append(step)

    return steps


def reboot(steps: Steps) -> int:

    cubes: List[Cube] = []
    for step in steps:
        cube = Cube(step[1], step[2], step[3], step[0])
        intersections = []

        for existing in cubes:
            intersect = existing.intersect(cube)
            if intersect:
                intersections.append(intersect)

        cubes.extend(intersections)
        if cube.on():
            cubes.append(cube)

    return sum([c.volume() for c in cubes])


if __name__ == "__main__":
    steps = read_reboot_steps("d22.txt")
    part1 = reboot(steps)
    print(part1)

    steps = read_reboot_steps("d22.txt", limit=False)
    part2 = reboot(steps)
    print(part2)
