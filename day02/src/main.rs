use std::any::Any;
use std::fs::File;
use std::io::Read;
use std::iter;

fn is_safe_diff(lvl1 : i32, lvl2 : i32) -> bool
{
    return 1 <= lvl1.abs_diff(lvl2) && lvl1.abs_diff(lvl2) <= 3;
}

fn are_levels_safe(report : &Vec<i32>) -> bool
{
    return report
        .windows(2)
        .map(|(w)| (w[0], w[1]))
        .all(|(v1, v2)| 
            is_safe_diff(v1, v2)
        );
}

fn is_report_safe(report : &Vec<i32>) -> bool
{
    let all_increasing = report.is_sorted();
    let all_decreasing = report.iter().rev().is_sorted();

    return (all_increasing || all_decreasing) && are_levels_safe(report);
}

fn parse_reports(input_str : String) -> Vec<Vec<i32>>
{
    return input_str
    .lines()
    .map(|line| 
        line
        .split_whitespace()
        .map(|number_str| number_str.parse::<i32>().unwrap())
        .collect()
    )
    .collect::<Vec<Vec<i32>>>();
}


fn part1(input_str : String, max_unsafe_levels : i32)
{
    let reports = parse_reports(input_str);

    let mut num_safe = 0;

    for report in reports.iter()
    {
        println!("report: {:?}", report.iter());

        let mut is_safe = is_report_safe(report);

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
    let reports = parse_reports(input_str);

    let mut num_safe = 0;

    for report in reports.iter()
    {
        println!("report: {:?}", report.iter());

        let mut is_safe = is_report_safe(report);

        if !is_safe
        {
             for i in 0..report.len()
             {
                 let mut alt_report = report.clone();
                 alt_report.remove(i);
    
                 is_safe = is_report_safe(&alt_report);
    
                 if is_safe
                 {
                     break;
                 }
             }
        }

        println!("safe: {:?}", is_safe);

        if is_safe
        {
            num_safe += 1;
        }
    }

    println!("num_safe: {:?}", num_safe);

}



fn main() {
    //let input_filename = "testinput.txt";
    let input_filename = "input.txt";

    let mut input_file = File::open(input_filename).unwrap();
    let mut input_str: String = Default::default();
    input_file.read_to_string(&mut input_str);

    //part1(input_str, 0);
    part2(input_str);
}
