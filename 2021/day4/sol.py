from typing import List, Tuple

import numpy as np


class BingoBoard:
    def __init__(self, numbers: List[List[int]]) -> None:

        self._numbers = np.array(numbers)
        self._hits = np.zeros((5, 5), dtype=np.uint8)
        self._last_called = 0

    def update(self, number: int) -> None:

        self._last_called = number
        hit = np.argwhere(self._numbers == number)
        if len(hit) > 0:
            self._hits[hit[0][0], hit[0][1]] = 1

    def has_bingo(self) -> bool:

        row_bingo = 5 in np.sum(self._hits, axis=0, dtype=np.uint8)
        col_bingo = 5 in np.sum(self._hits, axis=1, dtype=np.uint8)
        return row_bingo or col_bingo

    def get_score(self) -> int:

        unmarked = self._numbers[self._hits == 0]
        return np.sum(unmarked) * self._last_called


def read_bingo(filename: str) -> Tuple[List[int], List[BingoBoard]]:

    with open(filename) as file:
        numbers = [int(n) for n in file.readline().split(",")]
        lcounter = 0
        boards = []
        buffer = []

        while True:
            line = file.readline()
            if len(line) == 0:
                # EOF
                break

            if line == "\n":
                # reset line counter and buffer
                lcounter = 0
                buffer = []

            else:
                row = [n for n in line.rstrip().split(" ")]
                row = [int(n) for n in row if n != ""]
                buffer.append(row)
                lcounter += 1

                if lcounter == 5:
                    board = BingoBoard(buffer)
                    boards.append(board)

    return numbers, boards


def play_bingo(numbers: List[int], boards: List[BingoBoard]) -> int:

    for number in numbers:
        for board in boards:
            board.update(number)
            if board.has_bingo():
                return board.get_score()


if __name__ == "__main__":
    numbers, boards = read_bingo("d4.txt")
    part1 = play_bingo(numbers, boards)
    print(part1)
