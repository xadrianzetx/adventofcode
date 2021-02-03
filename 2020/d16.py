import re
import numpy as np


def parse_rules(rules: list) -> list:
    """Parses ticket rules and ranges"""

    valid_vals = {}

    for rule in rules:
        rle = re.findall('.*:', rule)[0][:-1]
        rng = re.findall(': (.*)', rule)[0].split(' or ')
        allrng = np.array([], dtype='int')

        for r in rng:
            low, high = r.split('-')
            rngarr = np.arange(int(low), int(high) + 1)
            allrng = np.append(rngarr, allrng)

        valid_vals[rle] = allrng

    return valid_vals


def parse_tickets(tickets: list) -> list:
    """Parses ticket contents"""

    all_fields = []

    for ticket in tickets:
        fields = np.array(ticket.split(','), dtype='int')
        all_fields.append(fields)

    return np.array(all_fields)


def find_invalid_rate(tickets: np.array, valid_fields: list) -> int:
    """
    Consider the validity of the nearby tickets you scanned.
    What is your ticket scanning error rate?
    """

    rate = 0

    for ticket in tickets:
        valid = np.array([f in valid_fields for f in ticket])
        passed = all(valid)

        if not passed:
            r = np.sum(ticket[~valid])
            rate += r

    return rate


def remove_invalid_tickets(tickets: np.array, rules: dict) -> np.array:
    """Removes invalid tickets from the list"""

    valid_tickets = []
    valid_fields = np.concatenate(list(rules.values()), axis=0)

    for ticket in tickets:
        valid = [f in valid_fields for f in ticket]
        if all(valid):
            valid_tickets.append(ticket)

    return np.array(valid_tickets)


def find_candidates(tickets: np.array, rules: dict) -> list:
    """Finds wich rules match which fields"""

    fields = []

    for col in tickets.T:
        uvals = np.unique(col)
        hits = []

        for rule, range in rules.items():
            matches = [val in range for val in uvals]

            if all(matches):
                hits.append(rule)

        fields.append(hits)

    return fields


def decode_ticket(nearby: np.array, rules: dict, ticket: list) -> int:
    """
    Once you work out which field is which, look for the six fields
    on your ticket that start with the word departure.
    What do you get if you multiply those six values together?
    """

    prod = 1
    fields = np.empty(shape=(len(ticket)), dtype='U32')

    while len(rules.keys()) > 0:
        candidates = find_candidates(nearby, rules)

        for idx, cand in enumerate(candidates):
            if len(cand) == 0 or len(cand) > 1:
                # field has multiple
                # options at this stage
                # or has been assigned a name already
                continue

            # assign name to a field
            rule = cand[0]
            fields[idx] = rule
            rules.pop(rule)

    for rowname, value in zip(fields, ticket):
        if rowname.startswith('departure'):
            prod *= value

    return prod
