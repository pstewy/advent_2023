use std::fs::File;
use std::io::Read;

fn main() {
    let mut data = String::new();
    let mut f = File::open("input.txt").expect("should read file");
    f.read_to_string(&mut data).expect("should read data");
    println!("{:?}", part_one(data))
}
// Loop over every line and store the coords (i,j) of the symbol
// Loop over every line again, and when I come across a number, see if there is a symbol nearby
// How can we store that symbol? Map of i to j? So we can do a lookup for each line of symbols?

fn part_one(input: String) -> i32 {
    let mut output = 0;
    let input = CustomMap::new(input);
    let mut current_number = String::new();
    for (i, spec) in input.specs.iter().enumerate() {
        for (j, spec_val) in spec.iter().enumerate() {
            match spec_val {
                Spec::Dot => {
                    // We are at a dot. Do we have any numbers? If so, is there a symbol adjacent to us?
                    if current_number != "" {
                        println!("looping: j: {}, len: {}", j, current_number.len());
                        for index in j-current_number.len()..=j-1 {
                            if has_adjacent((i, index), &input.symbols) {
                                output += current_number.parse::<i32>().expect("didn't create the numbers properly");
                                // Break out here regardless of where we are because we have added this number
                                break
                            }
                        }
                        current_number = String::new();
                    }
                },
                Spec::Number(n) => {
                    // Here we have a few cases.
                    // 1. This is the first number we've seen. Add it to the input
                    // 2. This is the middle number we've seen. Add it to the input
                    // 3. This is the last/only number we've seen.
                    current_number = format!("{}{}", current_number, n);
                }
                Spec::Symbol(_) => {
                    // We are at a symbol. Do we have any numbers? If so, combine them to our sum
                    if current_number!= "" {
                        output += current_number.parse::<i32>().expect("didn't create the numbers properly");
                    }
                    current_number = String::new();
                }
            }
        }
    }
    output
}

fn has_adjacent(number: (usize, usize), symbols: &Vec<(usize, usize)>) -> bool {
    let adjacent_differences = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1,1) ];

    /*
    (-1, -1), (-1, 0), (-1, +1)
     (0, -1)     x,     (0, +1) don't need to check this
     (+1, -1), (+1, 0), (+1, +1)
     */

    for s in symbols {
        let coord_diff: (i32, i32) = ((number.0 as i32 - s.0 as i32), (number.1 as i32 - s.1 as i32));
        if adjacent_differences.contains(&coord_diff) {
            return true
        }
        let coord_diff: (i32, i32) = ((number.0 as i32 - s.0 as i32).abs(), (number.1 as i32 - s.1 as i32).abs());
        if adjacent_differences.contains(&coord_diff) {
            return true
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