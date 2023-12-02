# Advent of Code Fetch usage

## Context

We use the [Advent of Code Fetch] crate to fetch our Advent of Code 2023 input.

[Advent of Code Fetch]: https://docs.rs/crate/aocf/0.1.21

To use directly within Rust, the crate may be used as follows, for getting input
data for a task as a string:

```rust
use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(1))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);

    if let Ok(i) = input {
        println!("{}", i);
    }
}
```

Documentation for the `Aoc` structure can be found
[here](https://docs.rs/aocf/latest/aocf/struct.Aoc.html).

Downloaded data is cached as JSON and queried each time the `Aoc` is
initialised, to avoid unecessary requests.

The CLI has a workflow similar to Git, e.g.

```bash
aocf init
aocf get-cookie # get cookie from firefox cookie store
aocf set-cookie <your-cookie-text>
aocf checkout 1 2019
aocf checkout --now
aocf fetch
aocf brief --pretty
aocf input
aocf status
aocf submit <answer>
```

More details can be found in in the CLI's [readme](https://github.com/nuxeh/aocf/blob/master/aocf_cli/README.md).

The CLI can be used even if you don't plan to solve problems in Rust, and the
CLI and the crate can also be freely used together, so if there is a root
configuration created by the CLI, the crate will find and use this, or else
store cached data in the current directory.

[How to get your session cookie](https://github.com/nuxeh/aocf/blob/master/cookie.md). This can be as easy as logging
in to AoC with Firefox, and having the CLI extract the authentication token
automatically.

### Installation

[Install Rust](https://www.rust-lang.org/tools/install)

Install `aocf`:

- `cargo install aocf_cli`.
- `cargo install --path .` from inside a cloned repository.
- `cargo deb --install` from inside a cloned repository.

### Dependencies

Depends on development packages for `openssl`, `sqlite` dependencies are bundled.

## Commands

### Set cookie

First, check out [AoC session cookie].

[AoC session cookie]: <https://github.com/nuxeh/aocf/blob/master/cookie.md>

```bash
 aocf set-cookie <your-session-cookie>
```

## Fetch AOCF

```bash
aocf fetch
```

## Switch to a AoC day

```bash
aocf checkout 1 2019
```

## Get description for the day

```bash
aocf brief
```

## Get input for the day

```bash
aocf input
```

## Get Advent of Code current day status

```bash
aocf status
```

## Submit Advent of Code answer for the current day

```bash
aocf submit <answer>
```
