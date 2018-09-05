//! # shiny-cli-utils
//! Your toolbox to create nice-looking CLIs using rust.

use std::{thread, time,};
use std::io::{self, Write};


#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn it_works() {
        let now = time::Instant::now();
        let sample_text = String::from("few character string 👍");
        print_string_with_delay(
            &sample_text,
            time::Duration::from_millis(10)).expect("panicking");

        assert!(now.elapsed() >= time::Duration::from_millis(10 * sample_text.chars().count() as u64));
    }
}

/// Helps you print String `character by character`, using provided delay
///
/// # Examples
/// To print text with this effect do simply import and use it:
/// ```
/// use std::time;
/// use shiny_cli_utils::print_string_with_delay;
///
/// let now = time::Instant::now(); // PoC
///
/// let your_text = String::from("Some string");
/// print_string_with_delay(&your_text, time::Duration::from_millis(20));
///
/// let how_long_it_should_actually_take =
///     time::Duration::from_millis(10 * your_text.chars().count() as u64); // PoC
/// assert!(now.elapsed() >= how_long_it_should_actually_take); // PoC
/// ```
pub fn print_string_with_delay(word: &String, delay: time::Duration) -> io::Result<()> {
    for line in word.lines() {
        for letter in line.chars() {
            let mut string = String::new();
            string.push(letter);
            io::stdout().write(string.as_bytes())?;
            io::stdout().flush()?;
            thread::sleep(delay);
        }
        io::stdout().write(b"\n")?;
        io::stdout().flush()?;
    }

    Ok(())
}

/// Helps you print string slice `character by character`, using provided delay
///
/// # Examples
/// To print text with this effect do simply import and use it:
/// ```
/// use std::time;
/// use shiny_cli_utils::print_slice_with_delay;
///
/// let now = time::Instant::now(); // PoC
///
/// let your_text = String::from("Some string");
/// print_slice_with_delay(&your_text, time::Duration::from_millis(20));
///
/// let how_long_it_should_actually_take =
///     time::Duration::from_millis(10 * your_text.chars().count() as u64); // PoC
/// assert!(now.elapsed() >= how_long_it_should_actually_take); // PoC
/// ```
pub fn print_slice_with_delay(word: &str, delay: time::Duration) -> io::Result<()> {
    print_string_with_delay(&String::from(word), delay)
}
