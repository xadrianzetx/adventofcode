import re
import networkx as nx


def create_bag_graph(rules: str) -> nx.DiGraph:
    """
    How many bag colors can eventually contain at least one shiny gold bag?
    """

    # container rules as graph
    g = nx.DiGraph()

    for rule in rules:
        bags = re.findall('([^ \r\n]+) ([^ \r\n]+) bags?', rule, re.IGNORECASE)

        # add predecessor
        predecessor = bags[0]
        pname = ' '.join(predecessor)
        g.add_node(pname)

        for bag in bags[1:]:
            # add successors
            sname = ' '.join(bag)
            g.add_node(sname)
            g.add_edge(pname, sname)

    return g


def get_graph_traverse(graph: nx.DiGraph) -> object:
    def get_containers(bag: str) -> list:
        """
        Unique of output gives the answer
        """
        prd = list(graph.predecessors(bag))
        if len(prd) == 0:
            return []
        baglist = []
        for p in prd:
            n = get_containers(p)
            baglist += n
        baglist += prd
        return baglist
    return get_containers
