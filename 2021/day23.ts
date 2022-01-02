import PriorityQueue from "ts-priority-queue";
import { readStrings } from "./helpers";
import "./collections"
import { exit } from "process";

let rawInput = readStrings(23)

let wall: Set<string> = new Set()
let initialAmphipods: Map<string, Amphipod> = new Map()

type Pos = { y: number, x: number }
const seperator = '-'
function key(pos: Pos): string {
    return `${pos.y}${seperator}${pos.x}`
}

type AmphipodType = 'A' | 'B' | 'C' | 'D'

class Amphipod {
    constructor(public type: AmphipodType, public moves: number, public pos: Pos) { }

    key() {
        return key(this.pos)
    }
    get energy() {
        return Math.pow(10, this.type.charCodeAt(0) - 'A'.charCodeAt(0))
    }

    clone(props: {
        moves: number,
        pos: Pos
    }): Amphipod {
        return new Amphipod(
            this.type,
            props.moves,
            props.pos
        )
    }
}

for (let y = 0; y < rawInput.length; y++) {
    for (let x = 0; x < rawInput[y].length; x++) {
        let cell = rawInput[y][x]
        if (cell == '#') {
            wall.add(key({ y: y, x: x }))
        } else if (cell == '.' || cell == ' ') {
            continue
        } else {
            let amphipod = new Amphipod(cell as AmphipodType, 0, { x: x, y: y })
            initialAmphipods.set(amphipod.key(), amphipod)
        }
    }
}
let immediatelyOutsideRoomPos: Pos[] = [
    { y: 1, x: 3 },
    { y: 1, x: 5 },
    { y: 1, x: 7 },
    { y: 1, x: 9 },
]

let immediatelyOutsideRoomPosSet = new Set(immediatelyOutsideRoomPos.map(pos => key(pos)))
console.log(initialAmphipods)
console.log(immediatelyOutsideRoomPosSet)

let offsets = [[-1, 0], [1, 0], [0, -1], [0, 1]]


class State {
    constructor(public energy: number, public amphipods: Map<string, Amphipod>) { }

    isFinal(): boolean {
        for (let [_, value] of this.amphipods) {
            if (value.pos.y < 2) {
                // There is an amphipod not in the room
                return false
            }
            for (let [dx, dy] of offsets) {
                let neighbour = this.amphipods.get(key({ y: value.pos.y + dy, x: value.pos.x + dx }))
                if (neighbour) {
                    // If there is a neighbour, it has to be of the same type
                    if (neighbour.type != value.type) {
                        return false
                    }
                }
            }
        }
        return true
    }

    clone(props: {
        energy: number,
        amphipods: Map<string, Amphipod>
    }): State {
        return new State(
            props.energy,
            props.amphipods
        )
    }
}

function getPotentialMoves(pos: Pos, amphipods: Map<string, Amphipod>, outsideRoom: boolean = true): Array<[Pos, number]> {
    let result: Array<[Pos, number]> = new Array()
    let visited: Set<string> = new Set()
    function dfs(current: Pos, distance: number) {
        let k = key(current)
        if (visited.has(k)) {
            return
        }
        if (wall.has(k)) {
            return
        }
        if (amphipods.has(k)) {
            return
        }
        visited.add(k)
        if (
            !immediatelyOutsideRoomPosSet.has(k) // cannot stop right outside of the room
            && (outsideRoom && current.y == 1) // if we are moving outside, then y has to be 1
        ) {
            result.push([current, distance])
        }
        for (let [dx, dy] of offsets) {
            dfs({
                x: current.x + dx,
                y: current.y + dy
            }, distance + 1)
        }
    }
    for (let [dx, dy] of offsets) {
        dfs({
            x: pos.x + dx,
            y: pos.y + dy
        }, 1)
    }

    return result
}

function debug(amphipods: Iterable<Amphipod>) {
    let buffer: Array<Array<string>> = new Array()
    for (let i = 0; i < rawInput.length; i++) {
        buffer.push(new Array(rawInput[i].length).fill(' '))
    }

    for (let w of wall) {
        let [y, x] = w.split(seperator).map(value => parseInt(value))
        buffer[y][x] = '#'
    }

    for (let amphipod of amphipods) {
        buffer[amphipod.pos.y][amphipod.pos.x] = amphipod.type
    }

    console.log(buffer.map(row => row.join('')).join('\n'))
}

let pq: PriorityQueue<State> = new PriorityQueue({ comparator: (s1, s2) => s1.energy - s2.energy })
pq.queue(new State(0, initialAmphipods))

let iteration = 0
let maxSize = 0
while (pq.length != 0 && pq.length < 80000) {
    let currentState = pq.dequeue()!
    maxSize = Math.max(pq.length, maxSize)
    iteration++
    console.log("\nIteration:", iteration)
    console.log("Current energy:", currentState.energy)
    console.log("Total states:", pq.length)
    debug(currentState.amphipods.values())

    if (currentState.isFinal()) {
        console.log(currentState.energy)
        break
    }

    for (let [_, amphipod] of currentState.amphipods) {
        if (amphipod.moves >= 1) { // TODO: change back to 1
            // Each amphipod can make no more than 2 moves. It will move
            // out of the room, and then move back to the right room.
            continue
        }
        // if (amphipod.type != 'B' && amphipod.type != 'C' && amphipod.type != 'D') {
        //     // Let's just move a tyep for now
        //     continue
        // }

        let potentialMoves = getPotentialMoves(amphipod.pos, currentState.amphipods)
        for (let [newPos, step] of potentialMoves) {
            let newAmphipod = amphipod.clone({
                pos: newPos,
                moves: amphipod.moves + 1
            })
            let newAmphipods = currentState.amphipods.clone()
            newAmphipods.delete(amphipod.key())
            newAmphipods.set(newAmphipod.key(), newAmphipod)
            let newState = new State(
                step * amphipod.energy + currentState.energy,
                newAmphipods
            )
            pq.queue(newState)
        }
    }
}

console.log(maxSize)
