use ndarray::Array2;

fn count_words_part1(matrix: &Array2<char>, width: usize, height: usize) -> i32 {
    let directions = [
        (0, 1),   // right
        (1, 1),   // diagonal down-right
        (1, 0),   // down
        (1, -1),  // diagonal down-left
        (0, -1),  // left
        (-1, -1), // diagonal up-left
        (-1, 0),  // up
        (-1, 1),  // diagonal up-right
    ];

    let mut count = 0;

    for i in 0..height {
        for j in 0..width {
            for (dx, dy) in directions {
                let is_xmas = "XMAS".chars().enumerate().all(|(k, character)| {
                    let x = i as isize + k as isize * dx;
                    let y = j as isize + k as isize * dy;

                    return x >= 0
                        && x < height as isize
                        && y >= 0
                        && y < width as isize
                        && matrix[(x as usize, y as usize)] == character;
                });

                if is_xmas {
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn count_words_part2(matrix: &Array2<char>, width: usize, height: usize) -> i32 {
    let directions = [
        (1, 1),   // diagonal down-right
        (1, -1),  // diagonal down-left
        (-1, -1), // diagonal up-left
        (-1, 1),  // diagonal up-right
    ];

    let mut count = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if matrix[(i, j)] == 'A' {
                let is_xmas = directions.iter().all(|&(dx, dy)| {
                    let x1 = i as isize + dx;
                    let y1 = j as isize + dy;

                    let x2 = i as isize - dx;
                    let y2 = j as isize - dy;

                    let c1 = matrix[(x1 as usize, y1 as usize)];
                    let c2 = matrix[(x2 as usize, y2 as usize)];

                    return (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M');
                });

                if is_xmas {
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn main() {
    let lines: Vec<_> = util::read_input_to_iter("./inputs/day4").collect();
    let width = lines[0].len();
    let height = lines.len();

    let map = lines
        .into_iter()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let matrix = Array2::from_shape_vec((height, width), map).unwrap();

    println!(
        "Day 4 part 1 solution: {}",
        count_words_part1(&matrix, width, height)
    );
    println!(
        "Day 4 part 2 solution: {}",
        count_words_part2(&matrix, width, height)
    );
}
