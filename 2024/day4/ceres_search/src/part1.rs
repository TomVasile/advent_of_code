// --- Day 4: Ceres Search ---

// "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

// As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

// This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:

// ..X...
// .SAMX.
// .A..A.
// XMAS.S
// .X....

// The actual word search will be full of letters instead. For example:

// MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX

// In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

// ....XXMAS.
// .SAMXMS...
// ...S..A...
// ..A.A.MS.X
// XMASAMX.MM
// X.....XA.A
// S.S.S.S.SS
// .A.A.A.A.A
// ..M.M.M.MM
// .X.X.XMASX

// Take a look at the little Elf's word search. How many times does XMAS appear?

use std::fs;

// better than dealing with tuples
#[derive(Clone, Copy)]
enum Moves {
    //standards
    Up,
    Down,
    Left,
    Right,
    //diags
    Uleft,
    Uright,
    Dleft,
    Dright,
}

impl Moves {
    fn direction(&self) -> (i32, i32) {
        match self {
            Moves::Up => (-1, 0),
            Moves::Down => (1, 0),
            Moves::Left => (0, -1),
            Moves::Right => (0, 1),
            Moves::Uleft => (-1, -1),
            Moves::Uright => (-1, 1),
            Moves::Dleft => (1, -1),
            Moves::Dright => (1, 1),
        }
    }
}

fn finder(
    // working matrix
    matrix: &[Vec<char>],
    // mark whats used
    seen: &mut [Vec<bool>],
    // XMAS word
    target_word: &str,
    // row and col
    row: usize,
    col: usize,
    // which way to move
    direction: Moves,
) -> bool {
    let (row_direction, col_direction) = direction.direction();
    //vec are usize so typecast here
    let mut row = row as i32;
    let mut col = col as i32;

    for letter in target_word.chars() {
        // out of bounds: top
        if row < 0
            // out of bounds: left
            || col < 0
            // out of bounds: bottom
            || row >= matrix.len() as i32
            // out of bounds: right
            || col >= matrix[0].len() as i32
            // not the letter
            || matrix[row as usize][col as usize] != letter
        {
            // if  any of the above
            return false;
        }

        //seen character
        seen[row as usize][col as usize] = true;

        // move
        row += row_direction;
        col += col_direction;
    }
    // yep its it
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Starting to get better at this input shizzz
    let file = "input.txt";
    let input = fs::read_to_string(file).unwrap();

    // word to find
    let target_word = "XMAS";
    let mut word_counter = 0;

    // my matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();

    // push each row
    for line in input.lines() {
        // char vec will simplify this
        let matrix_row: Vec<char> = line.chars().collect();
        matrix.push(matrix_row);
    }

    // Starting point
    for row in &matrix {
        println!("{:?}", row);
    }
    println!();

    // use the enum
    let directions = [
        Moves::Up,
        Moves::Down,
        Moves::Left,
        Moves::Right,
        Moves::Uleft,
        Moves::Uright,
        Moves::Dleft,
        Moves::Dright,
    ];

    let mut seen = vec![vec![false; matrix[0].len()]; matrix.len()];

    //stolen from day2, i j this bissh
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            for &direct in &directions {
                if finder(&matrix, &mut seen, target_word, i, j, direct) {
                    word_counter += 1;
                }
            }
        }
    }

    //reloop2st and replace with period
    // was easier than doing in above
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if !seen[i][j] {
                matrix[i][j] = '.'
            }
        }
    }

    // reprint with removals
    for row in &matrix {
        println!("{:?}", row);
    }

    // print my counter
    println!("XMAS count: {}", word_counter);

    // Complained otherwise
    // from the unwrap of loading the file I believe
    Ok(())
}
