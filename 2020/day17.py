from typing import List

file = open("day17.txt", "r")
grid = [list(line.strip()) for line in file.readlines()]

Grid3D = List[List[List[str]]]
Grid4D = List[Grid3D]


def show_grid(g: Grid3D):
    for layer in g:
        for row in layer:
            print("".join(row))
        print()


def show_grid_4d(g: Grid4D):
    for wi, layer in enumerate(g):
        print(f"w={wi}")
        show_grid(layer)


def count_active_neighbors(g: Grid3D, xi: int, yi: int, zi: int, include_self: bool = False):
    total = 0
    for nz in [zi - 1, zi, zi + 1]:
        for ny in [yi - 1, yi, yi + 1]:
            for nx in [xi - 1, xi, xi + 1]:
                if ((nx, ny, nz) != (xi, yi, zi) or include_self) and \
                        0 <= nz < len(g) and \
                        0 <= ny < len(g[nz]) and \
                        0 <= nx < len(g[nz][ny]):
                    total += g[nz][ny][nx] == '#'
    return total


def mutation(is_active, active_neighbors):
    if is_active:
        return '#' if active_neighbors in (2, 3) else '.'
    else:
        return '#' if active_neighbors == 3 else '.'


def part1_grid(grid3d: Grid3D):
    cycle = 6
    size_z = len(grid3d)
    size_y = len(grid3d[0])
    size_x = len(grid3d[0][0])

    for i in range(cycle):
        size_x += 2
        size_y += 2
        size_z += 2

        tmp = [[['.'] * size_x for _ in range(size_y)] for _ in range(size_z)]

        for z in range(size_z):
            for y in range(size_y):
                for x in range(size_x):
                    is_active = 0 < z < size_z - 1 and 0 < y < size_y - 1 and 0 < x < size_x - 1 and \
                                grid3d[z - 1][y - 1][x - 1] == '#'
                    active_neighbors = count_active_neighbors(grid3d, x - 1, y - 1, z - 1)
                    tmp[z][y][x] = mutation(is_active, active_neighbors)
        grid3d = tmp
        print(f"After {i + 1} cycle: \n")
        show_grid(grid3d)
    return grid3d


def count_active(g: Grid3D):
    return sum([cell == '#' for layer in g for row in layer for cell in row])


def count_active_neighbors_4d(g: Grid4D, xi: int, yi: int, zi: int, wi: int):
    total = 0
    for nw in [wi - 1, wi, wi + 1]:
        if 0 <= nw < len(g):
            total += count_active_neighbors(g=g[nw], xi=xi, yi=yi, zi=zi, include_self=nw != wi)
    return total


def part2_grid(grid4d: Grid4D):
    cycle = 6

    size_w = len(grid4d)
    size_z = len(grid4d[0])
    size_y = len(grid4d[0][0])
    size_x = len(grid4d[0][0][0])

    for i in range(cycle):
        size_x += 2
        size_y += 2
        size_z += 2
        size_w += 2

        tmp = [[[['.'] * size_x for _ in range(size_y)] for _ in range(size_z)] for _ in range(size_w)]

        for w in range(size_w):
            for z in range(size_z):
                for y in range(size_y):
                    for x in range(size_x):
                        is_active = 0 < w < size_w - 1 and 0 < z < size_z - 1 and \
                                    0 < y < size_y - 1 and 0 < x < size_x - 1 and \
                                    grid4d[w - 1][z - 1][y - 1][x - 1] == '#'
                        active_neighbors = count_active_neighbors_4d(grid4d, xi=x - 1, yi=y - 1, zi=z - 1, wi=w - 1)
                        tmp[w][z][y][x] = mutation(is_active, active_neighbors)
        grid4d = tmp
        print(f"After {i + 1} cycle: \n")
        show_grid_4d(grid4d)
    return grid4d


def count_active_4d(g: Grid4D):
    return sum(map(count_active, g))


part1 = count_active(part1_grid([grid]))
part2 = count_active_4d(part2_grid([[grid]]))

print(part1)
print(part2)
