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

var total = 0;
while (left < right) {
    total += Math.min(leftCount, rightCount);
    if (leftCount < rightCount) {
        left += 1;
        leftCount += (crabs.get(left) ?? 0);
    } else {
        right -= 1;
        rightCount += crabs.get(right) ?? 0;
    }
}

print_result(7, 1, total);
