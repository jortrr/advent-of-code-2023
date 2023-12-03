pub fn get<'a>(re: &str, line: &'a str) -> Vec<regex::Captures<'a>> {
    regex::Regex::new(re).unwrap().captures_iter(line).collect::<Vec<_>>()
}
