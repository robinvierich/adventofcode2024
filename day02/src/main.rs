fn part1(input_str : String)
{

}

fn part2(input_str : String)
{

}



fn main() {
    println!("Hello, world!");

    //let input_filename = "testinput.txt";
    let input_filename = "input.txt";

    let mut input_file = File::open(input_filename).unwrap();
    let mut input_str: String = Default::default();
    input_file.read_to_string(&mut input_str);

    part1(input_str);
    //part2(input_str);
}
