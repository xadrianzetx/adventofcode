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
