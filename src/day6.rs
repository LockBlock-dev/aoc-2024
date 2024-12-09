use ndarray::Array2;

fn step_to_next_position(
    map: &mut Array2<char>,
    width: usize,
    height: usize,
    (y, x): (usize, usize),
    (new_y, new_x): (usize, usize),
    current_char: char,
    next_char: char,
) -> bool {
    if new_y < 0 || new_y >= height || new_x < 0 || new_x >= width {
        map[(y, x)] = 'X';
        return true;
    }

    if map[(new_y, new_x)] == '#' {
        map[(y, x)] = next_char;
    } else {
        map[(new_y, new_x)] = current_char;
        return true;
    }

    return false;
}

fn step(mut map: &mut Array2<char>, width: usize, height: usize, (y, x): (usize, usize)) -> bool {
    return match map[(y, x)] {
        '^' => step_to_next_position(&mut map, width, height, (y, x), (y - 1, x), '^', '>'),

        '>' => step_to_next_position(&mut map, width, height, (y, x), (y, x + 1), '>', 'v'),

        'v' => step_to_next_position(&mut map, width, height, (y, x), (y + 1, x), 'v', '<'),

        '<' => step_to_next_position(&mut map, width, height, (y, x), (y, x - 1), '<', '^'),

        _ => false,
    };
}

fn main() {
    let lines: Vec<_> = util::read_input_to_iter("./inputs/day6").collect();
    let width = lines[0].len();
    let height = lines.len();

    let map = lines
        .into_iter()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut map = Array2::from_shape_vec((height, width), map).unwrap();

    let mut is_patrolling = true;

    while is_patrolling {
        let guard_pos = map
            .clone()
            .into_iter()
            .position(|c| ['^', '>', 'v', '<'].contains(&c));

        if guard_pos.is_none() {
            is_patrolling = false;
            break;
        }

        let y = guard_pos.unwrap() / height;
        let x = guard_pos.unwrap() % width;

        if step(&mut map, width, height, (y, x)) {
            map[(y, x)] = 'X';
        }
    }

    let visited_positions = map.into_iter().filter(|&c| c == 'X').count();

    println!("Day 6 part 1 solution: {}", visited_positions);
    println!("Day 6 part 2 solution: {}", 0);
}
