//! # shiny-cli-utils
//! Your toolbox to create nice-looking CLIs using rust.
#![macro_use]

pub use std::{thread, time,};
pub use std::io::{self, Write};


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
/// #[macro_use]
/// extern crate shiny_cli_utils;
/// use std::{thread, time,};
/// use std::io::{self, Write,};
///
/// fn main() {
///     let now = time::Instant::now(); // PoC
///     let your_text = String::from("Some string");
///     let how_long_it_should_actually_take =
///         time::Duration::from_millis(10 * your_text.chars().count() as u64); // PoC
///     print_delayed!(time::Duration::from_millis(20), your_text);
///
///     assert!(now.elapsed() >= how_long_it_should_actually_take); // PoC
///
///     let now = time::Instant::now(); // PoC
///     let how_long_it_should_actually_take =
///         time::Duration::from_millis(10 * 19); // PoC
///     print_delayed!(time::Duration::from_millis(20), "your_text {}", "some text");
///
///     assert!(now.elapsed() >= how_long_it_should_actually_take); // PoC
/// }

/// ```
#[macro_export]
macro_rules! print_delayed {
    ( $delay:expr, $text:expr) => {{
        let text = String::from($text);
        for letter in text.chars() {
            print!("{}", letter);
            io::stdout().flush().expect("Couldn't flush stdout!");
            thread::sleep($delay);
        }
    }};
    ( $delay:expr, $text:expr, $($attributes:tt)*) => (
        print_delayed!($delay, format!($text, $($attributes)*)));
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
#[deprecated(since = "0.1.1", note = "please use `print_delayed` macro instead")]
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
#[deprecated(since = "0.1.1", note = "please use `print_delayed` macro instead")]
pub fn print_slice_with_delay(word: &str, delay: time::Duration) -> io::Result<()> {
    print_string_with_delay(&String::from(word), delay)
}
