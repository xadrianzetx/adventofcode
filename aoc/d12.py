import math
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


def rotate(origin, point, angle):
    """
    Rotate a point counterclockwise by a given angle around a given origin.

    The angle should be given in radians.
    """
    ox, oy = origin
    px, py = point

    qx = ox + math.cos(angle) * (px - ox) - math.sin(angle) * (py - oy)
    qy = oy + math.sin(angle) * (px - ox) + math.cos(angle) * (py - oy)
    return qx, qy


def sail_to_waypoint(dirs: list) -> int:
    """
    Almost all of the actions indicate how to
    move a waypoint which is relative to the ship's position
    """

    ship = {'ew': 0, 'ns': 0}
    waypoint = {'ew': 10, 'ns': 1}

    for d in dirs:
        action = d[0]
        value = int(d[1:])

        if action in ['N', 'S']:
            w = waypoint['ns']
            waypoint['ns'] = w + value if action == 'N' else w - value

        if action in ['E', 'W']:
            w = waypoint['ew']
            waypoint['ew'] = w + value if action == 'E' else w - value

        if action in ['L', 'R']:
            angle = math.radians(-value) if action == 'R' else math.radians(value)
            point = tuple(waypoint.values())
            ew, ns = rotate((0, 0), point, angle)
            waypoint['ew'] = np.round(ew).astype('int')
            waypoint['ns'] = np.round(ns).astype('int')

        if action == 'F':
            ship['ns'] = ship['ns'] + (value * waypoint['ns'])
            ship['ew'] = ship['ew'] + (value * waypoint['ew'])

    dist = np.sum([np.abs(coord) for coord in ship.values()])

    return dist
