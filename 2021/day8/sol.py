from typing import Dict, List

PANELS = {
    "abcefg": "0",
    "cf": "1",
    "acdeg": "2",
    "acdfg": "3",
    "bcdf": "4",
    "abdfg": "5",
    "abdefg": "6",
    "acf": "7",
    "abcdefg": "8",
    "abcdfg": "9",
}


def read_notes(filename: str) -> List[str]:

    with open(filename) as file:
        data = [line.rstrip().split(" | ") for line in file]
    return data


def count_digits(notes: List[str]) -> int:

    total = 0
    for note in notes:
        output = note[1].split(" ")
        counts = [1 for x in output if len(x) in [2, 4, 3, 7]]
        total += sum(counts)

    return total


def sum_digits(notes: List[str]) -> int:

    total = 0
    for note in notes:
        signals = note[0].split(" ")
        mappings = decode(signals)
        mappings = {v: k for k, v in mappings.items()}
        values = note[1].split(" ")
        total += find_value(values, mappings)

    return total


def decode(signals: List[str]) -> Dict[str, str]:

    panel = {}
    cands = []

    # Step 1 - find displayed 1
    # This gives 2 cands for panels "C", "F"
    pone = list([s for s in signals if len(s) == 2][0])
    panel["c"] = pone
    panel["f"] = pone
    cands.extend(pone)

    # Step 2 - find displayed 7 and diff with panels from 1
    # This gives out mapping to panel "A"
    pseven = list([s for s in signals if len(s) == 3][0])
    chr = [c for c in pseven if c not in cands][0]
    panel["a"] = chr
    cands.extend(chr)

    # Step 3 - find displayed 4 to find two cands
    # for panels "B" and "D"
    pfour = list([s for s in signals if len(s) == 4][0])
    chrs = [c for c in pfour if c not in cands]
    panel["b"] = chrs
    panel["d"] = chrs
    cands.extend(chrs)

    # Step 3 - find displayed 8 to get two cands
    # for panels "E" and "G"
    peight = list([s for s in signals if len(s) == 7][0])
    chrs = [c for c in peight if c not in cands]
    panel["e"] = chrs
    panel["g"] = chrs

    # Step 4 - find displayed 2 - 5 char with both
    # candidates from "E" and "G" panels
    # This solves "B"-"D" and "C"-"F" collisions
    ptwo = list([s for s in signals if set(chrs) <= set(s) and len(s) == 5][0])
    bd = panel["b"]
    if bd[0] in ptwo:
        panel["d"] = bd[0]
        panel["b"] = bd[1]
    else:
        panel["d"] = bd[1]
        panel["b"] = bd[0]

    cf = panel["c"]
    if cf[0] in ptwo:
        panel["c"] = cf[0]
        panel["f"] = cf[1]
    else:
        panel["c"] = cf[1]
        panel["f"] = cf[0]

    # Step 5 - find displayed # - 5 char with "C" and "F" panel
    # This solves remaining "E"-"G" collision
    chrs = [panel["c"], panel["f"]]
    pthree = list([s for s in signals if set(chrs) <= set(s) and len(s) == 5][0])
    eg = panel["e"]
    if eg[0] in pthree:
        panel["g"] = eg[0]
        panel["e"] = eg[1]
    else:
        panel["g"] = eg[1]
        panel["e"] = eg[0]

    return panel


def find_value(signals: List[str], chrmap: Dict[str, str]) -> int:

    output = ""
    for signal in signals:
        output += PANELS[remap(signal, chrmap)]
    return int(output)


def remap(value: str, chrmap: Dict[str, str]) -> str:

    remap = ""
    for chr in value:
        remap += chrmap[chr]
    return "".join(sorted(remap))


if __name__ == "__main__":
    notes = read_notes("d8.txt")

    part1 = count_digits(notes)
    print(part1)

    part2 = sum_digits(notes)
    print(part2)
