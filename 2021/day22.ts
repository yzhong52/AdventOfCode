import { textSpanContainsTextSpan } from "typescript";
import { print_result, readStrings } from "./helpers";



class Step {
    constructor(public onOrOff: boolean, public boundaries: number[]) { }

    get region(): Region {
        return new Region(this.boundaries)
    }
}

let steps = readStrings(22).map(row => {
    let re = row.match("^(.*) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)$")
    if (re == undefined) {
        throw new Error("unable to parse" + row);
    }
    return new Step(re[1] == 'on', re.slice(2).map(num => parseInt(num)))
})

let cube3D = new Map()
for (let step of steps) {
    for (let x = Math.max(-50, step.boundaries[0]); x <= Math.min(50, step.boundaries[1]); x++) {
        for (let y = Math.max(-50, step.boundaries[2]); y <= Math.min(50, step.boundaries[3]); y++) {
            for (let z = Math.max(-50, step.boundaries[4]); z <= Math.min(50, step.boundaries[5]); z++) {
                cube3D.set(`${x},${y},${z}`, step.onOrOff)
            }
        }
    }
}

let part1 = 0
for (let [_, value] of cube3D) {
    part1 += value
}
print_result(22, 1, part1)


class Region {
    constructor(public boundaries: number[]) { }

    static max: Region = new Region([0, 1, 2, 3, 4, 5].map(index => {
        let values = steps.map(step => step.boundaries[index])
        return index % 2 == 0 ? Math.min(...values) : Math.max(...values)
    }))

    intersect(other: Region): Region {
        let boundaries = [0, 1, 2, 3, 4, 5].map(index => {
            let values = [this.boundaries[index], other.boundaries[index]]
            return index % 2 == 0 ? Math.max(...values) : Math.min(...values)
        })
        return new Region(boundaries)
    }

    count(): number {
        return [0, 2, 4]
            .map(index => Math.max(0, this.boundaries[index + 1] - this.boundaries[index] + 1))
            .reduce((a, b) => a * b, 1)
    }

    subtract(other: Region): Region[] {
        // Given a range in 1D, we can break it into 3 regions.
        // [===|===|===]
        // or 2 regions:
        // [===|===]----|
        // or 1 region:
        // --|--[===]----|
        // or 0 region:
        // --[----]----|----|----
        // In 2D, that's 9 regions; in 3D, that's 27 regions.
        //
        // Out of these 27 regions:
        // 1) There is one that is identical to the 'other' region.
        // 2) There are some of them are not valid (e.g. count is 0)
        // We can filter out both of these.

        let segments = [0, 2, 4].map(index => {
            return [
                this.boundaries[index],
                other.boundaries[index],
                other.boundaries[index + 1],
                this.boundaries[index + 1]
            ]
        })

        let regions = []
        for (let i = 0; i < 3; i++) {
            let boundaries = []
            boundaries.push(segments[0][i])
            boundaries.push(segments[0][i + 1])
            for (let j = 0; j < 3; j++) {
                boundaries.push(segments[1][j])
                boundaries.push(segments[1][j + 1])
                for (let k = 0; k < 3; k++) {
                    boundaries.push(segments[2][k])
                    boundaries.push(segments[2][k + 1])
                    regions.push(new Region(boundaries))
                }
            }
        }

        return regions
            .map(region => region.intersect(this))
            // .map(region => region.intersect(other))
            .filter(region => {
                // If region.boundaries is same as other.boundaries
                if (region.boundaries.every((value, index) => value == other.boundaries[index])) {
                    return false
                }
                if (region.count() == 0) {
                    return false
                }
                return true
            })
    }
}

var regions = [Region.max]

let part2 = 0
while (steps.length) {
    console.log("Number of steps", steps.length)
    console.log("Number of regions", regions.length)

    let currentStep = steps.pop()!
    let nextRegions: Array<Region> = []
    for (let region of regions) {
        if (currentStep.onOrOff) {
            part2 += region.intersect(currentStep.region).count()
        }
        nextRegions.push(...region.subtract(currentStep.region))
    }
    regions = nextRegions
}
print_result(22, 2, part2)
