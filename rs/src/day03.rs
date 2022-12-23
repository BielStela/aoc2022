use itertools::Itertools;

fn get_priority(letter: char) -> usize {
    let letters_lower: &str = "abcdefghijklmnopqrstuvwxyz";
    let letters = letters_lower.to_string() + &letters_lower.to_uppercase();

    letters.find(letter).unwrap() + 1
}

fn part1(input: &str) -> usize {
    let rucksacks = input.lines().collect_vec();
    let mut score = 0;

    for items in rucksacks {
        let split_point = items.len() / 2;
        let (left, right) = items.split_at(split_point);
        let mut seen = Vec::<char>::new();

        for c in left.chars() {
            if right.contains(c) && !seen.contains(&c) {
                score += get_priority(c);
                seen.push(c)
            }
        }
    }
    score
}

fn part2(input: &str) -> usize {
    let rucksacks = input.lines().collect_vec();
    let mut score = 0;
    for chunk in rucksacks.chunks(3) {
        let mut seen = Vec::<char>::new();
        for item in chunk[0].chars() {
            if chunk[1].contains(item) && chunk[2].contains(item) && !seen.contains(&item) {
                score += get_priority(item);
                seen.push(item);
            }
        }
    }
    score
}

pub fn run(input: &str) {
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}
