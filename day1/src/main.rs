use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_calories_carried_by_elves() -> Result<Vec<u32>, String> {
    match read_lines("input.txt") {
        Err(err) => {
            let error_output = format!("Error reading file: {}", err);
            return Err(error_output);
        },
        Ok(lines) => {
            let mut calories_vector = vec![];
            let mut calories_carried = 0;
            for line in lines {
                if let Ok(calories) = line {
                    if !calories.is_empty() {
                        calories_carried += calories.parse::<u32>().unwrap_or(0);
                    }
                    else {
                        calories_vector.push(calories_carried);
                        calories_carried = 0;
                    }
                }
            }
            return Ok(calories_vector);
        }
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn max_calories_carried_by_an_elf(calories_carried: &Vec<u32>) -> Result<u32, String> {
    if calories_carried.is_empty() {
        return Err("No Elfs.".to_string());
    }
    else {
        let max_calories_carried = calories_carried
            .iter()
            .max_by(|a, b| a.cmp(b))
            .unwrap();

        Ok(*max_calories_carried)
    }
}

fn main() {
    match get_calories_carried_by_elves() {
        Ok(calories_carried_by_elves) => {
            let max_calories = max_calories_carried_by_an_elf(&calories_carried_by_elves).unwrap();
            println!("Max calories carried by a single elf = {}", max_calories);  
        },
        Err(error) => println!("{error}"),
    };

}
