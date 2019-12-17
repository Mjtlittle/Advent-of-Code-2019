use std::fs::read_to_string;
use std::io;

struct Program {
    source: Vec<isize>,
    data: Vec<isize>,
    pc: usize,
}

enum ProgramCommand {
    Continue,
    Halt,
}

impl Program {
    fn read_from_file(file_path: &str) -> Program {
        let input_string: String = read_to_string(file_path).expect("Error opening file");

        let source: Vec<isize> = input_string
            .split(",")
            .map(|v| v.parse::<isize>())
            .filter_map(Result::ok)
            .collect();

        Program {
            source: source,
            data: Vec::new(),
            pc: 0,
        }
    }
    fn reset(&mut self) {
        self.data = self.source.clone();
        self.pc = 0;
    }

    fn run(&mut self) {
        // reset program
        self.reset();

        // program loop
        loop {
            // check to see if end of data
            if self.pc >= self.data.len() {
                break;
            }

            // step
            let command = self.step();

            // follow command
            match command {
                ProgramCommand::Continue => (),
                ProgramCommand::Halt => break,
            }
        }
    }

    fn consume(&mut self) -> isize {
        let value = self.data[self.pc];
        self.pc += 1;
        println!("{}", value);
        return value;
    }

    fn step(&mut self) -> ProgramCommand {

        let mut value = self.consume();
        println!("\nvalue:{}", value);
        // consume the last two digits which will be the opcode
        let opcode = value % 100;
        value /= 100;

        // read in parameter modes and consume
        let mut parameter_modes: [bool; 3] = [false; 3];
        for i in 0..3 {
            parameter_modes[i] = value % 10 == 1;
            value /= 10;
        }

        // completion command
        let mut command = ProgramCommand::Continue;

        match opcode {
            // read input
            3 => {
                // get destination
                let dest = self.consume() as usize;

                // read in value
                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .expect("failed to read from stdin");
                let trimmed = input_text.trim();
                let value = trimmed.parse::<isize>().expect("Expected an integer value");
                
                // store value
                self.data[dest] = value;
            },

            // output value
            4 => {
                // get source
                let source = self.consume();

                // print value
                let value = self.data[source as usize];
                println!("{}", value.to_string());
            },

            1 => {

                // get argument a
                let mut arg_a = self.consume();
                if !parameter_modes[0] {
                    arg_a = self.data[arg_a as usize];
                }

                // get argument b
                let mut arg_b = self.consume();
                if !parameter_modes[0] {
                    arg_b = self.data[arg_b as usize];
                }

                let dest = self.consume();
                
                self.data[dest as usize] = arg_a + arg_b;
            },
            
            2 => {

                // get argument a
                let mut arg_a = self.consume();
                if !parameter_modes[0] {
                    arg_a = self.data[arg_a as usize];
                }

                // get argument b
                let mut arg_b = self.consume();
                if !parameter_modes[0] {
                    arg_b = self.data[arg_b as usize];
                }

                let dest = self.consume();
                
                self.data[dest as usize] = arg_a * arg_b;
            }

            _ => {}
        }
        println!("end\n");
        return command;
    }
}
/*
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
}*/

fn main() {
    let mut program = Program::read_from_file("input.txt");
    program.run();
}
