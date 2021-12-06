import * as fs from 'fs';

export function readStrings(day: number, separator: string = '\n'): string[] {
    // .trim() here to remove the last line break
    return fs.readFileSync(`day${day}`, 'utf8').trim().split(separator);
}

export function readNumbers(day: number, separator: string = '\n'): number[] {
    return readStrings(day, separator).map(line => parseInt(line));
}

export function print_result(day: number, part: number, text: any) {
    console.log(`Day${day} Part${part}:`, text);
    fs.writeFileSync(`results/day${day}_part${part}.txt`, `${text}`);
}
