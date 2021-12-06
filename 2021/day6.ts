import { print_result, readNumbers } from "./helpers";

let numbers = readNumbers(6, ',');

let initial_counters: Array<number> = new Array(9).fill(0);

for (let num of numbers) {
    initial_counters[num] += 1
}

function count_fish(days: number) {
    let counters: Array<number> = initial_counters;
    for (let i = 0; i < days; i += 1) {
        let counters1: Array<number> = new Array(9).fill(0);

        counters1[6] += counters[0]
        counters1[8] += counters[0]

        for (let cid = 1; cid < counters.length; cid += 1) {
            counters1[cid - 1] += counters[cid];
        }

        counters = counters1;
    }
    return counters.reduce((sum, cur) => sum + cur)
}

print_result(6, 1, count_fish(80))
print_result(6, 2, count_fish(256))
