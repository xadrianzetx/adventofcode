from typing import List

import networkx as nx


def build_graph(filename: str) -> nx.Graph:

    G = nx.Graph()
    with open(filename) as file:
        for line in file:
            nodea, nodeb = line.rstrip().split("-")
            if nodea not in G:
                G.add_node(nodea)
            if nodeb not in G:
                G.add_node(nodeb)
            G.add_edge(nodea, nodeb)

    return G


def traverse(caves: nx.Graph, repeat: bool) -> int:

    paths = visit_cave(caves, "start", [], repeat)
    return paths


def visit_cave(caves: nx.Graph, cave: str, visited: List[str], repeat: bool) -> int:

    if cave == "end":
        return 1

    if cave in visited:
        if repeat and cave != "start":
            repeat = False
        else:
            return 0

    vcp = visited.copy()
    if cave.islower():
        vcp.append(cave)

    paths = 0
    for child in caves.neighbors(cave):
        paths += visit_cave(caves, child, vcp, repeat)

    return paths


if __name__ == "__main__":
    caves = build_graph("d12.txt")

    part1 = traverse(caves, repeat=False)
    print(part1)

    part2 = traverse(caves, repeat=True)
    print(part2)
