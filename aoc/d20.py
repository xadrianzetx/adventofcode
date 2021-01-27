import numpy as np
from scipy.signal import convolve2d


class Tile:

    def __init__(self, id: int, tile: np.array):
        self.id = id
        self.tile = tile
        self.linedup = 0
        self.edges = [
            tile[0, :],
            tile[-1, :],
            tile[:, 0],
            tile[:, -1],
            tile[0, :][::-1],
            tile[-1, :][::-1],
            tile[:, 0][::-1],
            tile[:, -1][::-1]
        ]

    def check_lineup(self, tile: 'Tile') -> 'Tile':
        """Check lineup with another tile"""

        for edgea in tile.edges:
            for edgeb in self.edges:
                if np.array_equal(edgea, edgeb):
                    self.linedup += 1

        return self


def read_tiles(file: str) -> list:
    """Parses tile to Tile object"""

    with open(file, 'r') as f:
        tiles = []
        tiledata = []

        for line in f:
            line = line.strip()

            if line == '':
                tid = tiledata[0].split()[1][:-1]
                tcontent = np.array([list(r) for r in tiledata[1:]])
                tcontent[tcontent == '.'] = 0
                tcontent[tcontent == '#'] = 1
                tile = Tile(id=int(tid), tile=tcontent.astype('int'))
                tiles.append(tile)
                tiledata = []

            else:
                tiledata.append(line)

    return tiles


def check_lineup(tiles: list) -> int:
    """Finds product of corner ids"""

    for tile in tiles:
        for cand in tiles:
            if tile.id != cand.id:
                tile.check_lineup(cand)

    corners = [t.id for t in tiles if t.linedup // 2 == 2]
    print(corners)
    return np.product(corners, dtype='int64')


def nessie_detection(tiles: list) -> int:
    """
    Shortcut to find number of nessies in picture

    Instead of tedious image rebuild process, this
    solution looks for head of nessie in separate tiles,
    hoping that one head cannot be split between two tiles.
    """

    tot_heads = []
    tot_pix = 0
    kernel = np.array(
        # nessies head!
        [[0., 0., 0., 1., 0.],
         [0., 0., 1., 1., 1.],
         [0., 1., 0., 0., 0.]]
    ).astype('int')
    kernel = np.flipud(np.fliplr(kernel))

    for tile in tiles:
        tile_heads = []
        # crop excess pixels from frame
        t = tile.tile[1:-1, 1:-1]
        tot_pix += np.sum(t)

        for rot in range(4):
            # convolve kernel with tile
            # in every possible position
            rtile = np.rot90(t, rot)
            sidea = convolve2d(rtile, kernel, mode='valid')
            sideb = convolve2d(np.flipud(rtile), kernel, mode='valid')
            tile_heads.append(len(sidea[sidea == 5]))
            tile_heads.append(len(sideb[sideb == 5]))

        # assuming that if a tile has nessies
        # head at all, correct tile position
        # is going to have max of it
        tot_heads.append(max(tile_heads))

    nessies = sum(tot_heads)
    roughness = tot_pix - (nessies * 15)

    return roughness
