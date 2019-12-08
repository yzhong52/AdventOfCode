use super::super::helpers::parser::*;

const PIXELS_COUNT_PER_LAYER: usize = 25 * 6;
const NOT_INIT: char = '*';

pub fn part1(input: Input<String>) -> Answer<usize> {
    let num_of_layers = input.data.len() / PIXELS_COUNT_PER_LAYER;

    let mut minimum_number_of_zeros = PIXELS_COUNT_PER_LAYER;
    let mut result = 0;
    for layer_index in 0..num_of_layers {
        let layer: String = input.data.chars()
            .skip(layer_index * PIXELS_COUNT_PER_LAYER)
            .take(PIXELS_COUNT_PER_LAYER)
            .collect();

        let zeros = layer.chars().filter(|x| x == &'0').count();
        let ones = layer.chars().filter(|x| x == &'1').count();
        let twos = layer.chars().filter(|x| x == &'2').count();

        if minimum_number_of_zeros > zeros {
            minimum_number_of_zeros = zeros;
            result = ones * twos;
        }
    }
    return Answer { question: input.question, result };
}

pub fn part2(input: Input<String>) -> Answer<String> {
    let num_of_layers = input.data.len() / PIXELS_COUNT_PER_LAYER;

    let mut visible_layer: Vec<char> = vec![NOT_INIT; PIXELS_COUNT_PER_LAYER];

    for layer_index in 0..num_of_layers {
        let layer: String = input.data.chars()
            .skip(layer_index * PIXELS_COUNT_PER_LAYER)
            .take(PIXELS_COUNT_PER_LAYER)
            .collect();

        for (i, c) in layer.chars().into_iter().enumerate() {
            if visible_layer[i] == NOT_INIT && c != '2' {
                visible_layer[i] = c;
            }
        }
    }

    let result: String = visible_layer.into_iter().collect();
    return Answer { question: input.question, result };
}
