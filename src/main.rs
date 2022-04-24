extern crate core;

mod company;

use std::io::stdin;
use company::{model::Company, cli::Cli};

macro_rules! user_input {
    () => {{
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        input
    }}
}

fn main() {
    println!("hello to company cli, print 'Exit' to exit");
    println!("aviliable commands: {}",
             company::cli::COMMANDS.iter().fold("".to_string(), |a, b| a + b.0 + " -> " + b.1 + "\n")
    );
    let mut cmp = Company::new();
    loop {
        match user_input!().trim() {
            "" => println!(),
            "Exit" => break,
            x => {
                println!("{}", cmp.apply_input(x.to_string()));
                continue;
            }
        };
    }
}