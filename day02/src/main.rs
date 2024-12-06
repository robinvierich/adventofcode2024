use std::fs::File;
use std::io::Read;





fn part1(input_str : String)
{

    let reports = 
        input_str
        .lines()
        .map(|line| 
            line
            .split_whitespace()
            .map(|number_str| number_str.parse::<i32>().unwrap())
            .collect()
        )
        .collect::<Vec<Vec<i32>>>();

    let mut num_safe = 0;

    for report in reports
    {
        println!("report: {:?}", report.iter());

        let all_increasing = report.is_sorted();
        let all_decreasing = report.iter().rev().is_sorted();

        let all_level_changes_safe = 
            report
            .windows(2)
            .map(|(w)| (w[0], w[1]))
            .all(|(v1, v2)| v1.abs_diff(v2) >= 1 && v1.abs_diff(v2) <= 3);

        let is_safe = (all_increasing || all_decreasing) && all_level_changes_safe;

        
        println!("all up: {:?}, down: {:?}", all_increasing, all_decreasing);
        println!("levels safe: {:?}", all_level_changes_safe);
        println!("safe: {:?}", is_safe);

        if is_safe
        {
            num_safe += 1;
        }
    }

    println!("num_safe: {:?}", num_safe);
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
