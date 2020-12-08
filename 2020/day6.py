file = open("day6.txt", "r")
lines = file.readlines()

cur = set()
part1 = 0
for line in lines + ['\n']:
    if line == '\n':
        part1 += len(cur)
        cur = set()
    else:
        for answer in line.strip():
            cur.add(answer)
print(part1)

part2 = 0
cur = None
for line in lines + ['\n']:
    if line == '\n':
        part2 += len(cur)
        cur = None
    else:
        new_answer = set(line.strip())
        if cur is None:
            cur = new_answer
        else:
            cur = cur.intersection(new_answer)
print(part2)
