use std::fs::read_to_string;
use std::io;

fn calculate(source_data: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut data = source_data.clone();

    let mut cursor: usize = 0;
    let mut opcode: usize;
    let mut src_a: usize;
    let mut src_b: usize;
    let mut dest: usize;
    let mut a: usize;
    let mut b: usize;

    data[1] = noun;
    data[2] = verb;

    loop {

        // reached end of program
        if cursor >= data.len() {
            break;
        }

        // interpret the opcode
        opcode = data[cursor];
        if opcode == 99 {
            break;
        }

        // get the source locations and destination
        src_a = data[cursor + 1];
        src_b = data[cursor + 2];
        dest = data[cursor + 3];

        // get the values at the sources
        a = data[src_a];
        b = data[src_b];

        // set the value at the destination to the operation detailed
        data[dest] = match data[cursor] {
            1 => a + b,
            2 => a * b,
            _ => 1337,
        };
        
        // increment cursor
        cursor += 4
    }

    data[0]

}

fn main() -> io::Result<()> {

    let input_string: String = read_to_string("input.txt")?;

    let mut data: Vec<usize> = input_string
        .split(",")
        .map(|v| v.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    let mut result: usize;
    for verb in 0..99 as usize {
        for noun in 0..99 as usize {
            result = calculate(&data, noun, verb);
            if result == 19690720 {
                println!("{:02}", noun * 100 + verb);
                return Ok(());
            }
        }
    }


    
    
    return Ok(());
}
