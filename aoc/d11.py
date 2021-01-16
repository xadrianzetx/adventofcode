import numpy as np


def read_map(seat_map: list) -> np.array:
    """
    map seats to array
    """

    seats = np.array([list(s) for s in seat_map])
    seats[seats == 'L'] = 1
    seats[seats == '.'] = 0
    seats = seats.astype('int')
    seats -= 1  # free as 0, floor as -1

    return seats


def conways_game_of_seats(seats: np.array) -> int:
    """
    If a seat is empty (L) and there are no occupied
    seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats
    adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.
    """

    nrows, ncols = seats.shape
    rowstate = seats.sum(axis=1)
    colstate = seats.sum(axis=0)
    newseats = seats.copy()

    while True:
        tmp_seats = newseats.copy()
        for i in range(nrows):
            for j in range(ncols):
                ymin, ymax = max(0, i - 1), min(nrows, i + 2)
                xmin, xmax = max(0, j - 1), min(ncols, j + 2)
                slice = newseats[ymin:ymax, xmin:xmax]

                if newseats[i, j] == 0 and np.max(slice) == 0:
                    #  seat is empty and all adjacent are empty/floor
                    tmp_seats[i, j] = 1

                if newseats[i, j] == 1 and len(slice[slice == 1]) >= 5:
                    # seat is taken and 4 or more adjacent are taken
                    tmp_seats[i, j] = 0

        newseats = tmp_seats
        newrowstate = newseats.sum(axis=1)
        newcolstate = newseats.sum(axis=0)
        rowcheck = np.array_equal(rowstate, newrowstate)
        colcheck = np.array_equal(colstate, newcolstate)

        if not rowcheck and not colcheck:
            # state has changed
            colstate = newcolstate
            rowstate = newrowstate

        else:
            # no change in state
            break

    newseats[newseats == -1] = 0
    taken = np.sum(newseats)

    return taken
