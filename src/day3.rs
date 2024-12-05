use regex::Regex;

fn main() {
    let re = Regex::new(r"don't\(\)|do\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;
    let mut conditional_result = 0;
    let mut mul_enabled = true;

    for line in util::read_input_to_iter("./inputs/day3") {
        for caps in re.captures_iter(&line) {
            let instruction = caps.get(0).map_or("", |m| m.as_str());
            let first = caps.get(1);
            let second = caps.get(2);

            if instruction == "don't()" {
                mul_enabled = false;
            } else if instruction == "do()" {
                mul_enabled = true;
            } else if instruction.starts_with("mul") && first.is_some() && second.is_some() {
                let res = first.unwrap().as_str().parse::<u32>().unwrap()
                    * second.unwrap().as_str().parse::<u32>().unwrap();

                result += res;

                if mul_enabled {
                    conditional_result += res;
                }
            }
        }
    }

    println!("Day 3 part 1 solution: {}", result);
    println!("Day 3 part 2 solution: {}", conditional_result);
}
