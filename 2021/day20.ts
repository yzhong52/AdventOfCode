import { assert } from "console";
import { print_result, readStrings } from "./helpers";

let [imageEnhancementAlgorithm, imageInput] = readStrings(20, "\n\n")

let initialImage = imageInput.split('\n')

class Image {
    constructor(public pixels: Array<string>, public background: string) { }
}

function decode(image: Image): Image {
    function getImageValue(i: number, j: number): string {
        if (0 <= i && i < image.pixels.length && 0 <= j && j < image.pixels[i].length) {
            return image.pixels[i][j]
        } else {
            return image.background
        }
    }
    function getBinary(i: number, j: number): string {
        let binary = ""
        for (let di of [-1, 0, 1]) {
            for (let dj of [-1, 0, 1]) {
                binary += getImageValue(i + di, j + dj) == '#' ? '1' : '0'
            }
        }
        return binary
    }

    function getValue(binary: string): string {
        assert(binary.length == 9)
        let position = parseInt(binary, 2)
        return imageEnhancementAlgorithm[position]
    }

    function getNewBackground(): string {
        let binary = new Array(9).fill(image.background == '#' ? '1' : '0').join('')
        return getValue(binary)
    }

    let padding = 2
    let height = image.pixels.length + padding * 2
    let width = image.pixels[0].length + padding * 2

    let background = getNewBackground()

    let outputImage: Array<Array<string>> = new Array()
    for (let i = 0; i < height; i++) {
        outputImage.push(new Array(width).fill(background))
    }


    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            let binary = getBinary(i - padding, j - padding)
            outputImage[i][j] = getValue(binary)
        }
    }

    let output = outputImage.map(row => row.join(''))

    return new Image(output, background)
}

function countBright(image: Array<string>): number {
    var count = 0
    for (let row of image) {
        for (let cell of row) {
            count += cell == '#' ? 1 : 0
        }
    }
    return count
}

let part1 = Array.from(Array(2).keys()).reduce((img, _) => decode(img), new Image(initialImage, '.'))
print_result(20, 1, countBright(part1.pixels))

let part2 = Array.from(Array(50).keys()).reduce((img, _) => decode(img), new Image(initialImage, '.'))
print_result(20, 1, countBright(part2.pixels))
