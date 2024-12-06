
use std::{fs::File, io::Read, iter::zip};

use itertools::Itertools;

fn parse_left_and_right_loc_ids(input_str : String, left_loc_ids: &mut Vec<i32>, right_loc_ids: &mut Vec<i32>)
{
    let input_lines = input_str.split("\n");

    for input_line in input_lines
    {
        let (left_loc_id, right_loc_id) = 
            input_line
            .split_whitespace()
            .take(2)
            .map(|loc_id_str| loc_id_str.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        left_loc_ids.push(left_loc_id);
        right_loc_ids.push(right_loc_id);
    }
}

fn part1(input_str: String) 
{
    let mut left_loc_ids : Vec<i32> = Default::default();
    let mut right_loc_ids : Vec<i32> = Default::default();

    parse_left_and_right_loc_ids(input_str, &mut left_loc_ids, &mut right_loc_ids);

    left_loc_ids.sort();
    right_loc_ids.sort();

    let total_dist = 
        zip(left_loc_ids, right_loc_ids)
        .fold(0, |acc, (left, right)| acc + left.abs_diff(right));

    println!("part1 - total distance: {}", total_dist);
}

fn part2(input_str: String) 
{
    let mut left_loc_ids : Vec<i32> = Default::default();
    let mut right_loc_ids : Vec<i32> = Default::default();

    parse_left_and_right_loc_ids(input_str, &mut left_loc_ids, &mut right_loc_ids);

    let right_counts = right_loc_ids.iter().counts();


    let mut total_similarity = 0 as usize;

    for left_loc_id in left_loc_ids.iter()
    {
        let num_in_right = right_counts.get(&left_loc_id);

        let num= match num_in_right
        {
            None => 0 as usize,
            Some(n) => *n
        };

        let similarity = usize::try_from(*left_loc_id).unwrap() * num;
        
        total_similarity += similarity;
    }


    println!("part2 - similarity score: {}", total_similarity);
}

fn main() {

    //let input_filename = "testinput.txt";
    let input_filename = "input.txt";
    let mut input_file = File::open(input_filename).unwrap();
    let mut input_str: String = Default::default();
    input_file.read_to_string(&mut input_str);

    //part1(input_str);
    part2(input_str);


}
