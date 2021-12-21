import { assert } from "console"
import { print_result, readStrings } from "./helpers"

abstract class Packet {
    version: number

    // 1) Packets with type ID 4 represent a literal value.
    // 2) Every other type of packet (any packet with a type ID other than 4)
    // represent an operator that performs some calculation on one or more
    // sub-packets contained within.
    typeId: number

    constructor(version: number, typeId: number) {
        this.version = version
        this.typeId = typeId
    }

    versionNumberSum(): number {
        return this.version
    }

    abstract evaulate(): number
}

class LiteralPacket extends Packet {

    value: number = 0

    constructor(version: number, typeId: number, value: number) {
        super(version, typeId)
        this.value = value
    }

    override toString(): string {
        return `LT_v${this.version}_(${this.value})`
    }

    override evaulate(): number {
        return this.value
    }
}

class OperatorPacket extends Packet {
    // 1) If the length type ID is 0, then the next 15 bits are a number that
    // represents the total length in bits of the sub-packets contained by this
    // packet.
    // 2) If the length type ID is 1, then the next 11 bits are a number that
    // represents the number of sub-packets immediately contained by this packet.
    lengthTypeID: number

    lengthIndicator: number

    subPackets: Array<Packet> = []

    constructor(version: number, typeId: number, lengthTypeId: number, lengthIndicator: number) {
        super(version, typeId)
        this.lengthTypeID = lengthTypeId
        assert(lengthIndicator > 0, `lengthIndicator ${lengthIndicator} is supposed to be greater than 0.`)
        this.lengthIndicator = lengthIndicator
    }

    isParsed(i: number): boolean {
        if (this.lengthTypeID == 0) {
            return i >= this.lengthIndicator
        }
        else if (this.lengthTypeID == 1) {
            return this.subPackets.length >= this.lengthIndicator
        }
        return false
    }

    override versionNumberSum(): number {
        return this.version + this.subPackets.reduce((previous, current) => previous + current.versionNumberSum(), 0)
    }

    override toString(): string {
        let childString = this.subPackets.map(packet => packet.toString()).join(",")
        return `OP_v${this.version}_i${this.lengthTypeID}_l${this.lengthIndicator}[${childString}]`
    }

    override evaulate(): number {
        let values = this.subPackets.map(packet => packet.evaulate())
        switch (this.typeId) {
            case 0:
                return values.reduce((previous, current) => previous + current, 0)
            case 1:
                return values.reduce((previous, current) => previous * current, 1)
            case 2:
                return Math.min(...values)
            case 3:
                return Math.max(...values)
            case 5:
                assert(values.length == 2, `There should be 2 childrens. But got ${values.length}`)
                return values[0] > values[1] ? 1 : 0
            case 6:
                assert(values.length == 2, `There should be 2 childrens. But got ${values.length}`)
                return values[0] < values[1] ? 1 : 0
            case 7:
                assert(values.length == 2, `There should be 2 childrens. But got ${values.length}`)
                return values[0] == values[1] ? 1 : 0
        }
        throw new Error(`Operator type id not handled: ${this.typeId}`)
    }
}


function hexToBinary(hex: string) {
    // #typescript-limitation.
    //
    // `parseInt` cannot handle bigger numbers, and can fail silently
    // e.g., `parseInt("8A004A801A8002F478", 16).toString(2)` should yield
    //
    //     100010100000000001001010100000000001101010000000000000101111010001111000
    //
    // But instead, it gives:
    //
    //     100010100000000001001010100000000001101010000000000000000000000000000000
    //
    // That's why we need menual parsing here by first converting it to an array.
    return Array.from(hex).map(char => {
        return parseInt(char, 16).toString(2).padStart(4, '0')
    }).join("")
}

function parseBits(hex: string): Packet {
    let root = new OperatorPacket(-1, -1, 1, 1)
    let stack: Array<OperatorPacket> = new Array(1).fill(root)

    let bits = hexToBinary(hex)

    var i = 0
    while (i < bits.length && stack.length > 0) {
        // First 3 bits are the packet version
        let packetVersion = parseInt(bits.slice(i, i += 3), 2)

        // The next 3 bits are the packet ID
        let packetTypeId = parseInt(bits.slice(i, i += 3), 2)

        if (packetTypeId == 4) {
            // Packets with type ID 4 represent a literal value
            let binary: string = ""
            while (true) {
                let isEnd = bits[i] == '0'
                binary += bits.slice(i + 1, i += 5)
                if (isEnd) {
                    break
                }
            }
            let literalValue = parseInt(binary, 2)
            let packet = new LiteralPacket(packetVersion, packetTypeId, literalValue)
            stack[stack.length - 1].subPackets.push(packet)
        } else {
            // Any other type Id indicates an operator packet
            let lengthTypeID = parseInt(bits.slice(i, i += 1))
            var lengthIndicator: number = 0
            if (lengthTypeID == 0) {
                let binary = bits.slice(i, i += 15)
                lengthIndicator = i + parseInt(binary, 2)
            } else if (lengthTypeID == 1) {
                lengthIndicator = parseInt(bits.slice(i, i += 11), 2)
            }
            let operatorPacket = new OperatorPacket(packetVersion, packetTypeId, lengthTypeID, lengthIndicator)
            stack[stack.length - 1].subPackets.push(operatorPacket)
            stack.push(operatorPacket)
        }
        while (stack.length > 0 && stack[stack.length - 1].isParsed(i)) {
            stack.pop()
        }
    }
    assert(root.subPackets.length == 1)
    return root.subPackets[0]
}


let test1 = parseBits(parseInt("D2FE28", 16).toString(2))
assert(test1 instanceof LiteralPacket)
assert((test1 as LiteralPacket).value = 2021)

let test2 = parseBits("38006F45291200")
assert(test2 instanceof OperatorPacket)
let test2_root = test2 as OperatorPacket
assert(test2_root.version == 1)
assert(test2_root.typeId == 6)
assert(test2_root.subPackets.length == 2)
assert(test2_root.subPackets[0] instanceof LiteralPacket)
let test2_child0 = test2_root.subPackets[0] as LiteralPacket
assert(test2_child0.value == 10)
assert(test2_root.subPackets[1] instanceof LiteralPacket)
let test2_child1 = test2_root.subPackets[1] as LiteralPacket
assert(test2_child1.value == 20)

// Part 2 tests
assert(parseBits("C200B40A82").evaulate() == 3)
assert(parseBits("04005AC33890").evaulate() == 54)
assert(parseBits("880086C3E88112").evaulate() == 7)
assert(parseBits("CE00C43D881120").evaulate() == 9)
assert(parseBits("D8005AC2A8F0").evaulate() == 1)
assert(parseBits("F600BC2D8F").evaulate() == 0)
assert(parseBits("9C005AC2F8F0").evaulate() == 0)
assert(parseBits("9C0141080250320F1802104A08").evaulate() == 1)

let line = readStrings(16)[0]
let parsed = parseBits(line)
print_result(16, 1, parsed.versionNumberSum())
print_result(16, 2, parsed.evaulate())
