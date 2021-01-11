def passport_reader(path: str) -> dict:
    passports = {}
    passnum = 0

    with open(path, 'r') as file:
        passport = ''

        for line in file:
            if line != '\n':
                passport = ''.join([passport, ' ', line.strip()])
            else:
                attrs = passport.lstrip().split()
                pdict = {}
                for attr in attrs:
                    key, value = attr.split(':')
                    pdict[key] = value
                passports[str(passnum)] = pdict
                passnum += 1
                passport = ''

    return passports


def check_passports(passports: dict) -> int:
    """
    Count the number of valid
    passports - those that have all required fields.
    Treat cid as optional. In your batch file, how many passports are valid?
    """

    counter = 0
    req_attrs = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

    for key, val in passports.items():
        p = passports.get(key)
        attrs = p.keys()
        valid = len(req_attrs & attrs) == 7
        if valid:
            counter += 1
    return counter
