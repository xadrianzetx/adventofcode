from typing import List


Directions = List[List[str]]


def read_directions(filename: str) -> Directions:

    with open(filename) as file:
        data = [line.rsplit() for line in file.readlines()]
    return data


def steer(directions: Directions) -> None:

    hpos = 0
    depth = 0

    for direction in directions:
        if direction[0] == "forward":
            hpos += int(direction[1])
        elif direction[0] == "up":
            depth -= int(direction[1])
        else:
            depth += int(direction[1])

    return depth * hpos


if __name__ == "__main__":
    directions = read_directions("d2.txt")
    print(steer(directions))
