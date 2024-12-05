fn is_safe(levels: &Vec<u32>) -> bool {
    if !levels.is_sorted() && !levels.is_sorted_by(|a, b| a >= b) {
        return false;
    }

    return levels.windows(2).all(|pair| {
        let diff = pair[0].abs_diff(pair[1]);

        return diff >= 1 && diff <= 3;
    });
}

fn main() {
    let mut safe_reports = 0;
    let mut dampened_safe_reports = 0;

    for report in util::read_input_to_iter("./inputs/day2") {
        let levels: Vec<u32> = report
            .split_whitespace()
            .map(|level| level.parse().unwrap())
            .collect();

        if is_safe(&levels) {
            safe_reports += 1;
            dampened_safe_reports += 1;
            continue;
        }

        let is_safe_dampened = (0..levels.len()).any(|i| {
            let mut filtered = levels.clone();

            filtered.remove(i);

            is_safe(&filtered)
        });

        if is_safe_dampened {
            dampened_safe_reports += 1;
        }
    }

    println!("Day 2 part 1 solution: {}", safe_reports);
    println!("Day 2 part 2 solution: {}", dampened_safe_reports);
}
