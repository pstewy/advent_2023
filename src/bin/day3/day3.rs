use std::fs::File;
use std::io::Read;

fn main() {
    let mut data = String::new();
    let mut f = File::open("src/bin/day3/input.txt").expect("should read file");
    f.read_to_string(&mut data).expect("should read data");
    println!("{:?}", part_two(data))
}
// Loop over every line and store the coords (i,j) of the symbol
// Loop over every line again, and when I come across a number, see if there is a symbol nearby
// How can we store that symbol? Map of i to j? So we can do a lookup for each line of symbols?

// part two:
// Identify any number's coordinates, then loop through searching for the *. If you find a star,
// see if there are any number coordinates adjacent

fn part_two(input: String) -> i32 {
    let mut symbols: Vec<(i32, i32)> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    for (i, line) in input.split("\n").enumerate() {
        let mut num = String::new();
        for (j, c) in line.chars().enumerate() {
            // Search for symbols, if its a star then keep it. Clear the num.
            // Search for numbers. If you find one, add to the num
            if let Some(d) = c.to_digit(10) {
                num = format!("{}{}", num, d);
                // We could be at the end of the line, if so, add the number.
                if j == line.len()-1 {
                    let start_j = j as i32 - num.len() as i32 +1;
                    let end_j = j as i32;
                    numbers.push(Number{
                        value: num.parse().expect("should parse number"),
                        coords: (i as i32, (start_j..end_j).into_iter().collect()),
                    })
                }
            } else {
                if c.to_string().eq("*") {
                    symbols.push((i as i32, j as i32));
                }
                // We are at a symbol, see snag our number if we had one
                if !num.eq("") {
                    let start_j = j as i32 - num.len() as i32;
                    let end_j = j as i32;
                    numbers.push(Number{
                        value: num.parse().expect("should parse number"),
                        coords: (i as i32, (start_j..end_j).into_iter().collect()),
                    })
                }
                num = "".to_string();
            }
        }
    };

    // Iterate through the symbols. Look for adjacent numbers
    let mut output = 0;
    for (i, s) in symbols.into_iter().enumerate() {
        let adjacent_nums = find_adjacent(s, numbers.clone());
        if adjacent_nums.len() == 2 {
            let a =adjacent_nums[0].value;
            let b =adjacent_nums[1].value;
            if a < b {
                println!("Product: {} * {}", a,b)
            } else {
                println!("Product: {} * {}",b, a )
            }
            output += (adjacent_nums[0].value * adjacent_nums[1].value);
        }
    }

   output
}

#[derive(Debug, Clone)]
struct Number {
    coords: (i32, Vec<i32>),
    value: i32
}

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
    let mut adjacent_coords: Vec<(usize, usize)> = vec![
        (number.0, number.1 + 1),
        (number.0 + 1, number.1 ),
        (number.0 + 1  , number.1 + 1) ];

    if number.0 != 0 && number.1 != 0 {
        adjacent_coords.push((number.0 - 1, number.1 - 1 ));
    }
    if number.0 != 0 {
        adjacent_coords.push((number.0 - 1, number.1 ));
        adjacent_coords.push((number.0 - 1, number.1 + 1 ));
    }
    if number.1 != 0 {
        adjacent_coords.push((number.0, number.1 - 1));
        adjacent_coords.push((number.0 + 1, number.1 -1));
    }

    /*
    (-1, -1), (-1, 0), (-1, +1)
     (0, -1)     x,     (0, +1) don't need to check this
     (+1, -1), (+1, 0), (+1, +1)
     */

    for a in adjacent_coords {
        if symbols.contains(&a) {
            return true
        }
    }


    false
}

fn find_adjacent(symbol: (i32, i32), numbers: Vec<Number>) -> Vec<Number> {
    let mut adjacent_coords: Vec<(i32, i32)> = vec![
        (symbol.0, symbol.1 + 1),
        (symbol.0 + 1, symbol.1 ),
        (symbol.0 + 1  , symbol.1 + 1) ];

    if symbol.0 != 0 && symbol.1 != 0 {
        adjacent_coords.push((symbol.0 - 1, symbol.1 - 1 ));
    }
    if symbol.0 != 0 {
        adjacent_coords.push((symbol.0 - 1, symbol.1 ));
        adjacent_coords.push((symbol.0 - 1, symbol.1 + 1 ));
    }
    if symbol.1 != 0 {
        adjacent_coords.push((symbol.0, symbol.1 - 1));
        adjacent_coords.push((symbol.0 + 1, symbol.1 -1));
    }

    /*
    (-1, -1), (-1, 0), (-1, +1)
     (0, -1)     x,     (0, +1) don't need to check this
     (+1, -1), (+1, 0), (+1, +1)
     */
    let mut adjacent_numbers: Vec<Number> = Vec::new();
    for n in numbers {
        for j in n.coords.1.clone() {
           let coord = (n.coords.0.clone(), j);
           if adjacent_coords.contains(&coord) {
               adjacent_numbers.push(n.clone());
               break // break out of inner for loop, we got our number
           }
        }
    }
    adjacent_numbers
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