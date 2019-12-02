use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_fuel_required(mass:i32) -> i32 {
    return mass / 3 - 2;
}

fn main() -> io::Result<()> {
        
    let mut sum: i32 = 0;

    // read the masses from the file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mass: i32 = line?.parse().unwrap();
        let fuel_required = get_fuel_required(mass);
        sum += fuel_required;
    }

    println!("{}", sum);
    return Ok(());
}