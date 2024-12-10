use std::{fs::File, io::Read};
use regex::Regex;


fn part1(input_str : String)
{

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut sum : i64 = 0;

    for (_, [xstr, ystr]) in re.captures_iter(input_str.as_str()).map(|cap| cap.extract())
    {
        let x = xstr.parse::<i64>().unwrap();
        let y = ystr.parse::<i64>().unwrap();

        let product = x * y;
        sum += product;

        println!("{} * {} = {}", x, y, product);
    }

    println!("sum: {}", sum);
}

fn part2(input_str : String)
{
}



fn main() {
    //let input_filename = "testinput.txt";
    let input_filename = "input.txt";

    let mut input_file = File::open(input_filename).unwrap();
    let mut input_str: String = Default::default();
    input_file.read_to_string(&mut input_str);

    part1(input_str);
    //part2(input_str);
}
