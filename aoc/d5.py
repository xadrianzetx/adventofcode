import numpy as np


def find_seat(code: str) -> int:
    rows = np.arange(1, 129)
    cols = np.arange(1, 9)
    for chr in code:
        if chr == 'F':
            # keep lower half of rows
            rows = rows[rows <= np.mean(rows)]
        elif chr == 'B':
            # keep upper half of rows
            rows = rows[rows > np.mean(rows)]
        elif chr == 'L':
            # keep lower half of cols
            cols = cols[cols <= np.mean(cols)]
        else:
            # keep upper half of cols
            cols = cols[cols > np.mean(cols)]
    seat_id = ((rows[0] - 1) * 8) + (cols[0] - 1)
    return seat_id
