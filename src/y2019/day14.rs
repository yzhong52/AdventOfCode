use super::super::helpers::parser::*;
use std::collections::HashMap;

#[derive(Debug)]
struct RequiredMaterial {
    material: String,
    quantity: i32,
}

impl RequiredMaterial {
    fn new(raw: &str) -> RequiredMaterial {
        let material_with_quantity: Vec<&str> = raw.split(' ').collect();
        RequiredMaterial {
            material: material_with_quantity[1].to_string(),
            quantity: material_with_quantity[0].parse::<i32>().unwrap(),
        }
    }
}


pub fn part1(input: Input<Vec<String>>) -> Answer<i32> {
    let mut reactions: HashMap<String, (Vec<RequiredMaterial>, i32)> = HashMap::new();

    let target_material: String = String::from("FUEL");
    let source_material: String = String::from("ORE");

    for row in input.data {
        let components: Vec<&str> = row.split(" => ").collect();
        let input_materials: Vec<RequiredMaterial> = components[0].split(", ").map(RequiredMaterial::new).collect();
        let output_material = RequiredMaterial::new(components[1]);
        assert!(!reactions.contains_key(&output_material.material), "There are multiple ways to generate one material");
        reactions.insert(output_material.material, (input_materials, output_material.quantity));
    }

    // Negative value indicates that we have some remain from other reactions
    let mut required_materials: HashMap<String, i32> = HashMap::new();
    required_materials.insert(target_material.clone(), 1);

    let mut next_ups = vec![target_material.clone()];
    while next_ups.len() > 0 {
        let next_up = next_ups.pop().unwrap();

        let (input_materials, output_quality) = reactions.get(&next_up).unwrap();

        let required_quality = required_materials.get(&next_up).unwrap();
        if required_quality > &0 {
            println!("Require: {} {}", next_up, required_quality);

            let scale = required_quality / output_quality + (required_quality % output_quality != 0) as i32;

            required_materials.insert(next_up.clone(), required_quality - scale * output_quality);

            println!("from: {:?}, to: {}, scale: {}", input_materials, next_up, scale);

            for input in input_materials {
                let required_quantity = required_materials.get(&input.material).unwrap_or(&0) + input.quantity * scale;

                if required_quantity > 0 && input.material != "ORE".to_string() {
                    next_ups.push(input.material.clone());
                }
                required_materials.insert(input.material.clone(), required_quantity);
            }
        }
    }

    return Answer { question: input.question, result: required_materials[&source_material] };
}

pub fn part2(input: Input<Vec<String>>) -> Answer<usize> {
    return Answer { question: input.question, result: 0 };
}
