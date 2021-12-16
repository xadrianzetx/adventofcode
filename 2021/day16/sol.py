from dataclasses import dataclass
from typing import List, Optional, Tuple

import numpy as np


def decode_message(filename: str) -> str:

    with open(filename) as file:
        msg = file.read().strip()
        msg = bin(int(msg, 16))

    return msg[2:]


@dataclass
class Packet:
    ver: int
    id: int
    body: Optional[int]
    children: Optional[List["Packet"]]

    def sum_versions(self) -> int:

        if self.body is not None:
            return int(self.ver, 2)

        ver_sum = 0
        for child in self.children:
            ver_sum += child.sum_versions()

        return ver_sum + int(self.ver, 2)

    def calculate(self) -> int:

        if self.id == 4:
            return self.body

        if self.id == 0:
            return sum([c.calculate() for c in self.children])

        if self.id == 1:
            return np.prod([c.calculate() for c in self.children])

        if self.id == 2:
            return min([c.calculate() for c in self.children])

        if self.id == 3:
            return max([c.calculate() for c in self.children])

        if self.id == 5:
            foo = self.children[0].calculate() > self.children[1].calculate()
            return 1 if foo else 0

        if self.id == 6:
            foo = self.children[0].calculate() < self.children[1].calculate()
            return 1 if foo else 0

        if self.id == 7:
            foo = self.children[0].calculate() == self.children[1].calculate()
            return 1 if foo else 0


def parse_packets(packet: str) -> Tuple[Packet, str]:

    version = packet[:3]
    type = int(packet[3:6], 2)
    packet = packet[6:]

    if type == 4:
        lit = ""
        while True:
            grp = packet[:5]
            packet = packet[5:]
            prefix = grp[0]
            lit += grp[1:]
            if int(prefix, 2) == 0:
                break
        body = int(lit, 2)
        p = Packet(version, type, body, None)

    else:
        ltid = packet[0]
        if int(ltid) == 0:
            # 15 bit total length
            tl = int(packet[1:16], 2)
            # print(tl)
            packet = packet[16:]
            children = []
            read = 0
            while read < tl:
                pl = len(packet)
                child, packet = parse_packets(packet)
                children.append(child)
                read += pl - len(packet)
            p = Packet(version, type, None, children)

        else:
            # 11 bit num of packages
            bl = int(packet[1:12], 2)
            packet = packet[12:]
            children = []
            for r in range(bl):
                child, packet = parse_packets(packet)
                children.append(child)
            p = Packet(version, type, None, children)

    return p, packet


if __name__ == "__main__":
    message = decode_message("d16.txt")

    packet, check = parse_packets(message)
    assert int(check, 2) == 0

    part1 = packet.sum_versions()
    print(part1)

    part2 = packet.calculate()
    print(part2)
