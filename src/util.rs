#[cfg(test)]
pub fn print_green(message: &str) {
    println!("\x1b[32m{}\x1b[0m", message);
}
