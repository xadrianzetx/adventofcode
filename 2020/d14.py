import re
import itertools
import numpy as np


def emulate_decoder(program: list) -> int:
    """
    Execute the initialization program.
    What is the sum of all values left
    in memory after it completes?
    (Do not truncate the sum to 36 bits.)
    """

    mask = None
    mem = {}

    for op in program:
        if op[:3] == 'mem':
            val = int(re.findall('= (.*)', op)[0])
            addr = int(re.findall('\[(.*?)\]', op)[0])

            for pos, bit in enumerate(mask[::-1]):
                if bit == 'X':
                    continue
                elif int(bit) == 0:
                    # clear bit at pos
                    val &= ~(1 << pos)
                else:
                    # set bit at pos
                    val |= (1 << pos)

            mem[addr] = val

        else:
            mask = re.findall('= (.*)', op)[0]

    return sum(mem.values())


def memory_addres_decoder(program: list) -> int:
    """
    Execute the initialization program
    using an emulator for a version 2 decoder chip.
    What is the sum of all values left in memory after it completes?
    """

    mask = None
    mem = {}

    for op in program:
        if op[:3] == 'mem':
            value = int(re.findall('= (.*)', op)[0])
            addr = int(re.findall('\[(.*?)\]', op)[0])

            addr = bin(addr)[2:]
            addr = list(addr.zfill(len(mask)))

            for pos, val in enumerate(mask):
                if val in ['X', '1']:
                    addr[pos] = val

            addr = np.array(addr)
            flt = addr[addr == 'X']

            for prd in itertools.product([0, 1], repeat=flt.shape[0]):
                tmp = np.array(addr, copy=True)
                tmp[tmp == 'X'] = prd
                memaddr = int(''.join(tmp), 2)
                mem[memaddr] = value

        else:
            mask = re.findall('= (.*)', op)[0]

    return sum(mem.values())
