use std::str;

fn main() {
    let score = include_bytes!("input.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            //TODO change to skip_while
            let mut splited_line = line.split(|b| b == &b':');
            let game_id = str::from_utf8(splited_line.next().unwrap()).unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

            let result = if check_max_cubes(splited_line.next().unwrap()) {
                game_id
            } else {
                0
            };
            println!("Game id {}, score {}", game_id, result);

            result

        })
        .sum::<usize>();
    println!("{}", score);
}

fn check_max_cubes(games_result: &[u8]) -> Result<usize,> {
    games_result.split(|b| b == &b';').all(|roll| {
        roll.split(|b| b == &b',').all(|cubes| {
            let mut iter = str::from_utf8(cubes).unwrap().split_whitespace();
            let count = iter.next().unwrap().parse::<usize>().unwrap();
            let color = iter.next().unwrap();
            println!("{} {}",color,count);

            if match match color {
                "red" => count <= 12,
                "blue" => count <= 14,
                "green" => count <= 13,
                _ => true,
            } {

            }

        })
    })
}
