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


def flip_tiles(tiles: list) -> int:
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

    return n_black
