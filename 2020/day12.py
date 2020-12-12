file = open("day12.txt", "r")
lines = file.readlines()

transformation = {'N': (0, 1), "S": (0, -1), "E": (1, 0), 'W': (-1, 0)}


def take_actions_part1(actions):
    dx, dy = 1, 0
    x, y = 0, 0

    for line in actions:
        action = line[0]
        unit = int(line[1:])

        if action == 'F':
            x += dx * unit
            y += dy * unit
        elif action in transformation:
            t = transformation[action]
            x += t[0] * unit
            y += t[1] * unit
        elif action in {'L', 'R'}:
            angle = -unit if action == 'R' else unit
            angle %= 360
            if angle == 90:
                dx, dy = -dy, dx
            elif angle == 180:
                dx, dy = -dx, -dy
            elif angle == 270:
                dx, dy = dy, -dx
    return abs(x) + abs(y)


def take_actions_part2(actions):
    x, y = 0, 0

    # position of the waypoint
    wx, wy = 10, 1

    for line in actions:
        action = line[0]
        unit = int(line[1:])

        if action == 'F':
            x += wx * unit
            y += wy * unit
        elif action in transformation:
            t = transformation[action]
            wx += t[0] * unit
            wy += t[1] * unit
        elif action in {'L', 'R'}:
            angle = -unit if action == 'R' else unit
            angle %= 360
            if angle == 90:
                wx, wy = -wy, wx
            elif angle == 180:
                wx, wy = -wx, -wy
            elif angle == 270:
                wx, wy = wy, -wx
    return abs(x) + abs(y)


print(take_actions_part1(lines))
print(take_actions_part2(lines))
