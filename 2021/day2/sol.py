import abc
from abc import ABC
from dataclasses import dataclass
from typing import List

Directions = List[List[str]]


@dataclass
class SubPosition:
    horizontal: int
    depth: int
    aim: int

    def get_answer(self) -> int:
        return self.depth * self.horizontal


class BaseSteeringInput(ABC):
    @abc.abstractclassmethod
    def from_directions(cls, directions: Directions):

        raise NotImplementedError

    def update(self, pos: SubPosition) -> None:

        raise NotImplementedError


class Part1Steering(BaseSteeringInput):
    def __init__(self, op: str, amt: int) -> None:
        self._op = op
        self._amt = amt

    @classmethod
    def from_directions(cls, directions: Directions):

        return cls(directions[0], int(directions[1]))

    def update(self, pos: SubPosition) -> None:

        if self._op == "up":
            pos.depth -= self._amt

        elif self._op == "down":
            pos.depth += self._amt

        else:
            pos.horizontal += self._amt


class Part2Steering(BaseSteeringInput):
    def __init__(self, op: str, amt: int) -> None:
        self._op = op
        self._amt = amt

    @classmethod
    def from_directions(cls, directions: Directions):

        return cls(directions[0], int(directions[1]))

    def update(self, pos: SubPosition) -> None:

        if self._op == "up":
            pos.aim -= self._amt

        elif self._op == "down":
            pos.aim += self._amt

        else:
            pos.horizontal += self._amt
            pos.depth += pos.aim * self._amt


def read_directions(filename: str) -> Directions:

    with open(filename) as file:
        data = [line.rsplit() for line in file.readlines()]
    return data


def make_steering_input(
    directions: Directions, part1: bool = True
) -> List[BaseSteeringInput]:

    steering_input = []
    for direction in directions:
        if part1:
            steering_input.append(Part1Steering.from_directions(direction))

        else:
            steering_input.append(Part2Steering.from_directions(direction))

    return steering_input


def dive(steering: List[BaseSteeringInput]) -> int:

    pos = SubPosition(0, 0, 0)
    for s in steering:
        s.update(pos)
    return pos.get_answer()


if __name__ == "__main__":
    raw_directions = read_directions("d2.txt")

    part1_steering = make_steering_input(raw_directions)
    print(dive(part1_steering))

    part2_steering = make_steering_input(raw_directions, part1=False)
    print(dive(part2_steering))
