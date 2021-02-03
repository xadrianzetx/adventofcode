import regex
import numpy as np


PATTERN = r'\((?:[^)(]+|(?R))*+\)'


def _evaluate(expr: list) -> str:
    """Evaluates minimal expression"""

    if expr[1] == '+':
        return int(expr[0]) + int(expr[2])

    else:
        return int(expr[0]) * int(expr[2])


def op_mul(expr: str) -> str:
    """Multiplication operation"""

    expr = expr.split(' * ')
    expr = np.array([int(x) for x in expr], dtype=np.int64)
    expr = np.product(expr)

    return str(expr)


def op_add(expr: str) -> str:
    """Addition operation"""

    expr = expr.split(' + ')
    expr = [int(x) for x in expr]
    expr = np.sum(expr)

    return str(expr)


def evaluate_expression_lr(expr: str) -> str:
    """
    Evaluates expression left to right with
    parentheses first.
    """

    if not isinstance(expr, str):
        # got regex group by recursion
        expr = expr.group(0)
        expr = expr[1:-1]

    expr = regex.sub(PATTERN, evaluate_expression_lr, expr)
    expr = expr.split()

    while len(expr) != 1:
        part = expr[:3]
        expr = expr[3:]
        val = _evaluate(part)
        expr.insert(0, str(val))

    return expr[0]


def evaluate_expression_sum(expr: str) -> str:
    """
    Evaluate expression with multiplication first
    (parentheses still have priority)
    """

    if not isinstance(expr, str):
        # got regex group by recursion
        expr = expr.group(0)
        expr = expr[1:-1]

    expr = regex.sub(PATTERN, evaluate_expression_sum, expr)
    expr = expr.split(' * ')
    expr = [op_add(e) for e in expr]
    expr = op_mul(' * '.join(expr))

    return expr


def solve(expressions: list, part_two: bool = False) -> int:
    """
    Evaluate the expression on each line of the homework;
    what is the sum of the resulting values?
    """
    f = evaluate_expression_sum if part_two else evaluate_expression_lr
    results = [int(f(e)) for e in expressions]
    return sum(results)
