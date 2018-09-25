#[macro_use(print_delayed)]
extern crate shiny_cli_utils;

pub use std::{thread, time,};
pub use std::io::{self, Write};


fn main() {
    print_delayed!(std::time::Duration::from_millis(85), "Hello, world {}! What an aweeesome crazyyyy tuple... {:03}!\n");
    for i in  1 .. 10 {
        println!();
        print_delayed!(std::time::Duration::from_millis(75 - i * 5), "Hello, world {:04}! What an aweeesome crazyyyy tuple... {:?}!", i, (i + 1,i + 11));

        thread::sleep(time::Duration::from_millis(10));
    }
}
