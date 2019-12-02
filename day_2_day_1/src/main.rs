use std::fs::read_to_string;
use std::io;
fn main() -> io::Result<()> {
    // let file = File::open("input.txt")?;

    let input_string: String = read_to_string("input.txt")?;

    let mut ops: Vec<usize> = input_string
        .split(",")
        .map(|v| v.parse::<usize>())
        .filter_map(Result::ok)
        .collect();
    let mut cursor: usize = 0;

    let mut acc: usize = 0;

    loop {
        // perform opcode
        match ops[cursor] {
            1 => {
                acc = ops[cursor + 1];
                acc += ops[cursor + 2];
                ops[cursor + 3] = acc;
                cursor += 4;
            }
            2 => {
                acc = ops[cursor + 1];
                acc *= ops[cursor + 2];
                ops[cursor + 3] = acc;
                cursor += 4;
            }
            99 | _ => break,
        }

        if cursor >= ops.len() {
            break;
        }
    }

    // for op in ops {
    //     println!("{}", &op);
    // }

    //let reader = io::BufReader::new(file);

    // for line in reader.lines() {}

    return Ok(());
}
