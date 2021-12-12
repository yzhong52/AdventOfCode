import * as fs from 'fs';

export function readStrings(day: number, separator: string = '\n'): string[] {
    // .trim() here to remove the last line break
    return fs.readFileSync(`day${day}`, 'utf8').trim().split(separator);
}

export function readNumbers(day: number, separator: string = '\n'): number[] {
    return readStrings(day, separator).map(line => parseInt(line));
}

export function readGrid(day: number): Array<Array<number>> {
    let lines = readStrings(day);

    let grid: Array<Array<number>> = lines.map(line => {
        // This is a stupid behavior in Typescript, #ts-limitations
        // https://stackoverflow.com/q/262427/1035008
        // 'Array.from(line).map(parseInt)' won't produce correct result.
        return Array.from(line).map(value => parseInt(value))
    })

    return grid;
}

export function print_result(day: number, part: number, text: any) {
    console.log(`Day${day} Part${part}:`, text);
    fs.writeFileSync(`results/day${day}_part${part}.txt`, `${text}`);
}

// There isn't a sleep implementation in Typescript
// https://stackoverflow.com/a/37764963/1035008
export function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export const colors = {
    Reset: "\x1b[0m",
    Bright: "\x1b[1m",
    Dim: "\x1b[2m",
    Underscore: "\x1b[4m",
    Blink: "\x1b[5m",
    Reverse: "\x1b[7m",
    Hidden: "\x1b[8m",

    FgBlack: "\x1b[30m",
    FgRed: "\x1b[31m",
    FgGreen: "\x1b[32m",
    FgYellow: "\x1b[33m",
    FgBlue: "\x1b[34m",
    FgMagenta: "\x1b[35m",
    FgCyan: "\x1b[36m",
    FgWhite: "\x1b[37m",

    BgBlack: "\x1b[40m",
    BgRed: "\x1b[41m",
    BgGreen: "\x1b[42m",
    BgYellow: "\x1b[43m",
    BgBlue: "\x1b[44m",
    BgMagenta: "\x1b[45m",
    BgCyan: "\x1b[46m",
    BgWhite: "\x1b[47m",
};

// There is no easy way to deep clone a Set otherwise, #ts-limitations
// https://stackoverflow.com/q/28150967/1035008
export function clone<T>(set: Set<T>): Set<T> {
    let newSet: Set<T> = new Set();
    for (let element of set) {
        newSet.add(element)
    }
    return newSet
}
