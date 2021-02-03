import numpy as np


def pass_policy_parser(data: str) -> tuple:
    """
    Parser for password policy.
    Input: '3-4 m: xmmmf'
    Output: (3, 4, 'm', 'xmmmf')
    """

    elems = data.split()
    minchr, maxchr = [int(elem) for elem in elems[0].split('-')]
    target = elems[1][0]
    password = elems[2]
    return minchr, maxchr, target, password


def match_philosophy(data: list) -> int:
    """
    How many passwords are valid according to their policies?
    """

    valid = 0
    for datum in data:
        minchr, maxchr, target, password = pass_policy_parser(datum)
        hits = sum([elem == target for elem in list(password)])
        if minchr <= hits <= maxchr:
            valid += 1
    return valid


def match_new_philosophy(data: list) -> int:
    """
    Each policy actually describes two positions in the password,
    where 1 means the first character, 2 means the second character, and so on.
    (Be careful; Toboggan Corporate Policies have no concept of "index zero"!)
    Exactly one of these positions must contain the given letter.
    Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
    """

    valid = 0
    for datum in data:
        posa, posb, target, password = pass_policy_parser(datum)
        passchrs = list(password)
        hita = passchrs[posa - 1] == target
        hitb = passchrs[posb - 1] == target

        if hita != hitb:
            # xor hits
            valid += 1

    return valid
