from dataclasses import dataclass
from typing import Generator, List, Optional, Tuple, Union

import numpy as np


@dataclass
class Packet:
    version: int
    type: int
    body: Optional[int]
    children: Optional[List["Packet"]]

    def sum_versions(self) -> int:

        if self.body is not None:
            return self.version

        ver_sum = 0
        for child in self.children:
            ver_sum += child.sum_versions()

        return ver_sum + self.version

    def calculate(self) -> int:

        if self.type == 4:
            return self.body

        if self.type == 0:
            return sum([c.calculate() for c in self.children])

        if self.type == 1:
            return np.prod([c.calculate() for c in self.children])

        if self.type == 2:
            return min([c.calculate() for c in self.children])

        if self.type == 3:
            return max([c.calculate() for c in self.children])

        if self.type == 5:
            foo = self.children[0].calculate() > self.children[1].calculate()
            return 1 if foo else 0

        if self.type == 6:
            foo = self.children[0].calculate() < self.children[1].calculate()
            return 1 if foo else 0

        if self.type == 7:
            foo = self.children[0].calculate() == self.children[1].calculate()
            return 1 if foo else 0


class Stream:
    def __init__(self, generator: Generator) -> None:

        self._generator = generator
        self.consumed = 0

    @classmethod
    def from_transmission(cls, transmission: str) -> "Stream":
        def _generator(transmission: str) -> Generator:

            for char in transmission:
                yield char

        return cls(_generator(transmission))

    def take(self, n_bits: int, decimal: bool = True) -> Union[int, str]:

        msg = "".join([next(self._generator) for _ in range(n_bits)])
        self.consumed += n_bits

        return int(msg, 2) if decimal else msg

    def checksum(self) -> int:

        return sum([int(bit, 2) for bit in self._generator])


def decode_transmission(filename: str) -> str:

    with open(filename) as file:
        msg = file.read().strip()
        msg = bin(int(msg, 16))

    return msg[2:]


def parse_packets(stream: Stream) -> Tuple[Packet, int]:

    version = stream.take(3)
    type = stream.take(3)

    if type == 4:
        lit = ""
        while True:
            group = stream.take(5, decimal=False)
            lit += group[1:]
            if int(group[0], 2) == 0:
                break

        packet = Packet(version, type, int(lit, 2), None)

    else:
        len_type_id = stream.take(1)
        children = []
        if len_type_id == 0:
            # 15 bits are a number that represents the total
            # length in bits of the sub-packets contained by this packet.
            to_read = stream.take(15)
            checkpoint = stream.consumed
            while True:
                child = parse_packets(stream)
                children.append(child)
                if stream.consumed - checkpoint == to_read:
                    break

        else:
            # 11 bits are a number that represents the number of
            # sub-packets immediately contained by this packet.
            num_subpackets = stream.take(11)
            for _ in range(num_subpackets):
                child = parse_packets(stream)
                children.append(child)

        packet = Packet(version, type, None, children)

    return packet


if __name__ == "__main__":
    stream = Stream.from_transmission(decode_transmission("d16.txt"))
    packet = parse_packets(stream)
    # Ensure only garbage bits are left in message
    assert stream.checksum() == 0

    part1 = packet.sum_versions()
    print(part1)

    part2 = packet.calculate()
    print(part2)
