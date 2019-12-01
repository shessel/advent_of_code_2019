fn calc_fuel(module_weights: &[u32]) -> u32 {
    module_weights.iter()
        // values < 9 would be <= 0 for (x/3) - 2
        .filter(|&&x| x > 8)
        .fold(0u32, |acc, x| acc + (x / 3) - 2)
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(calc_fuel(&[12]), 2);
        assert_eq!(calc_fuel(&[14]), 2);
        assert_eq!(calc_fuel(&[1969]), 654);
        assert_eq!(calc_fuel(&[100756]), 33583);
    }

    #[test]
    fn test_aoc_input() {
        use std::fs;
        let parsed_input: Vec<u32> = fs::read_to_string("data/input01")
            .expect("Failed to read input file")
            .split_whitespace()
            .map(|line| line.parse().expect("Found non-numerical input"))
            .collect();
        assert_eq!(calc_fuel(&parsed_input), 3278434u32);
    }
}