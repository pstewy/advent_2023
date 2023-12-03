use std::fs::File;
use std::io::Read;

fn main() {
    let mut data = String::new();
    let mut f = File::open("input.txt").expect("should read file");
    f.read_to_string(&mut data).expect("should read data");
    let actual_in = data.split("\n").collect();
    println!("{}", part_one(actual_in));

    let mut data = String::new();
    let mut f = File::open("input_2.txt").expect("should read file");
    f.read_to_string(&mut data).expect("should read data");
    let actual_in = data.split("\n").collect();
    println!("{}", part_one(actual_in))
}

fn part_one(input: Vec<&str>) -> i32 {
    let mut sum = 0;
    for j in input {
        sum += sum_of_string(j.to_string());
    }
    sum
}

fn sum_of_string(input: String) -> i32 {
    if input == "" {
        return 0;
    }
    let fixed = replace_numbers(input.clone());
    println!("old: {}, new: {}", input, fixed);
    let end_index = fixed.len() - 1;
    let mut sum = "".to_string();
    let mut first = false;
    let mut last = false;
    let input_c = fixed.chars();
    let input_v = input_c.clone().collect::<Vec<_>>();
    for (i, c) in input_c.enumerate() {
        if let Some(n) = c.to_digit(10) {
            if !first {
                first = true;
                sum = format!("{}{}", n, sum);
            }
        }
        if let Some(n) = input_v[end_index - i].to_digit(10) {
            if !last {
                last = true;
                sum = format!("{}{}", sum, n);
            }
        }
    }
    sum.parse::<i32>().unwrap_or(0)
}
fn replace_numbers(input: String) -> String {
    let mut out = input.replace("one", "o1e");
    out = out.replace("two", "t2o");
    out = out.replace("three", "t3e");
    out = out.replace("four", "f4r");
    out = out.replace("five", "f5e");
    out = out.replace("six", "s6x");
    out = out.replace("seven", "s7n");
    out = out.replace("eight", "e8t");
    out.replace("nine", "n9e")
}
