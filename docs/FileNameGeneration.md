# File generation

Below are the commands I used to generate the 25 Rust binary files, and the Cargo.toml binaries.

- To create all the AdventOfCode files:

```Rust
use std::fs::File;
use std::io::Write;

fn main() {
    for i in 1..=25 {
        let filename = format!("src/day{:02}.rs", i);

        // Create and open the file
        let mut file = File::create(&filename).expect("Failed to create file");

        // Write the "Hello, World!" function to the file
        writeln!(
            &mut file,
            "fn main() {{
    println!(\"Hello, World! from {}!\"); 
}}",
            filename
        )
        .expect("Failed to write to file");

        println!("Created file: {}", filename);
    }
}
```

- In Rust, for Cargo.toml:

```Rust
fn main() {
    for i in 1..=25 {
        println!("[[bin]]");
        println!("name = \"day{:02}\"", i);
        println!("path = \"src/day{:02}.rs\"", i);
        println!();
    }
}
```
