def interpreter(program: list) -> int:
    acc = 0
    last_acc = 0
    pcounter = 0
    executed = [0]

    while True:
        try:
            op, arg = program[pcounter].split()
            arg = int(arg)
        except IndexError:
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

    return last_acc
