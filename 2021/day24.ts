import { readStrings } from "./helpers"

let instructions = readStrings(24)

function run(number: string) {
    if (number.length != 14) {
        return false
    }

    for (let instruction of instructions) {
        let parts = instruction.split(' ')
        console.log(parts)
    }
}

let lower = (Math.pow(10, 14) - 1) / 9 * 8
console.log(lower, lower.toString().length)
for (let i = Math.pow(10, 14) - 1; i >= lower; i--) {
    run(i.toString())
    break
}
