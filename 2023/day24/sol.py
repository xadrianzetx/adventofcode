from z3 import IntVector, Solver


def parse_line(line: str) -> tuple[int, ...]:
    line = line.replace(" @ ", ", ")
    return tuple(int(c.strip()) for c in line.split(","))


if __name__ == "__main__":
    with open("input", "r") as file:
        hailstones = [parse_line(line) for line in file.readlines()]

    px, py, pz, vx, vy, vz = IntVector("part2", 6)
    timestamps = IntVector("t", len(hailstones))
    s = Solver()

    for t, (hpx, hpy, hpz, hvx, hvy, hvz) in zip(timestamps, hailstones):
        s.add(px + t * vx == hpx + t * hvx)
        s.add(py + t * vy == hpy + t * hvy)
        s.add(pz + t * vz == hpz + t * hvz)

    s.check()
    m = s.model()

    print("Part2:", m[px].as_long() + m[py].as_long() + m[pz].as_long())
