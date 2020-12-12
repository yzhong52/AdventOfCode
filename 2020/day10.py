import collections

file = open("day10.txt", "r")
joltages = [int(x) for x in file.readlines()]

device_joltage = max(joltages) + 3
joltages.append(0)
joltages.append(device_joltage)
joltages.sort()
counter = collections.Counter([x - y for x, y in zip(joltages[1:], joltages)])
print(counter[1] * counter[3])

dp = [0] * 3
dp[0] = 1
for i in range(1, len(joltages)):
    arrangement = 0
    for j in range(3):
        if i - 1 - j < 0 or joltages[i] - joltages[i - 1 - j] > 3:
            break
        arrangement += dp[j]
    dp = [arrangement, dp[0], dp[1]]
print(dp[-1])
