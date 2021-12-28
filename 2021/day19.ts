import { assert } from "console";
import { sign } from "crypto";
import { exit } from "process";
import { readStrings } from "./helpers";

type Point3D = [number, number, number]

class Beacon {

    constructor(public pos: Point3D) { }

    rotate(rotationMetrix: RotationMetrix): Beacon {
        let newPos: Point3D = [0, 0, 0]
        for (let i = 0; i < 3; i++) {
            for (let j = 0; j < 3; j++) {
                newPos[i] += rotationMetrix[j][i] * this.pos[i]
            }
        }
        return new Beacon(newPos)
    }

    subtract(beacon1: Beacon): Point3D {
        let diff: Point3D = [0, 0, 0]
        for (let i = 0; i < 3; i++) {
            diff[i] = this.pos[i] - beacon1.pos[i]
        }
        return diff
    }

    add(offset: Point3D) {
        let newPos: Point3D = [0, 0, 0]
        for (let i = 0; i < 3; i++) {
            newPos[i] = this.pos[i] + offset[i]
        }
        return new Beacon(newPos)
    }

    get x() {
        return this.pos[0]
    }
    get y() {
        return this.pos[1]
    }
    get z() {
        return this.pos[2]
    }

    toString(): string {
        return `${this.x},${this.y},${this.z}`
    }
}

class Scanner {
    constructor(public beacons: Beacon[]) { }

    rotate(rotate: RotationMetrix): Scanner {
        return new Scanner(this.beacons.map(beacon => beacon.rotate(rotate)))
    }

    add(transform: Point3D): Scanner {
        return new Scanner(this.beacons.map(beacon => beacon.add(transform)))
    }

    toString(): string {
        return this.beacons.map(beacon => beacon.toString()).join('\n')
    }
}

type RotationMetrix = number[][]

let scanners: Scanner[] = readStrings(19, "\n\n").map(line => {
    let beacons = line
        .split('\n')
        .slice(1)
        .map(row => {
            let pos = row.split(',').map(num => parseInt(num))
            assert(pos.length == 3)
            return new Beacon(pos as Point3D)
        })
    return new Scanner(beacons)
})

let rotations: Array<RotationMetrix> = []

function build_rotation(axis: number[], signs: boolean[]) {
    assert(axis.length == 3)
    assert(signs.length == 3)
    let result: RotationMetrix = new Array()
    for (let i = 0; i < 3; i++) {
        let row = new Array(3).fill(0)
        row[axis[i]] = signs[i] ? 1 : -1
        result.push(row)
    }
    rotations.push(result)
}


function build_rotations(axis: number[], signs: boolean[], remaining: number[]) {
    if (remaining.length == 0) {
        build_rotation(axis, signs)
    }

    for (let i = 0; i < remaining.length; i++) {
        let rest = remaining.slice(0, i).concat(remaining.slice(i + 1))
        build_rotations(axis.concat([remaining[i]]), signs.concat([true]), rest)
        build_rotations(axis.concat([remaining[i]]), signs.concat([false]), rest)
    }
}

build_rotations([], [], [0, 1, 2])


function countOverlap(scaner1: Scanner, scaner2: Scanner, rotation: RotationMetrix, transform: Point3D): number {
    let counter = 0
    let seen: Set<string> = new Set()

    for (let beacon1 of scaner1.beacons) {
        seen.add(beacon1.toString())
    }

    for (let beacon2 of scaner2.beacons) {
        let signature = beacon2.rotate(rotation).add(transform).toString()
        if (seen.has(signature)) {
            counter += 1
        }
    }

    return counter
}


function findMatch(scaner1: Scanner, scaner2: Scanner): [RotationMetrix, Point3D] | undefined {
    for (let beacon1 of scaner1.beacons) {
        for (let beacon2 of scaner2.beacons) {
            for (let rotation of rotations) {
                let transform = beacon1.subtract(beacon2.rotate(rotation))
                let overlaps = countOverlap(scaner1, scaner2, rotation, transform)
                if (overlaps >= 12) {
                    return [rotation, transform]
                }
            }
        }
    }
    return undefined
}

for (var i = 0; i < scanners.length; i++) {
    for (var j = 0; j < scanners.length; j++) {
        if (i != j) {
            console.log(`${i} and ${j}`, findMatch(scanners[i], scanners[j]))
        }
    }
}

// let finalizedScanners = []
// let alignedScanners = [scanners[0]]
// var pendingScanners = new Array(...scanners.slice(1))
// while (alignedScanners.length) {
//     let current = alignedScanners.pop()!
//     finalizedScanners.push(current)

//     let nextPending = []
//     for (let pending of pendingScanners) {
//         let matchingMatrix = findMatch(current, pending)
//         if (matchingMatrix) {
//             let [rotation, transformation] = matchingMatrix
//             console.log(`We found a match! ${rotation} and ${transformation}`)
//             let aligned = pending.rotate(rotation).add(transformation)
//             alignedScanners.push(aligned)
//         } else {
//             nextPending.push(pending)
//         }
//     }
//     pendingScanners = nextPending
// }

// console.log(rotations)
// console.log(finalizedScanners)
