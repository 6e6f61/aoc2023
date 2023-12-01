fn main() {
    let result = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(filter_digits)
        .filter_map(calculate)
        .sum::<u32>();
    
    println!("{result}");
}

fn filter_digits(from: String) -> Vec<u32> {
    from.chars()
        .filter_map(|a| a.to_digit(10))
        .collect()
}

fn calculate(from: Vec<u32>) -> Option<u32> {
    format!("{}{}", from.first()?, from.last()?).parse().ok()
}