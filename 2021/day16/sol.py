from dataclasses import dataclass
from enum import Enum
from typing import Callable, Generator, List, Optional, Union

import numpy as np


class PacketType(Enum):

    OP_SUM = 0
    OP_PROD = 1
    OP_MIN = 2
    OP_MAX = 3
    LITERAL = 4
    OP_GT = 5
    OP_LT = 6
    OP_EQ = 7

    @classmethod
    def associated_op(cls, type: "PacketType") -> Callable:

        return {
            cls.OP_SUM: sum,
            cls.OP_PROD: np.prod,
            cls.OP_MIN: min,
            cls.OP_MAX: max,
            cls.OP_GT: lambda c: 1 if c[0] > c[1] else 0,
            cls.OP_LT: lambda c: 1 if c[0] < c[1] else 0,
            cls.OP_EQ: lambda c: 1 if c[0] == c[1] else 0,
        }[type]


@dataclass
class Packet:
    version: int
    type: PacketType
    body: Optional[int]
    children: Optional[List["Packet"]]

    def sum_version(self) -> int:

        if self.body is not None:
            return self.version

        ver_sum = 0
        for child in self.children:
            ver_sum += child.sum_version()

        return ver_sum + self.version

    def eval_expression(self) -> int:

        if self.type == PacketType.LITERAL:
            return self.body

        op = PacketType.associated_op(self.type)
        children = [c.eval_expression() for c in self.children]
        return op(children)


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


def parse_packets(stream: Stream) -> Packet:

    version = stream.take(3)
    type = PacketType(stream.take(3))

    if type == PacketType.LITERAL:
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
            before = stream.consumed
            while True:
                child = parse_packets(stream)
                children.append(child)
                if stream.consumed - before == to_read:
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
    # Ensure only garbage bits are left in message.
    assert stream.checksum() == 0

    part1 = packet.sum_version()
    print(part1)

    part2 = packet.eval_expression()
    print(part2)
