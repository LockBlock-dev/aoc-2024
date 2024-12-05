use util;

fn main() {
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in util::read_input_to_iter("./inputs/day1") {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        first_list.push(parts[0].parse().unwrap());
        second_list.push(parts[1].parse().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let total_distances: u32 = first_list
        .iter()
        .zip(second_list.iter())
        .map(|(first, second)| first.abs_diff(*second))
        .sum();

    let similarity_score: u32 = first_list
        .iter()
        .map(|num| num * u32::try_from(second_list.iter().filter(|n| **n == *num).count()).unwrap())
        .sum();

    println!("Day 1 part 1 solution: {}", total_distances);
    println!("Day 1 part 2 solution: {}", similarity_score);
}
