import numpy as np


COMPLEX = {
    'nw': -0.5 + 1j,
    'w': -1 + 0j,
    'sw': -0.5 - 1j,
    'se': 0.5 - 1j,
    'e': 1 + 0j,
    'ne': 0.5 + 1j
}


def parse_line(line: str) -> list:
    """Parses tile directions"""

    maxidx = len(line)
    idx = 0
    dirs = []

    while True:
        if idx >= maxidx:
            break

        char = line[idx]

        if char in ['s', 'n']:
            # need to parse one more char
            char += line[idx + 1]
            idx += 2

        else:
            idx += 1

        dirs.append(char)

    return dirs


def flip_tiles(tiles: list) -> tuple:
    """Navigates to tile and flips it"""

    visited = {}

    for tile in tiles:
        moves = parse_line(tile)
        ref = 0 + 0j

        for mov in moves:
            ref += COMPLEX[mov]

        if ref not in visited.keys():
            # initial flip to black
            visited[ref] = 0

        else:
            # flip already visited tile
            visited[ref] += 1

    black = [t for t in visited.values() if t % 2 == 0]
    n_black = len(black)

    return n_black, visited


def expand_grid(tiles: dict) -> dict:
    """
    Set tiles not visited during initial
    flipping to default side (white).

    100x100 grid is large enough for part 2
    """

    for i in np.arange(-100, 100.5, 0.5):
        for j in range(-100, 101):
            coord = complex(i, j)

            if coord not in tiles.keys():
                tiles[coord] = 1

    return tiles


def game_of_tiles(tiles: dict) -> int:
    """Flips tile grid according to rules"""

    for i in range(100):
        tmp_tiles = tiles.copy()

        for tilepos, tileval in tiles.items():
            # get adjacents
            adjacents = [tilepos + a for a in COMPLEX.values()]
            blackup = []

            for adj in adjacents:
                try:
                    adj_tile = tiles[adj]
                    if adj_tile % 2 == 0:
                        blackup.append(0)

                except KeyError:
                    # out of bounds tile
                    # is white by default
                    pass

            if tileval % 2 == 0 and (len(blackup) == 0 or len(blackup) > 2):
                tmp_tiles[tilepos] = 1

            elif tileval % 2 != 0 and len(blackup) == 2:
                tmp_tiles[tilepos] = 0

            else:
                #  no change
                tmp_tiles[tilepos] = tileval

        tiles = tmp_tiles

    black = [t for t in tiles.values() if t % 2 == 0]
    n_black = len(black)

    return n_black
