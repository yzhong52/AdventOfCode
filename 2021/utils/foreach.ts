
// https://stackoverflow.com/q/38434337/1035008
declare global {
    interface String {
        forEach(f: (char: string) => void): void
    }
}

String.prototype.forEach = function (f: (char: string) => void) {
    Array.from(this).forEach(f)
}

export { };
