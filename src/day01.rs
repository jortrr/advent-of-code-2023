use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new().year(Some(2023)).day(Some(1)).init().unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);

    if let Ok(i) = input {
        println!("Input for day {:02}:\n", aoc.day.unwrap());
        println!("{}", i);
    }
    
    let sum = input.split("\n").collect::<Vec<&str>>().iter().map(|&line| line.chars().filter(|c|c.is_digit()).collect::<String>()).map(|&line|format!("{}{}",line.chars().next(),line.chars().last()).parse::<u32>().unwrap()).sum();
}
