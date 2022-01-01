import { print_result } from "./helpers"
import { assertEqual } from "./utils/asserts"
import "./collections"

class DireacDice {
    public rolls = 0
    constructor(public currentValue: number = 0) { }

    roll3(): number {
        let res = 0
        for (let i = 0; i < 3; i++) {
            this.currentValue++
            res += this.currentValue
        }
        this.rolls += 3
        return res
    }
}

function simulate(pawn1: number, pawn2: number) {
    let dice = new DireacDice()
    let player1: number = 0
    let player2: number = 0

    while (true) {
        pawn1 += dice.roll3()
        pawn1 = (pawn1 - 1) % 10 + 1
        player1 += pawn1
        if (player1 >= 1000) {
            break
        }

        pawn2 += dice.roll3()
        pawn2 = (pawn2 - 1) % 10 + 1
        player2 += pawn2
        if (player2 >= 1000) {
            break
        }
    }

    return Math.min(player1, player2) * dice.rolls
}

print_result(21, 1, simulate(4, 5))


let rolls = [1, 2, 3]
let splits = new Array(10).fill(0)
for (let i of rolls) {
    for (let j of rolls) {
        for (let k of rolls) {
            splits[i + j + k] += 1
        }
    }
}
console.log("The splits of the universe for 3 dice rolls", splits)

type Props = {
    pos1: number,
    pos2: number,
    score1: number,
    score2: number,
    flag: boolean, // 'true' if we are calculating wins, 'false' for loose
}

function countUniverse(props: Props): number {
    if (props.score2 >= 21) {
        if (props.flag) {
            return props.score1 > props.score2 ? 1 : 0
        } else {
            return props.score1 > props.score2 ? 0 : 1
        }
    }

    let total = 0
    for (let moves = 3; moves < 10; moves++) {
        let pos1 = (props.pos1 + moves - 1) % 10 + 1
        let score1 = props.score1 + pos1
        total += splits[moves] * countUniverse({
            pos1: props.pos2,
            pos2: pos1,
            score1: props.score2,
            score2: score1,
            flag: !props.flag
        })
    }
    return total
}

let player1Wins = countUniverse({ pos1: 4, pos2: 5, score1: 0, score2: 0, flag: true })
let player2Wins = countUniverse({ pos1: 4, pos2: 5, score1: 0, score2: 0, flag: false })
print_result(21, 2, Math.max(player1Wins, player2Wins))
