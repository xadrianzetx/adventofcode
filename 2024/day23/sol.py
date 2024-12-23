import networkx as nx

# Networkx is just too op for graph problems.
G = nx.Graph()

with open("input") as file:
    for line in file.readlines():
        c = line.strip().split("-")
        G.add_edge(c[0], c[1])

part_1 = 0
for clique in nx.enumerate_all_cliques(G):
    if len(clique) == 3:
        part_1 += int(any(elem.startswith("t") for elem in clique))

part_2 = ",".join(sorted(clique))
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
