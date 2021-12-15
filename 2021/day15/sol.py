import networkx as nx
import numpy as np


def read_map(filename: str) -> np.array:

    with open(filename) as file:
        data = [list(line.rstrip()) for line in file]

    return np.array(data, dtype=np.uint8)


def find_path(risk_map: np.array) -> int:

    graph = nx.grid_2d_graph(*risk_map.shape, create_using=nx.DiGraph)
    for src, tar in graph.edges():
        graph[src][tar]["risk"] = risk_map[tar]

    exit_node = tuple(s - 1 for s in risk_map.shape)
    min_risk = nx.shortest_path_length(graph, (0, 0), exit_node, weight="risk")

    return min_risk


def expand_map(cmap: np.array) -> np.array:

    rows = []
    for row in range(5):
        tiles = []
        for col in range(5):
            tile = np.copy(cmap)
            tile += row + col
            tile[tile > 9] = tile[tile > 9] - 9
            tiles.append(tile)

        row = np.concatenate(tiles, axis=-1)
        rows.append(row)

    expanded = np.concatenate(rows, axis=0)
    return expanded


if __name__ == "__main__":
    cavemap = read_map("d15.txt")

    part1 = find_path(cavemap)
    print(part1)

    part2 = find_path(expand_map(cavemap))
    print(part2)
