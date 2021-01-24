import regex


PATTERN = r'\((?:[^)(]+|(?R))*+\)'


def _evaluate(expr: list) -> str:
    """Evaluates minimal expression"""

    if expr[1] == '+':
        return int(expr[0]) + int(expr[2])

    else:
        return int(expr[0]) * int(expr[2])


def evaluate_expression(expr: str) -> str:
    """
    Evaluates expression left to right with
    parentheses first.
    """

    if not isinstance(expr, str):
        # got regex group by recursion
        expr = expr.group(0)
        expr = expr[1:-1]

    expr = regex.sub(PATTERN, evaluate_expression, expr)
    expr = expr.split()

    while len(expr) != 1:
        part = expr[:3]
        expr = expr[3:]
        val = _evaluate(part)
        expr.insert(0, str(val))

    return expr[0]


def solve(expressions: list) -> int:
    """
    Evaluate the expression on each line of the homework;
    what is the sum of the resulting values?
    """

    results = [int(evaluate_expression(e)) for e in expressions]
    return sum(results)
