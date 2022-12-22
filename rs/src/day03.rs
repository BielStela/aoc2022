use itertools::Itertools;

fn get_priority(letter: char) -> usize {
    let letters_lower: &str = "abcdefghijklmnopqrstuvwxyz";
    let letters = letters_lower.to_string() + &letters_lower.to_uppercase();

    return letters.find(letter).unwrap() + 1;
}


fn part1(input: &str) -> usize {
    let rucksacks = input.lines().collect_vec();
    let mut score = 0;

    for items in rucksacks {
        let split_point = items.len() / 2;
        let (comp1, comp2) = items.split_at(split_point);
        let mut seen = Vec::<char>::new();

        for c in comp1.chars() {
            if comp2.contains(c) && !seen.contains(&c) {
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

    for group in rucksacks.chunks(3) {
        
    }
    return score;
}


pub fn run(input: &str) {
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}
