from dataclasses import dataclass
from typing import Self

from scipy.optimize import linprog


@dataclass
class Machine:
    buttons: list[list[int]]
    indicator_lights: int
    joltage_requirements: list[int]

    @classmethod
    def from_machine_schematics(cls, schematics: str) -> Self:
        buttons = []
        for schematic in schematics.split():
            if schematic.startswith("["):
                schematic = schematic.replace(".", "0,").replace("#", "1,")
                indicator_lights = to_int(parse_schematic(schematic, grouping="[]"))

            elif schematic.startswith("("):
                buttons.append(parse_schematic(schematic, grouping="()"))

            else:
                joltage_requirements = parse_schematic(schematic, grouping="{}")

        return cls(buttons, indicator_lights, joltage_requirements)


def parse_schematic(schematic: str, grouping: str) -> list[int]:
    return [int(digit) for digit in schematic.strip(grouping).split(",") if digit]


def to_int(schematic: list[int]) -> int:
    res = 0
    for index, digit in enumerate(schematic):
        res |= digit << index
    return res


def maybe_press_button(
    buttons: list[list[int]], next_button: int, current: int, desired: int, presses: int
) -> int | None:
    if current == desired:
        return presses

    if next_button >= len(buttons):
        return

    skip = maybe_press_button(buttons, next_button + 1, current, desired, presses)

    to_press = buttons[next_button]
    for button in to_press:
        current ^= 1 << button

    press = maybe_press_button(buttons, next_button + 1, current, desired, presses + 1)

    if skip is not None and press is not None:
        return min(skip, press)

    return skip or press


def configure_lights(buttons: list[list[int]], desired: int) -> int | None:
    return maybe_press_button(buttons, 0, 0, desired, 0)


def configure_joltages(buttons: list[list[int]], joltages: list[int]) -> int:
    c = [1 for _ in range(len(buttons))]
    A = []
    bounds = [(0, None) for _ in range(len(buttons))]

    for joltage_idx in range(len(joltages)):
        eq = [0 for _ in range(len(buttons))]

        for button_idx, button in enumerate(buttons):
            if joltage_idx in button:
                eq[button_idx] = 1

        A.append(eq)

    res = linprog(c, A_eq=A, b_eq=joltages, bounds=bounds, integrality=1)
    return int(sum(res.x))


def main() -> None:
    with open("input") as file:
        data = file.readlines()

    part_1 = 0
    part_2 = 0

    for schematics in data:
        m = Machine.from_machine_schematics(schematics)
        part_1 += configure_lights(m.buttons, m.indicator_lights)
        part_2 += configure_joltages(m.buttons, m.joltage_requirements)

    print(f"Part 1: {part_1}")
    print(f"Part 2: {part_2}")


if __name__ == "__main__":
    main()
