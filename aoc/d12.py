import numpy as np


def sail(dirs: list) -> int:
    """
    Figure out where the navigation instructions lead.
    What is the Manhattan distance between that location
    and the ship's starting position?
    """

    ns = 0
    ew = 0

    # clockwise rotation in degrees
    # 0 means east
    # 90 means south
    # 180 means west
    # 270 means north
    rot = 0

    for d in dirs:
        action = d[0]
        value = int(d[1:])

        if action == 'F':
            # translate forward into direction
            # based on current rotation
            action = {
                '0': 'E',
                '90': 'S',
                '180': 'W',
                '270': 'N'
            }[str(rot)]

        if action in ['N', 'S']:
            ns = ns + value if action == 'N' else ns - value

        if action in ['E', 'W']:
            ew = ew + value if action == 'E' else ew - value

        if action == 'R':
            rot += value
            rot = rot % 360

        if action == 'L':
            rot += (360 - value)
            rot = rot % 360

    dist = np.sum([np.abs(ns), np.abs(ew)])

    return dist
