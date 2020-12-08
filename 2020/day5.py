file = open("day5.txt", "r")
lines = file.readlines()


def to_seat(code: str) -> (int, int):
    row = 0
    for i in range(7):
        row = row * 2 + (code[i] == 'B')
    col = 0
    for i in range(7, 10):
        col = col * 2 + (code[i] == 'R')
    return row * 8 + col


# Part 1
all_seats = [to_seat(line) for line in lines]
print(max(all_seats))

# Part 2
sorted_seats = sorted(all_seats)
for left, right in zip(sorted_seats, sorted_seats[1:]):
    if left + 1 != right:
        print(left + 1)
