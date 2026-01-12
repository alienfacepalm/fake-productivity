use rand::prelude::*;
use colored::*;

pub fn generate_matrix_code(rng: &mut ThreadRng) {
    let code_chars = "0123456789ABCDEFアイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";
    let chars_vec: Vec<char> = code_chars.chars().collect();
    let line_length = rng.gen_range(20..80);
    let line: String = (0..line_length)
        .map(|_| chars_vec.choose(rng).unwrap())
        .collect();
    
    let intensity = rng.gen_range(0..3);
    match intensity {
        0 => println!("{}", line.green().dimmed()),
        1 => println!("{}", line.green()),
        _ => println!("{}", line.bright_green().bold()),
    }
}
