use std::collections::HashMap;

fn main() {
    let mut page_ordering: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut middle_page_numbers_total = 0;
    let mut reordered_middle_page_numbers_total = 0;

    for line in util::read_input_to_iter("./inputs/day5") {
        if line.contains("|") {
            let orders: Vec<&str> = line.split("|").collect();
            let key: u32 = orders[0].parse().unwrap();
            let value: u32 = orders[1].parse().unwrap();

            if page_ordering.contains_key(&key) {
                let mut old_value = page_ordering.get(&key).unwrap().clone();
                old_value.push(value);

                page_ordering.insert(key, old_value);
            } else {
                page_ordering.insert(key, [value].to_vec());
            }
        } else if line.contains(",") {
            let pages: Vec<u32> = line
                .split(",")
                .map(|page| page.parse::<u32>().unwrap())
                .collect();

            updates.push(pages);
        }
    }

    for update in updates {
        let mut invalid_order = false;
        let mut reordered_update = update.clone();

        for page_pair in update.windows(2) {
            let first = page_pair.first().unwrap();
            let second = page_pair.last().unwrap();

            if page_ordering.contains_key(second) {
                let orders = page_ordering.get(second).unwrap();

                if orders.contains(first) {
                    invalid_order = true;
                    break;
                }
            }
        }

        if !invalid_order {
            let middle_idx: usize = update.len() / 2;
            middle_page_numbers_total += update[middle_idx];
        } else {
            for i in 0..reordered_update.len() {
                for j in i + 1..reordered_update.len() {
                    let first = reordered_update[i];
                    let second = reordered_update[j];

                    if page_ordering.contains_key(&second) {
                        let orders = page_ordering.get(&second).unwrap();

                        if orders.contains(&first) {
                            reordered_update.swap(i, j);
                        }
                    }
                }
            }

            let middle_idx: usize = reordered_update.len() / 2;
            reordered_middle_page_numbers_total += reordered_update[middle_idx];
        }
    }

    println!("Day 5 part 1 solution: {}", middle_page_numbers_total);
    println!(
        "Day 5 part 2 solution: {}",
        reordered_middle_page_numbers_total
    );
}
