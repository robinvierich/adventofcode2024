use std::{fs::File, io::Read};
use regex::Regex;


fn part1(input_str : String)
{

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)");

    // for (_, (x, y)) in re.captures_iter(input_str)
    // {

    // }

}

fn part2(input_str : String)
{
}



fn main() {
    let input_filename = "testinput.txt";
    //let input_filename = "input.txt";

    let mut input_file = File::open(input_filename).unwrap();
    let mut input_str: String = Default::default();
    input_file.read_to_string(&mut input_str);

    part1(input_str);
    //part2(input_str);
}
