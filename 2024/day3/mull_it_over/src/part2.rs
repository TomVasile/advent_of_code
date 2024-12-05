// --- Day 3: Mull It Over ---

// "Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.

// The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

// The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!

// It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.

// However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

// For example, consider the following section of corrupted memory:

// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

// Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).

// Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?


//xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

use regex::Regex;

fn remove_between_dont_and_do(input: &str) -> String {
    //match everything between don't()..do() aka disabled
    // so delete it
    let re = Regex::new(r"don't\(\)(.*?)do\(\)").unwrap();
    
    // Replace the matched portion with an empty string
    let result= re.replace_all(input, "");
    
    // Convert the result back into a String
    result.to_string()
}


fn main() {
    //let corrupted = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let corrupted = include_str!("../input.txt");

    println!("My corrupted data: {}", corrupted.trim());
    println!();

    // Normalize the data, my first attempt with don't do failed for some reason
    // I'm blaming the corrupted data so I'm going to rip all the symbols out minus (,)'
    let reggie_clean = Regex::new(r"[^a-zA-Z0-9(),']").unwrap();
    // Tom Struggled here and decided he'd go old school brute :)
    // rip out all the weirdness
    let cleaned= reggie_clean.replace_all(corrupted, "!!--!!");
    let cleaned = cleaned.replace("who", "!!--!!");
    let cleaned = cleaned.replace("what", "!!--!!");
    let cleaned = cleaned.replace("when", "!!--!!");
    let cleaned = cleaned.replace("where", "!!--!!");
    let cleaned = cleaned.replace("why", "!!--!!");
    let cleaned = cleaned.replace("how", "!!--!!");
    let cleaned = cleaned.replace("do_not_", "!!--!!");
    let cleaned = cleaned.replace("then", "!!--!!");
    let cleaned = cleaned.replace("select", "!!--!!");
    let cleaned = cleaned.replace("from", "!!--!!");


    println!("Cleaned data: {}", cleaned);
    println!();

    let take_2 = remove_between_dont_and_do(&cleaned);

    let reggie_bush = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mut uncorruptable = Vec::new();
    
    for capture in reggie_bush.captures_iter(&take_2) {
        //println!("{}", capture.get(0).unwrap().as_str());
        uncorruptable.push(capture.get(0).unwrap().as_str());
    }
    println!("My uncorruptable: {:?}", uncorruptable.concat());
    println!();

    let mut sum = 0;
    for i in uncorruptable {
        let numbers: Vec<i32> = i[4..i.len()-1]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        //println!("{}", numbers[0] * numbers[1]);
        sum += numbers[0] * numbers[1];
    }
    println!("My sum: {}", sum);

}
