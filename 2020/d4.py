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


def check_byr(val):
    return 1920 <= int(val) <= 2002


def check_iyr(val):
    return 2010 <= int(val) <= 2020


def check_eyr(val):
    return 2020 <= int(val) <= 2030


def chech_hgt(val):
    if val[-2:] == 'in':
        return 59 <= int(val[:-2]) <= 76
    elif val[-2:] == 'cm':
        return 150 <= int(val[:-2]) <= 193
    else:
        return False


def check_hcl(val):
    try:
        if len(list(val)) != 7:
            return False
        int(val[1:], base=16)
        return True
    except ValueError:
        return False


def check_ecl(val):
    return val in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']


def check_pid(val):
    try:
        return len(list(val)) == 9
    except ValueError:
        return False


CHECKLIST = {
    'byr': check_byr,
    'iyr': check_iyr,
    'eyr': check_eyr,
    'hgt': chech_hgt,
    'hcl': check_hcl,
    'ecl': check_ecl,
    'pid': check_pid
}


def strict_check_passports(passports: dict) -> int:
    """
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
    """

    counter = 0
    req_attrs = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

    for key, val in passports.items():
        p = passports.get(key)
        attrs = p.keys()
        valid = len(req_attrs & attrs) == 7
        if not valid:
            continue
        checks = []
        for attr in attrs:
            if attr == 'cid':
                continue
            f = CHECKLIST.get(attr)
            val = p.get(attr)
            valid = f(val)
            checks.append(valid)
        if all(checks):
            counter += 1

    return counter
