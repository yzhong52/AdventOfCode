file = open("day3.txt", "r")
map = file.readlines()
map = [i.strip() for i in map]

width = len(map[0])
height = len(map)


def count_trees(right: int, down: int) -> int:
    trees = 0
    x = 0
    y = 0
    while y < height:
        trees += map[y][x] == '#'
        y = y + down
        x = (x + right) % width
    return trees


part1 = count_trees(right=3, down=1)
print(part1)

part2 = 1
for right, down in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]:
    part2 *= count_trees(right=right, down=down)
print(part2)
