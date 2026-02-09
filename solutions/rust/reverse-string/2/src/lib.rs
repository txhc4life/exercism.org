pub fn reverse (input: &str) -> String {
    let reversed_string = input.chars().rev().collect::<String>();

    reversed_string
}