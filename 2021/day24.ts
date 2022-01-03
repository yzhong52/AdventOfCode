import { assert } from "console";
import { exit } from "process";
import { DefaultDict } from "./collections"
import { readStrings } from "./helpers"

const isNumeric = (val: string): boolean => {
    return !isNaN(Number(val));
}

class Memory extends DefaultDict<string, number> {
    constructor() {
        super(() => 0)
    }
    override get(key: string): number {
        if (isNumeric(key)) {
            return parseInt(key)
        } else {
            return super.get(key)
        }
    }
}

function evaluate(number: string, instructions: string[]) {
    if (number.length != 14) {
        return false
    }
    let digits: number[] = new Array(...number).map(value => parseInt(value))
    if (!digits.every(digit => digit != 0)) {
        return false
    }

    let stack: Array<number> = digits.reverse()

    let memory = new Memory()
    for (let i = 0; i < instructions.length; i++) {
        let instruction = instructions[i]
        let parts = instruction.split(' ')
        var a = 0
        var b = 0
        switch (parts[0]) {
            case 'inp':
                memory.set(parts[1], stack.pop()!)
                break
            case 'add':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], a + b)
                break
            case 'mul':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], a * b)
                break
            case 'div':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], Math.floor(a / b))
                break
            case 'mod':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], a % b)
                break
            case 'eql':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], a == b ? 1 : 0)
                break
        }
        // console.log(parts)
        // console.log(memory)
    }
}

// This will take forever to run
// for (let i = Math.pow(10, 14) - 1; i >= 1; i--) {
//     if (run(i.toString(), instructions)) {
//         break
//     }
// }


abstract class Expression {
    constructor(
        public lowerbound: number | undefined = undefined,
        public upperbound: number | undefined = undefined) { }

    isZero() {
        return false
    }

    isOne(): boolean {
        return false
    }

    boundsDesc(): string {
        // return `<${this.lowerbound}~${this.upperbound}>`
        return ''
    }
    bounds(): number[] {
        assert(this.lowerbound != undefined)
        assert(this.upperbound != undefined)
        return [this.lowerbound!, this.upperbound!]
    }
}

class Constant extends Expression {
    constructor(public value: number) {
        super(value, value)
    }

    isZero() {
        return this.value == 0
    }

    override isOne(): boolean {
        return this.value == 1
    }

    override toString(): string {
        return this.value.toString()
    }
}

class Arg extends Expression {
    constructor(public i: number) { super(1, 9) }

    override toString(): string {
        return `A(${this.i})`
    }
}


class Add extends Expression {
    constructor(public left: Expression, public right: Expression) {
        super()
    }

    static apply(lhs: Expression, rhs: Expression): Expression {
        if (lhs.isZero()) {
            return rhs
        }
        if (rhs.isZero()) {
            return lhs
        }
        if (lhs instanceof Constant && rhs instanceof Constant) {
            return new Constant(lhs.value + rhs.value)
        }

        // We can bundle the constant tgt
        // (x + c1) + c2 = x + (c1 + c2)
        if (lhs instanceof Add && rhs instanceof Constant) {
            if (lhs.right instanceof Constant) {
                return Add.apply(
                    lhs.left,
                    new Constant(lhs.right.value + rhs.value)
                )
            }
        }

        let result = new Add(lhs, rhs)
        if (lhs.lowerbound != undefined && rhs.lowerbound != undefined) {
            result.lowerbound = lhs.lowerbound + rhs.lowerbound
        }
        if (lhs.upperbound != undefined && rhs.upperbound != undefined) {
            result.upperbound = lhs.upperbound + rhs.upperbound
        }
        return result
    }

    override toString(): string {
        return `(${this.left.toString()}+${this.right.toString()})${this.boundsDesc()}`
    }
}

class Mul extends Expression {
    constructor(
        public left: Expression,
        public right: Expression
    ) {
        super()
    }

    static apply(a: Expression, b: Expression): Expression {
        if (a.isZero() || b.isZero()) {
            return new Constant(0)
        }
        if (a.isOne()) {
            return b
        }
        if (b.isOne()) {
            return a
        }

        if (a instanceof Constant && b instanceof Constant) {
            return new Constant(a.value * b.value)
        }

        let result = new Mul(a, b)

        let bounds = a.bounds()
            .map(bound1 => b.bounds()
                .map(bound2 => bound1 * bound2))
            .flat()
            .sort()

        result.lowerbound = bounds[0]
        result.upperbound = bounds[bounds.length - 1]
        return result
    }

    override toString(): string {
        return `(${this.left.toString()}*${this.right.toString()})${this.boundsDesc()}`
    }
}

class Mod extends Expression {
    constructor(
        public left: Expression,
        public right: Expression
    ) {
        super()
    }

    static apply(lhs: Expression, rhs: Expression): Expression {
        if (lhs instanceof Constant && rhs instanceof Constant) {
            return new Constant(lhs.value % rhs.value)
        }

        // When X is bigger than b
        // (a * X + b) % X = b % X = b
        if (lhs instanceof Add) {
            if (lhs.left instanceof Mul) {
                if (lhs.left.right instanceof Constant) {
                    if (rhs instanceof Constant) {
                        if (lhs.left.right.value == rhs.value) {
                            if (lhs.right.upperbound! < rhs.value) {
                                return lhs.right
                            }
                        }
                    }
                }
            }
        }

        let result = new Mod(lhs, rhs)
        if (rhs.upperbound != undefined) {
            result.upperbound = rhs.upperbound - 1
            result.lowerbound = 0
        }
        if (rhs.lowerbound != undefined && lhs.upperbound != undefined) {
            if (rhs.lowerbound > lhs.upperbound) {
                result.lowerbound = lhs.lowerbound
            }
        }
        if ((lhs.lowerbound ?? 0) < 0 || (rhs.lowerbound ?? 0) < 0) {
            // TODO: Yuchen - negative number not handled
            assert(false, lhs.bounds(), rhs.bounds())
            exit(0)
        }
        return result
    }

    override toString(): string {
        return `(${this.left.toString()}%${this.right.toString()})${this.boundsDesc()}`
    }
}

class Div extends Expression {
    constructor(
        public left: Expression,
        public right: Expression
    ) {
        super()
    }

    static apply(lhs: Expression, rhs: Expression): Expression {
        if (rhs.isOne()) {
            return lhs
        }
        if (lhs instanceof Constant && rhs instanceof Constant) {
            return new Constant(Math.floor(lhs.value / rhs.value))
        }

        // When X is bigger than b
        // (a * X + b) / X = a
        if (lhs instanceof Add) {
            if (lhs.left instanceof Mul) {
                if (lhs.left.right instanceof Constant) {
                    if (rhs instanceof Constant) {
                        if (lhs.left.right.value == rhs.value) {
                            if (lhs.right.upperbound! < rhs.value) {
                                return lhs.left.left
                            }
                        }
                    }
                }
            }
        }

        let result = new Div(lhs, rhs)
        if (lhs.upperbound != undefined && rhs.lowerbound != undefined) {
            result.upperbound = Math.floor(lhs.upperbound / rhs.lowerbound)
        }
        if (lhs.lowerbound != undefined && rhs.upperbound != undefined) {
            result.lowerbound = Math.floor(lhs.lowerbound / rhs.upperbound)
        }

        if ((lhs.lowerbound ?? 0) < 0 || (rhs.lowerbound ?? 0) < 0) {
            // TODO: Yuchen - negative number not handled
            assert(false, lhs.bounds(), rhs.bounds())
            exit(0)
        }
        return result
    }


    override toString(): string {
        return `(${this.left.toString()}/${this.right.toString()})${this.boundsDesc()}`
    }
}


class Eql extends Expression {
    constructor(
        public left: Expression,
        public right: Expression
    ) {
        super(0, 1)
    }

    static apply(a: Expression, b: Expression): Expression {
        if (a.upperbound != undefined && b.lowerbound != undefined) {
            if (a.upperbound < b.lowerbound) {
                return new Constant(0)
            }
        }

        if (a.lowerbound != undefined && b.upperbound != undefined) {
            if (a.lowerbound > b.upperbound) {
                return new Constant(0)
            }
        }

        if ((b instanceof Arg) && a instanceof Constant) {
            if (a.value >= 10) {
                return new Constant(0)
            }
        }
        if (a instanceof Constant && b instanceof Constant) {
            return new Constant(a.value == b.value ? 1 : 0)
        }
        return new Eql(a, b)
    }

    override toString(): string {
        return `(${this.left.toString()} == ${this.right.toString()})${this.boundsDesc()}`
    }
}


class Memory2 extends DefaultDict<string, Expression> {
    constructor() {
        super(() => new Constant(0))
    }
    override get(key: string): Expression {
        if (isNumeric(key)) {
            return new Constant(parseInt(key))
        } else {
            return super.get(key)
        }
    }
}

function optimize(instructions: string[]) {

    let memory = new Memory2()

    let i = 0
    let inputIndex = 0
    for (let instruction of instructions) {
        let parts = instruction.split(' ')
        var a: number | Expression = 0
        var b: number | Expression = 0
        switch (parts[0]) {
            case 'inp':
                memory.set(parts[1], new Arg(inputIndex))
                inputIndex += 1
                break
            case 'add':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], Add.apply(a, b))
                break
            case 'mul':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], Mul.apply(a, b))
                break
            case 'div':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], Div.apply(a, b))
                break
            case 'mod':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], Mod.apply(a, b))
                break
            case 'eql':
                a = memory.get(parts[1])
                b = memory.get(parts[2])
                memory.set(parts[1], Eql.apply(a, b))
                break
        }
        console.log(i++)
        console.log(parts)
        for (let [k, v] of memory) {
            console.log("  -", k, v.toString())
        }
    }

    return memory.get('z')
}

let n: any = 1
console.log(n instanceof Expression)
console.log(new Arg(1) instanceof Expression)

let instructions = readStrings(24)
let z = optimize(instructions.slice(0, 131))

console.log("\nFinal", z.toString())
