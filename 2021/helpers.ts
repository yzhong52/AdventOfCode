import * as fs from 'fs';

export function readStrings(day: number): string[] {
    return fs.readFileSync(`day${day}`, 'utf8').split('\n');
}

export function readNumbers(day: number): number[] {
    return readStrings(day).map(line => parseInt(line));
}

export function print_result(day: number, part: number, text: any) {
    console.log(`Day${day} Part${part}:`, text);
    fs.writeFileSync(`results/day${day}_part${part}.txt`, `${text}`);
}
