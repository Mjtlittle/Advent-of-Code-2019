use std::fs::read_to_string;
use std::io;

mod intcode;
use intcode::Program;

fn main(){

    let mut program = Program::new_from_file("inputs/day_2/input.txt");
    //program = Program::new_from_string(1,1,1,4,99,5,6,0,99);
    program.reset();
    program.write_data(1, 12);
    program.write_data(2, 2);
    program.print_data();

    program.step();
    program.print_data();

    println!("{}", program.get_data(0));

}
