import numpy as np


def declaration_form_counter(path: str) -> list:
    """
    For each group, count the number of questions
    to which anyone answered "yes". What is the sum of those counts?
    """

    forms = []

    with open(path, 'r') as file:
        form = ''

        for line in file:
            if line != '\n':
                form += line.strip()
            else:
                qcounts = len(np.unique(list(form)))
                forms.append(qcounts)
                form = ''

    return np.sum(forms)


def strict_declaration_form_counter(path: str) -> list:
    """
    For each group, count the number of questions
    to which everyone answered "yes". What is the sum of those counts?
    """

    forms = []
    with open(path, 'r') as file:
        form = ''
        pcount = 0
        for line in file:
            if line != '\n':
                form += line.strip()
                pcount += 1
            else:
                chrs = list(form)
                uchrs, chrcounts = np.unique(chrs, return_counts=True)
                qcounts = len(uchrs[chrcounts == pcount])
                forms.append(qcounts)
                form = ''
                pcount = 0

    return np.sum(forms)
