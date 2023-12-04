use std::fs::File;
use std::io::Read;

fn main() {
    let mut data = String::new();
    let mut f = File::open("src/bin/day3/input.txt").expect("should read file");
    f.read_to_string(&mut data).expect("should read data");
    println!("{:?}", part_one(data))
}
// Loop over every line and store the coords (i,j) of the symbol
// Loop over every line again, and when I come across a number, see if there is a symbol nearby
// How can we store that symbol? Map of i to j? So we can do a lookup for each line of symbols?

// part two:
// Identify any number's coordinates, then loop through searching for the *. If you find a star,
// see if there are any number coordinates adjacent

fn part_one(input: String) -> i32 {
    let mut output = Vec::new();
    let input = CustomMap::new(input);
    let mut current_number = String::new();
    let mut adjacent = false;

    for (i, spec) in input.specs.iter().enumerate() {
        for (j, spec_val) in spec.iter().enumerate() {
            match spec_val {
                Spec::Dot => {
                    // We are at a dot. Do we have any numbers? If so, is there a symbol adjacent to us?
                    if current_number != "" && adjacent {
                        let num = current_number.parse::<i32>().expect("didn't create the numbers properly");
                        println!("adding: {}", num);
                        output.push(num);
                    }
                    current_number = String::new();
                    adjacent = false;
                },
                Spec::Number(n) => {
                    // Here we have a few cases.
                    // 1. This is the first number we've seen. Add it to the input
                    // 2. This is the middle number we've seen. Add it to the input
                    // 3. This is the last/only number we've seen.
                    current_number = format!("{}{}", current_number, n);
                    if has_adjacent((i, j), &input.symbols) {
                        adjacent = true;
                    }
                }
                Spec::Symbol(_) => {
                    // We are at a symbol. Do we have any numbers? If so, combine them to our sum
                    if current_number!= "" {
                        let num = current_number.parse::<i32>().expect("didn't create the numbers properly");
                        println!("adding: {}", num);
                        output.push(num);
                    }
                    current_number = String::new();
                    adjacent = false;
                }
            }
        }
    }
    output.iter().sum()
}

fn has_adjacent(number: (usize, usize), symbols: &Vec<(usize, usize)>) -> bool {
    for i in (number.0 as isize - 1)..=(number.0 as isize + 1) {
        for j in (number.1 as isize - 1)..=(number.1 as isize + 1) {
            if (i >= 0 && j >= 0) && symbols.contains(&(i as usize, j as usize)) {
                return true;
            }
        }
    }
    false
}

#[derive(Debug)]
struct CustomMap{
    specs: Vec<Vec<Spec>>,
    symbols: Vec<(usize, usize)>,
}

impl CustomMap {
    fn new(input: String) -> Self {
        let mut symbols= Vec::new();
        let cm = input
            .split("\n")
            .into_iter().enumerate()
            .map(|(i, s)| s.chars().into_iter().enumerate()
                .map(|(j, f)| {
                    let spec: Spec = f.to_string().into();
                    if let Spec::Symbol(_) = spec {
                        symbols.push((i,j));
                    }
                    spec
                }).collect()).collect();
        Self{specs: cm, symbols}
    }
}

#[derive(Debug)]
enum Spec {
    Dot,
    Number(i32),
    Symbol(String)
}

impl From<String> for Spec {
    fn from(value: String) -> Self {
        if let Ok(v) = value.parse::<i32>() {
            return Spec::Number(v)
        }
        if value == "." {
            return Spec::Dot
        }
        Spec::Symbol(value)
    }
}