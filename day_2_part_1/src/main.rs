use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
    let input_string: String = read_to_string("input.txt")?;

    let mut data: Vec<usize> = input_string
        .split(",")
        .map(|v| v.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    data[1] = 12;
    data[2] = 2;

    let mut cursor: usize = 0;
    let mut opcode: usize;
    let mut src_a: usize;
    let mut src_b: usize;
    let mut dest: usize;
    let mut a: usize;
    let mut b: usize;

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

    println!("{}", data[0]);
    return Ok(());
}
