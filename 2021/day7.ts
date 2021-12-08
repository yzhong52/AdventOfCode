import { print_result, readNumbers } from "./helpers";

let numbers: Array<number> = readNumbers(7, ',');

let crabs: Map<number, number> = new Map();
for (let num of numbers) {
    crabs.set(num, (crabs.get(num) ?? 0) + 1)
}
var left = Math.min(...crabs.keys())
var right = Math.max(...crabs.keys())
var leftCount = crabs.get(left)!
var rightCount = crabs.get(right)!

var part1 = 0;
while (left < right) {
    part1 += Math.min(leftCount, rightCount);
    if (leftCount < rightCount) {
        left += 1;
        leftCount += (crabs.get(left) ?? 0);
    } else {
        right -= 1;
        rightCount += crabs.get(right) ?? 0;
    }
}

// Let's do binary search for part 2

var left = Math.min(...crabs.keys())
var right = Math.max(...crabs.keys())

function calculate_fuel(distance: number): number {
    return distance * (distance + 1) / 2
}

function calculate_total_fuel(position: number): number {
    var sum = 0;
    for (let [crab_pos, crab_count] of crabs) {
        sum += crab_count * calculate_fuel(Math.abs(crab_pos - position))
    }
    return sum
}

let t0 = new Date();

var part2 = 0;
while (left < right) {
    var middle = Math.floor((left + right) / 2);

    let m0 = calculate_total_fuel(middle);
    let m1 = calculate_total_fuel(middle + 1);

    part2 = Math.min(m1, m0);
    if (m0 > m1) {
        left = middle + 1;
    } else if (m0 < m1) {
        right = middle;
    } else {
        break;
    }
}
let t1 = new Date()
console.log(t0, t1, t1.getMilliseconds() - t0.getMilliseconds())

print_result(7, 1, part1);
print_result(7, 1, part2);
