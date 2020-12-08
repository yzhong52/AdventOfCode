file = open("day1.txt", "r")
nums = file.readlines()
nums = list(map(int, nums))
target = 2020
# part 1

seen = set()
for num in nums:
    if target - num in seen:
        print((target - num) * num)
    else:
        seen.add(num)

# part 2

nums.sort()
for i in range(len(nums) - 2):
    j = i + 1
    k = len(nums) - 1
    while j < k:
        sum3 = nums[i] + nums[j] + nums[k]
        if sum3 == target:
            print(nums[i] * nums[j] * nums[k])
            j += 1
            k -= 1
        elif sum3 < target:
            j += 1
        else:
            k -= 1
