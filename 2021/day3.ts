import { print_result, readStrings } from "./helpers";

let lines = readStrings(3);
var counters = new Array(lines[0].length).fill(0);
for (let line of lines) {
    for (let i = 0; i < line.length; i++) {
        if (line[i] == '1') {
            counters[i] += 1
        } else {
            counters[i] -= 1
        }
    }
}
console.log("counters", counters);

var gamma = 0;
var epsilon = 0;
for (let counter of counters) {
    gamma *= 2;
    epsilon *= 2;
    if (counter >= 0) {
        gamma += 1;
    } else if (counter < 0) {
        epsilon += 1;
    }
}

print_result(3, 1, gamma * epsilon)


function find_rating(
    bit_criteria: (count1: number, count0: number) => string
) {
    var bit_strings = lines;

    var index = 0;

    // Continue until there is only one number left
    while (bit_strings.length > 1) {
        let bits_at_pos = bit_strings.map(bit_string => bit_string[index]);
        let count1s = bits_at_pos.reduce((count, current) => current == '1' ? count + 1 : count, 0);
        let count0s = bit_strings.length - count1s;
        let current_bit = bit_criteria(count1s, count0s);
        bit_strings = bit_strings.filter(bit_string => bit_string[index] == current_bit)
        index += 1;
    }
    return bit_strings[0];
}

let oxygen_generator_rating = find_rating((count1s, count0s) => count1s >= count0s ? '1' : '0')
let CO2_scrubber_rating = find_rating((count1s, count0s) => count1s < count0s ? '1' : '0')
console.log('oxygen generator rating:', oxygen_generator_rating, parseInt(oxygen_generator_rating, 2));
console.log('CO2 scrubber rating:', CO2_scrubber_rating, parseInt(CO2_scrubber_rating, 2));
print_result(3, 2, parseInt(oxygen_generator_rating, 2) * parseInt(CO2_scrubber_rating, 2));
