def memory_game(mem: dict, init: int) -> int:
    """
    In this game, the players take turns saying numbers.
    They begin by taking turns reading from a list
    of starting numbers (your puzzle input).
    Then, each turn consists of considering the
    most recently spoken number:

    If that was the first time the number has been spoken,
    the current player says 0.
    Otherwise, the number had been spoken before;
    the current player announces how many turns apart the
    number is from when it was previously spoken.
    """

    ls = init
    init_nums = len(mem.keys())

    for round in range(init_nums + 2, 2021):
        if ls in mem.keys():
            diff = round - 1 - mem[ls]
            mem[ls] = round - 1
            ls = diff

        else:
            mem[ls] = round - 1
            ls = 0

    return ls
