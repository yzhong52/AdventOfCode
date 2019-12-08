use super::super::helpers::parser::*;

pub fn part1(input: Input<String>) -> Answer<usize> {
    let pixels_count_per_layer = 25 * 6;
    let num_of_layers = input.data.len() / pixels_count_per_layer;

    let mut minimum_number_of_zeros = pixels_count_per_layer;
    let mut result = 0;
    for layer_index in 0..num_of_layers {
        let layer: String = input.data.chars()
            .skip(layer_index * pixels_count_per_layer)
            .take(pixels_count_per_layer)
            .collect();

        let zeros = layer.chars().filter(|x| x == &'0').count();
        let ones = layer.chars().filter(|x| x == &'1').count();
        let twos = layer.chars().filter(|x| x == &'2').count();
        println!("{} {} {}", zeros, ones, twos);

        if minimum_number_of_zeros > zeros {
            minimum_number_of_zeros = zeros;
            result = ones * twos;
        }
    }
    return Answer { question: input.question, result: result };
}

pub fn part2(input: Input<String>) -> Answer<usize> {
    return Answer { question: input.question, result: 0 };
}
