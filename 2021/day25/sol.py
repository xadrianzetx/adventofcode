from typing import Dict, Tuple


def read_map(filename: str) -> Dict[complex, str]:

    cucumbers = {}
    maxx, maxy = 0, 0
    with open(filename) as file:
        pos = 0j
        for line in file:
            for char in list(line.rstrip()):
                if char in ["v", ">"]:
                    cucumbers[pos] = char
                pos += 1
            maxx = int(pos.real) - 1
            pos = complex(0, pos.imag + 1)
        maxy = int(pos.imag) - 1

    return cucumbers, maxx, maxy


def move(cucumbers: Dict[complex, str], bounds: Tuple[int]) -> int:

    counter = 0
    while True:
        moved = 0
        newc = cucumbers.copy()
        for pos, c in cucumbers.items():
            newpos = pos + 1
            if newpos.real > bounds[0]:
                newpos = complex(0, pos.imag)
            if c == ">" and cucumbers.get(newpos) is None:
                newc[newpos] = ">"
                newc.pop(pos)
                moved += 1

        cucumbers = newc.copy()
        for pos, c in cucumbers.items():
            newpos = pos + 1j
            if newpos.imag > bounds[1]:
                newpos = complex(pos.real, 0)
            if c == "v" and cucumbers.get(newpos) is None:
                newc[newpos] = "v"
                newc.pop(pos)
                moved += 1

        cucumbers = newc.copy()
        counter += 1
        if moved == 0:
            break

    return counter


if __name__ == "__main__":
    cucumbers, *bounds = read_map("d25.txt")
    print(move(cucumbers, bounds))
