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


def count_msgs_matching(msgs: list, rules: list) -> int:
    """
    How many messages completely match rule 0?
    """

    count = 0

    for msg in msgs:
        if msg in rules:
            count += 1

    return count
