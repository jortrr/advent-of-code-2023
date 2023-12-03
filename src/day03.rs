fn main() {
    println!("Hello, World! from src/day03.rs!");
    //let tuple: (u32, u32, u32) = [1, 2, 3].into_iter().map(|x| x * 2).collect();

    //dbg!(tuple);

    let x = ["Hel2lo", "World2"]
        .iter()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<String>())
        .collect::<Vec<String>>();
    dbg!(x);
}
