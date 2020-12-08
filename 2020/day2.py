file = open("day2.txt", "r")
lines = file.readlines()

# Part 1

valid = 0
for line in lines:
    policy, password = line.split(": ")
    nums, char = policy.split(" ")
    num1, num2 = map(int, nums.split("-"))
    if int(num1) <= sum([c == char for c in password]) <= int(num2):
        valid += 1
print(valid)

# Part 2

valid = 0
for line in lines:
    policy, password = line.split(": ")
    nums, char = policy.split(" ")
    num1, num2 = map(int, nums.split("-"))
    if (password[num1 - 1] == char) ^ (password[num2 - 1] == char):
        valid += 1
print(valid)
