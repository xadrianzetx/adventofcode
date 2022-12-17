from typing import Dict, List


class Room:
    def __init__(self, name: str, next_: List[str], flow: int) -> None:
        self.name = name
        self.next_ = next_
        self.flow = flow
        self.opened = False
        self.visited = False


CAVE: Dict[str, Room] = {}
ATH = []


def visit_room(
    left: int,
    room: str,
    released: int,
    visited: List[str],
    opened: List[str],
    patience: int,
    p2: bool,
) -> int:
    if left == 0:
        if p2:
            ATH.append((released, opened))
        return released

    croom = CAVE[room]
    exploitation = 0
    exploration = 0
    updated = False
    if croom.flow > 0 and room not in opened:
        r = released + (croom.flow * (left - 1))
        s = max(CAVE[o].flow for o in CAVE if o not in opened)
        exploitation = visit_room(
            left - 1, croom.name, r, [], opened + [croom.name], patience, p2
        )
        updated = True

    if patience > 0:
        updated = True
        for neighbor in croom.next_:
            if neighbor not in visited:
                b = visit_room(
                    left - 1,
                    neighbor,
                    released,
                    visited + [croom.name],
                    opened,
                    patience - 1,
                    p2,
                )
                exploration = max(exploration, b)

    if not updated and p2:
        ATH.append((released, opened))
    return max(released, max(exploitation, exploration))


def part1():
    x = visit_room(30, "AA", 0, [], [], patience=17, p2=False)
    print(f"Part1: {x}")


def part2():
    visit_room(26, "AA", 0, [], [], patience=17, p2=True)
    maxp = 0
    seen = set()
    for entry in sorted(ATH, key=lambda x: x[0], reverse=True)[:50]:
        if "".join(entry[1]) not in seen:
            seen.add("".join(entry[1]))
            y = visit_room(26, "AA", 0, [], entry[1], patience=17, p2=True)
            maxp = max(entry[0] + y, maxp)
    print(f"Part2: {maxp}")


def parse_line(line: str) -> Room:
    l, r = line.split(";")
    name = l.split()[1]
    flow = int(l.split("=")[-1])
    next_ = [n.replace(",", "") for n in r.split()[4:]]
    return Room(name, next_, flow)


if __name__ == "__main__":
    with open("input") as file:
        for line in file:
            room = parse_line(line)
            CAVE[room.name] = room

    # The hackery begins! Runs in about 5s with pypy.
    # This could use some Rust and DP.
    part1()
    part2()
