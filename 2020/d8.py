import copy


def interpreter(program: list) -> int:
    """
    Part one
    """

    acc = 0
    last_acc = 0
    pcounter = 0
    executed = [0]
    exit_code = 1

    while True:
        try:
            op, arg = program[pcounter].split()
            arg = int(arg)
        except IndexError:
            exit_code = 0
            break

        if op == 'nop':
            pcounter += 1

        elif op == 'acc':
            pcounter += 1
            acc += arg

        elif op == 'jmp':
            pcounter += arg

        if pcounter in executed:
            break

        last_acc = acc
        executed.append(pcounter)

    return last_acc, exit_code


def self_fixing_interpreter(program: str) -> int:
    """
    Part two
    """

    for i in range(len(program)):
        tested_op, arg = program[i].split()
        pcopy = copy.deepcopy(program)

        if tested_op == 'nop':
            pcopy[i] = f'jmp {arg}'

        elif tested_op == 'jmp':
            pcopy[i] = f'nop {arg}'

        else:
            # no reason to test program on acc
            continue

        acc, exit_code = interpreter(pcopy)

        if exit_code == 0:
            break

    return acc
