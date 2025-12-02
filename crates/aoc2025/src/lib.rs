// use std::path::Path;

// /// Reads from stdin if data is piped; otherwise reads from a file.
// pub fn input(path: impl AsRef<Path>) -> String {
//     let path = path.as_ref();
//     // No stdin input → fall back to file
//     std::fs::read_to_string(path)
//         .unwrap_or_default()
//         .trim()
//         .to_string()
// }

use std::io::Read as _;

pub fn input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
