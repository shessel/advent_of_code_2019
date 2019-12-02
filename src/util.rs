pub fn parse_input_file<T>(file_name: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    use std::fs;
    fs::read_to_string(file_name)
        .expect(&format!("Failed to read input file \"{}\"", file_name))
        .split_whitespace()
        .map(|line| line.parse().expect("Found non-numerical input"))
        .collect()
}

pub fn parse_input_file_separator<T>(file_name: &str, separator: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    use std::fs;
    fs::read_to_string(file_name)
        .expect(&format!("Failed to read input file \"{}\"", file_name))
        .split(separator)
        .map(|line| line.trim().parse().expect("Found non-numerical input"))
        .collect()
}
