import numpy as np


class Tile:

    def __init__(self, id: int, tile: np.array):
        self.id = id
        self.tile = tile
        self.linedup = 0
        self.edges = []

    def create_edges(self) -> 'Tile':
        """
        """

        edges = [
            self.tile[0, :],
            self.tile[-1, :],
            self.tile[:, 0],
            self.tile[:, -1]
        ]

        for edge in edges:
            # covers all possible
            # arrangements of edges
            self.edges.append(edge)
            self.edges.append(edge[::-1])

        return self

    def check_lineup(self, tile: 'Tile') -> 'Tile':
        """Check lineup with another tile"""

        for edgea in tile.edges:
            for edgeb in self.edges:
                if np.array_equal(edgea, edgeb):
                    self.linedup += 1

        return self


def read_tiles(file: str) -> list:
    """
    """

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
    """
    """

    tiles = [t.create_edges() for t in tiles]
    for tile in tiles:
        for cand in tiles:
            if tile.id != cand.id:
                tile.check_lineup(cand)

    corners = [t.id for t in tiles if t.linedup == 4]
    return np.product(corners, dtype='int64')
