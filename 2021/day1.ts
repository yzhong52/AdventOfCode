import { print_result, readNumbers } from './helpers';

let numbers: number[] = readNumbers(1);

function count_increased(gap: number): number {
    var count = 0;
    for (var i = gap; i < numbers.length; i++) {
        if (numbers[i] > numbers[i - gap]) {
            count += 1
        }
    }
    return count;
}

print_result(1, 1, count_increased(1));
print_result(1, 2, count_increased(3));
