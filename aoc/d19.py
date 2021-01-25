import itertools


def parse_rulebook(data: list) -> dict:
    """Parses rulebook"""

    rules = {}

    for datum in data:
        num, rule = datum.split(':')
        rules[int(num)] = rule.lstrip().replace('"', '')

    return rules


def parse_rules(rule: int, allrules: dict) -> list:
    """Parses rules into combinations"""

    rulestr = allrules.get(rule)

    if rulestr in ['a', 'b']:
        return [rulestr]

    combos = []
    options = rulestr.split('|')

    for option in options:
        chrops = []
        subrules = option.split()

        for sub in subrules:
            out = parse_rules(int(sub), allrules)
            chrops.append(out)

        chrprd = itertools.product(*chrops)
        chrprd = [''.join(c) for c in chrprd]

        for prd in chrprd:
            combos.append(prd)

    return combos


def is_valid(msg: str, rulea: list, ruleb: list) -> bool:
    """
    rulea is rule 8
    ruleb is rule 31

    length of combos for both 8 and 31 is 8.
    rule 0 starts with rule 8 (which is 42 repeated n times)
    and then goes to 11 (which is 42 n 31)
    so a valid rule 0 is always gonna be 42 repeated n1 times
    followed by 31 repeated n2 times where n1 > n2, n1 >= 2
    n2 >= 1
    """

    rac = 0
    rbc = 0

    while True:
        # iterate from beggining
        # trying to match rule 8
        sub = msg[:8]
        if sub in rulea:
            rac += 1
            msg = msg[8:]
        else:
            break

    while True:
        # iterate from end
        # trying to match rule 31
        sub = msg[-8:]
        if sub in ruleb:
            rbc += 1
            msg = msg[:-8]
        else:
            break

    if rac > rbc and rac >= 2 and rbc >= 1 and len(msg) == 0:
        return True
    return False


def count_msgs_matching(msgs: list, rules: dict) -> int:
    """
    How many messages completely match rule 0 + loops?
    """

    count = 0
    rulea = parse_rules(8, rules)
    ruleb = parse_rules(31, rules)

    for msg in msgs:
        if is_valid(msg, rulea, ruleb):
            count += 1

    return count
