import * as fs from 'fs';
let lines: string[]  = fs.readFileSync('day1', 'utf8').split('\n');
let numbers: number[] = lines.map(line => parseInt(line));

function count_increased(gap: number): number {
    var count = 0;
    for (var i = gap; i < numbers.length; i++) {
        if (numbers[i] > numbers[i - gap]) {
            count += 1
        }
    }
    return count;
}

console.log("Part 1:", count_increased(1));
console.log("Part 2:", count_increased(3));
