use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(u8, u8)> {
    let moves = input.lines()
        .map(|line| line.split(' ')
            .map(|code| code.bytes().next().unwrap())
            .collect_vec()
        )
        // convert to ascii value and normalize between 0 and 2. 0: rock, 1: paper...
        .map(|v| (v[0] - b'A', v[1] - b'X'))
        .collect_vec();
    return moves;
}

fn play(input: &str, part1: bool) -> i32 {
    // opponent codes: A = rock, B = paper, C = scissors
    // your codes: X = rock, Y = paper, Z = scissors
    // scores: AX=1, BY=2, CZ=3. loose=0, drawn=3, win=6
    // part 2 X=loose, Y=drawn, Z=win
    parse_input(input)
        .iter()
        .map(|&(elf, me)| {
            // first part `me` is the figure chosen. in the 2nd `me` is the
            // outcome so convert me to figure needed. Done by... why?

            let me = if part1 { me } else { (elf + me + 2) % 3 };
            let out_points = [3, 0, 6];
            // get the index of the of the outcome table as 0: drawn, 1: loose, 2: win
            let out = ((3 + elf - me) % 3) as usize;
            me as i32 + 1 + out_points[out]
        })
        .sum()
}

pub fn run(input: &str) {
    println!("{}", play(input, true));
    println!("{}", play(input, false));
}
