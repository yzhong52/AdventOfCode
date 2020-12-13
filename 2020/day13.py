file = open("day13.txt", "r")
lines = file.readlines()


def find_timestamp_part1(arrival_time: int, bus_schedule: str):
    from heapq import heappop, heappush

    buses = [int(bus) for bus in bus_schedule.split(',') if bus != 'x']

    schedule = [(bus, bus) for bus in buses]

    while schedule[0][0] < arrival_time:
        depart_time, bus = heappop(schedule)
        heappush(schedule, (depart_time + bus, bus))

    depart_time, bus = heappop(schedule)
    return (depart_time - arrival_time) * bus


# Find the earliest timestamp such that the first bus ID departs at that time and each subsequent listed bus ID departs
# at that subsequent minute
def find_timestamp_part2(bus_schedule: str):
    bus_offsets = [(int(bus), offset) for offset, bus in enumerate(bus_schedule.split(',')) if bus != 'x']
    multiplier, _ = bus_offsets[0]
    timestamp = 0
    for bus, offset in bus_offsets[1:]:
        while (timestamp + offset) % bus != 0:
            timestamp += multiplier
        multiplier *= bus
    return timestamp


print(find_timestamp_part1(int(lines[0]), lines[1]))
print(find_timestamp_part2(lines[1]))


def find_timestamp_part1_jon(arrival_time: int, bus_schedule: str):
    """
    Translated from Jon
    """
    buses = [int(bus) for bus in bus_schedule.split(',') if bus != 'x']

    def departure_time_for_bus(bus):
        return (arrival_time // bus + (arrival_time % bus != 0)) * bus

    departure_time, target_bus = min([(departure_time_for_bus(bus), bus) for bus in buses])

    return (departure_time - arrival_time) * target_bus


def find_timestamp_part2_patric(vs):
    """
    Copied from Patrick
    https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_(direct_construction)
    """
    from functools import reduce
    from itertools import count

    def bezout(nm, pk):
        """find n' = n + km such that n' mod p = k"""
        (n, m), (p, k) = nm, pk
        return next(x for x in count(n, m) if (x - k) % p == 0), p * m

    return reduce(bezout, sorted(((v, -k) for (v, k) in zip(vs, range(len(vs))) if v), reverse=True), (0, 1))[0]


print(find_timestamp_part1_jon(int(lines[0]), lines[1]))
print(find_timestamp_part2_patric([int(bus) if bus.isdigit() else None for bus in lines[1].split(",")]))
