import networkx as nx
import numpy as np

G = nx.Graph()


if __name__ == "__main__":
    # Going back to py as networkx is just too op.
    with open("input") as file:
        for line in file.readlines():
            src, dests = line.split(":")
            for dest in dests.split():
                G.add_edge(src, dest)

    for src, dest in nx.minimum_edge_cut(G):
        G.remove_edge(src, dest)

    print("Part 1:", np.prod([len(c) for c in nx.connected_components(G)]))
