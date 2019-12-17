use std::fs::read_to_string;
use std::io;

pub struct Program {
    source: Vec<isize>,
    pub data: Vec<isize>,
    pc: usize,
}

pub enum ProgramCommand {
    Continue,
    Halt,
}

impl Program {
    pub fn new_from_file(file_path: &str) -> Program {
        let input_string: String = read_to_string(file_path).expect("Error opening file");
        Program::new_from_string(input_string.as_ref())
    }

    pub fn new_from_string(input_string: &str) -> Program {
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

    pub fn reset(&mut self) {
        self.data = self.source.clone();
        self.pc = 0;
    }

    pub fn run(&mut self) {
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
        return value;
    }

    pub fn step(&mut self) -> ProgramCommand {

        println!("{:?}", self.data);
        let mut value = self.consume();
        println!("value1:{}", value);

        // consume the last two digits which will be the opcode
        let opcode = value % 100;
        value /= 100;

        println!("value2:{}", value);

        // read in parameter modes and consume
        #[derive(Debug)]
        let mut parameter_modes: [bool; 3] = [false; 3];
        for i in 0..3 {
            parameter_modes[i] = value % 10 == 1;
            value /= 10;
        }

        println!("modes:{:?}", parameter_modes);

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
                if !parameter_modes[1] {
                    arg_b = self.data[arg_b as usize];
                }

                let dest = self.consume();
                println!("a:{} b:{} dest:{}", arg_a, arg_b, dest );                
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
                if !parameter_modes[1] {
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

    pub fn write_data(&mut self, index: usize, value: isize) {
        self.data[index] = value;
    }

    pub fn get_data(&self, index: usize) -> isize {
        return self.data[index];
    }

    pub fn print_data(&self) {
        println!("{:?}", self.data);
    }

    // pub fn get_data_value_at(&self, i: usize) -> isize {

    // }

    // pub fn get_current_data_value(&self, i:usize) -> isize {

    // }
}