from scipy.optimize import linprog


def parse_line(line: str):
    all_buttons = []
    for elem in line.split():
        if elem.startswith("("):
            all_buttons.append(parse_buttons(elem))
        if elem.startswith("{"):
            joltages = parse_joltages(elem)
    return all_buttons, joltages


def parse_buttons(data: str) -> list[int]:
    return [int(digit) for digit in data.strip("()").split(",")]


def parse_joltages(data: str) -> list[int]:
    return [int(digit) for digit in data.strip("{}").split(",")]


def solve(buttons: list[list[int]], joltages: list[int]) -> int:
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


with open("input") as file:
    data = file.readlines()

part_2 = 0
for idx, line in enumerate(data):
    res = solve(*parse_line(line))
    print(f"({idx + 1}/{len(data)}) {res}")
    part_2 += res

print(f"Part 2: {part_2}")
