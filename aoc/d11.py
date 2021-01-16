import numpy as np


def read_map(seat_map: list) -> np.array:
    """
    map seats to array
    """

    seats = np.array([list(s) for s in seat_map])
    seats[seats == '#'] = 2
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


def get_closest_seat(seats: np.array) -> int:
    seats = seats[seats != -1]
    closest = seats[0] if len(seats) > 0 else 0
    return closest


def conways_game_of_closest_seats(seats: np.array) -> int:
    """
    thats for sure wrong way of doing this
    """

    nrows, ncols = seats.shape
    rowstate = seats.sum(axis=1)
    colstate = seats.sum(axis=0)
    newseats = seats.copy()

    while True:
        tmp_seats = newseats.copy()
        for i in range(nrows):
            for j in range(ncols):
                occupied = 0

                r = newseats[i, j:][1:]
                r = get_closest_seat(r)
                l = newseats[i, :j][::-1]
                l = get_closest_seat(l)
                u = newseats[:i, j][::-1]
                u = get_closest_seat(u)
                d = newseats[i:, j][1:]
                d = get_closest_seat(d)

                # right down
                rd = newseats[i+1:, j+1:]
                rdh, rdw = rd.shape
                rd = rd[:rdw, :] if rdw < rdh else rd[:, :rdh]
                rd = rd.diagonal()
                rd = get_closest_seat(rd)

                # right up
                ru = newseats[:i, j + 1:]
                ruh, ruw = ru.shape
                ru = ru[ruh - ruw:, :] if ruw < ruh else ru[:, :ruw]
                ru = np.flipud(ru).diagonal()
                ru = get_closest_seat(ru)

                # left down
                ld = newseats[i + 1:, :j]
                ldh, ldw = ld.shape
                ld = ld[:ldh, ldw - ldh:] if ldh <= ldw else ld[:ldw, :]
                ld = np.flipud(ld).diagonal()[::-1]
                ld = get_closest_seat(ld)

                # left up
                lu = newseats[:i, :j]
                luh, luw = lu.shape
                lu = lu[:, luw - luh:] if luh <= luw else lu[luh - luw:, :]
                lu = lu.diagonal()[::-1]
                lu = get_closest_seat(lu)

                occupied += sum([l, r, u, d, rd, ru, ld, lu])

                if newseats[i, j] == 0 and occupied == 0:
                    tmp_seats[i, j] = 1

                if newseats[i, j] == 1 and occupied >= 5:
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
