use std::{fs::File, io::Read};
use regex::Regex;

fn get_mul_regex() -> Regex
{
    return Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
}


fn part1(input_str : String)
{
    let mul_regex = get_mul_regex();

    let mut sum : i64 = 0;

    for (_, [xstr, ystr]) in mul_regex.captures_iter(input_str.as_str()).map(|cap| cap.extract())
    {
        let x = xstr.parse::<i64>().unwrap();
        let y = ystr.parse::<i64>().unwrap();

        let product = x * y;
        sum += product;

        println!("{} * {} = {}", x, y, product);
    }

    println!("sum: {}", sum);
}

struct ExecutionContext
{
    is_mul_enabled: bool,
    curr_product_sum: i64
}

#[derive(Debug)]
enum InstructionType
{
    Do,
    Dont,
    Mul { operand1 : i64, operand2 : i64 }
}

#[derive(Debug)]
struct Instruction
{
    offset: usize,
    instruction_type : InstructionType,
}

impl Instruction
{
    fn execute(&self, context : &mut ExecutionContext)
    {
        match self.instruction_type
        {
            InstructionType::Do => context.is_mul_enabled = true,
            InstructionType::Dont => context.is_mul_enabled = false,

            InstructionType::Mul {operand1, operand2} 
                if context.is_mul_enabled => context.curr_product_sum += (operand1 * operand2),

            _ => ()
        }
    }
}

fn make_instr(offset: usize, instruction_type : InstructionType) -> Instruction
{
    return Instruction {
        offset,
        instruction_type
    };
}


fn part2(input_str : String)
{
    let mul_regex = get_mul_regex();

    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut mul_enabled = true;

    let do_sym = "+";
    let dont_sym = "-";

    let mut do_instrs = vec![ Instruction { offset: 0, instruction_type: InstructionType::Do } ];

    do_instrs.extend(
        do_regex
        .find_iter(&input_str)
        .map(|m| Instruction { offset: m.end(), instruction_type: InstructionType::Do })
    );

    let mut dont_instrs : Vec<Instruction> = 
        dont_regex
        .find_iter(&input_str)
        .map(|m| Instruction { offset: m.end(), instruction_type: InstructionType::Dont})
        .collect();

    dont_instrs.push(make_instr(input_str.len(), InstructionType::Dont));

    let mul_instrs = 
        mul_regex.captures_iter(&input_str)
        .map(|c| (c.get(0).unwrap(), c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()))
        .map(|(m, xstr, ystr)| Instruction {
                offset: m.end(), 
                instruction_type: InstructionType::Mul { 
                    operand1: xstr.parse::<i64>().unwrap(), 
                    operand2: ystr.parse::<i64>().unwrap() 
                }
            });

    let mut instrs: Vec<Instruction> = do_instrs.into_iter().chain(dont_instrs.into_iter()).chain(mul_instrs).collect();
    instrs.sort_by(|a, b| a.offset.cmp(&b.offset));

    for instr in instrs
    {
        println!("instr: {:?}", instr);
    }
}



fn main() {
    let input_filename = "testinput.txt";
    //let input_filename = "input.txt";

    let mut input_file = File::open(input_filename).unwrap();
    let mut input_str: String = Default::default();
    input_file.read_to_string(&mut input_str);

    //part1(input_str);
    part2(input_str);
}
