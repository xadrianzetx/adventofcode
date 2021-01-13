import re
import networkx as nx


def create_bag_graph(rules: str, with_counts: bool = False) -> nx.DiGraph:
    """
    Bagging rules represented as directional graph
    """

    # container rules as graph
    g = nx.DiGraph()

    for rule in rules:
        if with_counts:
            # weak regex game
            container = re.findall('([^ \r\n]+) ([^ \r\n]+) bags?', rule)[0]
            contained = re.findall('([^ \r\n]+)? ([^ \r\n]+) ([^ \r\n]+) bags?', rule)
            bags = [container] + contained

        else:
            bags = re.findall('([^ \r\n]+) ([^ \r\n]+) bags?', rule)

        # add predecessor
        predecessor = bags[0]
        pname = ' '.join(predecessor)
        g.add_node(pname)

        for bag in bags[1:]:
            # add successors
            sname = ' '.join(bag[1:]) if with_counts else ' '.join(bag)
            g.add_node(sname)
            g.add_edge(pname, sname)

            if with_counts:
                # register counts of subsequent bags
                count = 1 if bag[0] == 'contain' else int(bag[0])
                g.nodes[pname][sname] = count

    return g


def get_bag_containers(graph: nx.DiGraph) -> object:
    def get_containers(bag: str) -> list:
        """
        How many bag colors can eventually
        contain at least one shiny gold bag?
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


def get_bag_counter(graph: nx.DiGraph) -> object:
    def get_counter(bag: str) -> int:
        """
        How many individual bags are required
        inside your single shiny gold bag?
        """
        suc = list(graph.successors(bag))
        if len(suc) == 1 and suc[0] == 'no other':
            return 0
        count = 0
        for s in suc:
            n = get_counter(s)
            mul = graph.nodes[bag][s]
            count += (n * mul) + mul
        return count
    return get_counter
