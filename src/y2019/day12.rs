use super::super::helpers::parser::*;
use super::super::helpers::models::*;
use std::collections::HashMap;

pub fn parse_points(data: &Vec<String>) -> Vec<Point3D> {
    let mut points: Vec<Point3D> = vec![];
    for row in data {
        // Remove '<' and '>' from "<x=-5, y=4, z=9>".
        let sub_str = &row[1..row.len() - 1];

        let components: Vec<&str> = sub_str.split(',').map(|x| x.split('=').last().unwrap()).collect();

        let point = Point3D {
            x: components[0].parse::<i32>().unwrap(),
            y: components[1].parse::<i32>().unwrap(),
            z: components[2].parse::<i32>().unwrap(),
        };

        points.push(point);
    }
    points
}

fn adjust(left: i32, right: i32) -> i32 {
    if left == right {
        0
    } else {
        (left - right) / (left - right).abs()
    }
}

pub fn part1(input: Input<Vec<String>>) -> Answer<i32> {
    let mut points: Vec<Point3D> = parse_points(&input.data);
    let mut velocities: Vec<Point3D> = vec![Point3D::origin(); points.len()];

    let mut total_energy: i32 = 0;
    for _ in 0..1000 {
        let mut next_velocities = velocities.clone();

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                next_velocities[i].x += adjust(points[j].x, points[i].x);
                next_velocities[i].y += adjust(points[j].y, points[i].y);
                next_velocities[i].z += adjust(points[j].z, points[i].z);

                next_velocities[j].x += adjust(points[i].x, points[j].x);
                next_velocities[j].y += adjust(points[i].y, points[j].y);
                next_velocities[j].z += adjust(points[i].z, points[j].z);
            }
        }

        for i in 0 .. points.len() {
            points[i].x += next_velocities[i].x;
            points[i].y += next_velocities[i].y;
            points[i].z += next_velocities[i].z;

            println!("{:?} {:?}", points[i], velocities[i]);
        }

        velocities = next_velocities;

        total_energy = 0;
        for i in 0 .. points.len() {
            let kinetic_energy = velocities[i].x.abs() + velocities[i].y.abs() + velocities[i].z.abs();
            let potential_energy = points[i].x.abs() + points[i].y.abs() + points[i].z.abs();
            total_energy += kinetic_energy * potential_energy
        }
        println!(" - - - - - {} ", total_energy);
    }
    return Answer { question: input.question, result: total_energy };
}


pub fn part2(input: Input<Vec<String>>) -> Answer<usize> {
    return Answer { question: input.question, result: 0 };
}


