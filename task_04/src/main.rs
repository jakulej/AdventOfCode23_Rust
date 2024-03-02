fn main() {
    let score: usize = include_bytes!("input.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            let mut iter = line
                .splitn(2, |b| b == &b':')
                .nth(1)
                .unwrap()
                .split(|b| b == &b'|');
            let winning_numbers: Vec<&str> = std::str::from_utf8(iter.next().unwrap())
                .unwrap()
                .split_whitespace()
                .collect();
            let score = std::str::from_utf8(iter.next().unwrap())
                .unwrap()
                .split_whitespace()
                .filter(|card_score| {
                    winning_numbers.iter().any(|winning_number| {
                        winning_number.parse::<usize>().unwrap()
                            == card_score.parse::<usize>().unwrap()
                    })
                })
                .count();

            match score {
                0 => 0,
                _ => 2usize.pow({ score - 1usize } as u32),
            }



        })
        .sum::<usize>();
    print!("{}", score);
}
