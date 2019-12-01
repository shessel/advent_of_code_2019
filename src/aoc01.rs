fn calc_fuel(module_weights: &[u32]) -> u32 {
    module_weights.iter()
        // values < 9 would be <= 0 for (x/3) - 2
        .filter(|&&x| x > 8)
        .fold(0u32, |acc, x| acc + (x / 3) - 2)
}

fn calc_fuel_advanced(module_weights: &[u32]) -> u32 {
    module_weights.iter()
        .fold(0u32, |mut acc, x| {
            let mut fuel = *x;
            while fuel > 8 {
                fuel = (fuel / 3) - 2;
                acc += fuel;
            }
            acc
        })
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_calc_fuel_examples() {
        assert_eq!(calc_fuel(&[12]), 2);
        assert_eq!(calc_fuel(&[14]), 2);
        assert_eq!(calc_fuel(&[1969]), 654);
        assert_eq!(calc_fuel(&[100756]), 33583);
    }

    #[test]
    fn test_calc_fuel_advanced_examples() {
        assert_eq!(calc_fuel_advanced(&[12]), 2);
        assert_eq!(calc_fuel_advanced(&[14]), 2);
        assert_eq!(calc_fuel_advanced(&[1969]), 966);
        assert_eq!(calc_fuel_advanced(&[100756]), 50346);
    }

    fn parse_input_file(file_name: &str) -> Vec<u32> {
        use std::fs;
        fs::read_to_string(file_name)
            .expect(&format!("Failed to read input file \"{}\"", file_name))
            .split_whitespace()
            .map(|line| line.parse().expect("Found non-numerical input"))
            .collect()
    }

    #[test]
    fn test_calc_fuel_aoc_input() {
        let parsed_input = parse_input_file("data/input01");
        assert_eq!(calc_fuel(&parsed_input), 3278434u32);
    }

    #[test]
    fn test_calc_fuel_advanced_aoc_input() {
        let parsed_input = parse_input_file("data/input01");
        assert_eq!(calc_fuel_advanced(&parsed_input), 4914785u32);
    }
}