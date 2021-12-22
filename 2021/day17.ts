
import { print_result } from "./helpers"

// Taken from input target area: x=269..292, y=-68..-44
const X0 = 269
const X1 = 292
const Y0 = -68
const Y1 = -44

function simulate(vx: number, vy: number): number | undefined {
    // initial pos
    let x = 0
    let y = 0

    let maxY = 0
    while (x <= X1 && y >= Y0) {
        x += vx
        y += vy
        maxY = Math.max(maxY, y)
        if (X0 <= x && x <= X1 && Y0 <= y && y <= Y1) {
            return maxY
        }
        vx = Math.max(0, vx - 1)
        vy--
    }

    return undefined
}

let maxHeight = 0
let hitCounts = 0
for (let i = 0; i < 300; i++) {
    for (let j = -100; j < 100; j++) {
        let height = simulate(i, j)
        hitCounts += height != undefined ? 1 : 0
        if (height != undefined) {
            if (height > maxHeight) {
                maxHeight = height
            }
        }
    }
}

print_result(17, 1, maxHeight)
print_result(17, 2, hitCounts)
