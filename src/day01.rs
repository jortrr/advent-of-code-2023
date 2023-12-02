fn main() {
    println!("Hello world!");
    for i in 1..=25 {
        //TODO: Your code here
        println!("[[bin]]");
        println!("name = \"day{:02}\"", i);
        println!("path = \"src/day{:02}.rs\"", i);
        println!();
    }
}
