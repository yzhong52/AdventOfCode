file = open("day4.txt", "r")
lines = file.readlines()

# Part 1
valid = 0
required_fields = {'hcl', 'iyr', 'eyr', 'ecl', 'pid', 'byr', 'hgt'}
cur = set()
for line in lines + ['\n']:
    if line == '\n':
        valid += cur.issuperset(required_fields)
        cur = set()
    else:
        for data in line.split(' '):
            key, value = data.split(':')
            cur.add(key)
print(valid)


# Part 2
def is_valid(key: str, value: str):
    if key == "byr":
        return len(value) == 4 and 1920 <= int(value) <= 2002
    elif key == "iyr":
        return len(value) == 4 and 2010 <= int(value) <= 2020
    elif key == 'eyr':
        return len(value) == 4 and 2020 <= int(value) <= 2030
    elif key == "hgt":
        if value[-2:] == "cm":
            return 150 <= int(value[:-2]) <= 193
        if value[-2:] == "in":
            return 59 <= int(value[:-2]) <= 76
    elif key == 'hcl':
        if len(value) == 7 and value[0] == '#':
            for i in range(1, 7):
                if 'a' <= value[i] <= 'f' or '0' <= value[i] <= '9':
                    pass
                else:
                    return False
            return True
    elif key == 'ecl':
        return value in {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}
    elif key == 'pid':
        return len(value) == 9 and value.isdigit()
    return False


cur = set()
valid = 0
for line in lines + ['\n']:
    if line == '\n':
        valid += cur.issuperset(required_fields)
        cur = set()
    else:
        for data in line.split(' '):
            key, value = data.split(':')
            value = value.strip()
            if is_valid(key, value):
                cur.add(key)

print(valid)
