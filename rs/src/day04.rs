use itertools::Itertools;

type Range = (usize, usize);

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|value| value.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn full_overlap(pair: &&(Range, Range)) -> bool {
    // Calculate the full_overlap of two ranges
    let (a, b) = pair;
    ((a.0 >= b.0) & (a.1 <= b.1)) | ((a.0 <= b.0) & (a.1 >= b.1))
}

fn any_overlap(pair: &&(Range, Range)) -> bool {
    let (a, b) = pair;
    ((a.0 >= b.0) & (a.0 <= b.1)) | ((b.0 >= a.0) & (b.0 <= a.1))
}

fn part1(pairs: &[(Range, Range)]) -> usize {
    pairs.iter().filter(full_overlap).count()
}

fn part2(pairs: &[(Range, Range)]) -> usize {
    pairs.iter().filter(any_overlap).count()
}

pub fn run(input: &str) {
    let pairs = parse_input(input);
    println!("{:?}", part1(&pairs));
    println!("{:?}", part2(&pairs));
}
