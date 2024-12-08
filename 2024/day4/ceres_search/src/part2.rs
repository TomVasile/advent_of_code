// --- Part Two ---

// The Elf looks quizzically at you. Did you misunderstand the assignment?

// Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:

// M.S
// .A.
// M.S

// Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.

// Here's the same example from before, but this time all of the X-MASes have been kept instead:

// .M.S......
// ..A..MSMS.
// .M.S.MAA..
// ..A.ASMSM.
// .M.S.M....
// ..........
// S.S.S.S.S.
// .A.A.A.A..
// M.M.M.M.M.
// ..........

// In this example, an X-MAS appears 9 times.

// Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?


use std::fs;

fn find_mas_x(matrix: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut output = vec![vec!['.'; cols]; rows];
    let mut count = 0;

    println!();
    println!("===============================");
    println!("    starting MAS HUNTER3000    ");
    println!("===============================");
    println!();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if matrix[i][j] == 'A' {
                // didnt use the enum but comments for the dirs to keep my head straight
                // Moves::Uleft => (-1, -1),
                // Moves::Uright => (-1, 1),
                // Moves::Dleft => (1, -1),
                // Moves::Dright => (1, 1),
                //I duplicated this lazily for the other cases to solve this faster
                //I should totally fix this 
                // Find double MAS X
                if matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S' &&
                   matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S' {
                    // Found double MAS
                    output[i - 1][j - 1] = 'M';
                    output[i - 1][j + 1] = 'M';
                    output[i][j] = 'A';
                    output[i + 1][j - 1] = 'S';
                    output[i + 1][j + 1] = 'S';
                    count += 1;
                }
    
                // Find MASSAM X
                if matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S' &&
                   matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M' {
                    // Found MASSAM
                    output[i - 1][j - 1] = 'M';
                    output[i - 1][j + 1] = 'S';
                    output[i][j] = 'A';
                    output[i + 1][j - 1] = 'M';
                    output[i + 1][j + 1] = 'S';
                    count += 1;
                }
    
                // Find SAMMAS X
                if matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M' &&
                   matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S' {
                    // Found SAMMAS
                    output[i - 1][j - 1] = 'S';
                    output[i - 1][j + 1] = 'M';
                    output[i][j] = 'A';
                    output[i + 1][j - 1] = 'S';
                    output[i + 1][j + 1] = 'M';
                    count += 1;
                }
    
                // Find double SAM X
                if matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M' &&
                   matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M' {
                    // Found double SAM
                    output[i - 1][j - 1] = 'S';
                    output[i - 1][j + 1] = 'S';
                    output[i][j] = 'A';
                    output[i + 1][j - 1] = 'M';
                    output[i + 1][j + 1] = 'M';
                    count += 1;
                }
            }
        }
    }

    (output, count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Starting to get better at this input shizzz
    let file = "input.txt";
    let input = fs::read_to_string(file).unwrap();

    // my matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();

    // push each row
    for line in input.lines() {
        // char vec will simplify this
        let matrix_row: Vec<char> = line.chars().collect();
        println!("{:?}", matrix_row);
        matrix.push(matrix_row);
    }

    // rip out X not needed
    // I got this idea from my previous day of ripping out stuff
    // didnt need this but it did help visualize the issue so i left it
    // commented
    // for row in &mut matrix {
    //     row.iter_mut().for_each(|elem| {
    //         if *elem == 'X' {
    //             *elem = '.'; // Replace 'X' with '.'
    //         }
    //     });
    // }

    let (found_matrix, count) = find_mas_x(&matrix);

    for row in &found_matrix {
        println!("{:?}", row);
    }
    println!();
    println!("MAS count: {}", count);

    // Complained otherwise
    // from the unwrap of loading the file I believe
    Ok(())
}
