from heapq import heappop, heappush

file = open("day13.txt", "r")
lines = file.readlines()


# Estimate of the earliest timestamp you could depart on a bus
def find_timestamp_part1(arrival_time: int, bus_schedule: str):
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
