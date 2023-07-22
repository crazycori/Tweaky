use std::io::{self, Write};

mod title;
mod alc294_pulse;
mod alc294_pipewire;

fn main () {
    title::title();

    loop {
        println!("Make a Selection \n");
        println!("1. ALC294 Pipewire patch \n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                match num {
                    1 => {
                        println!("Running ALC294 Pipewire Patch");
                        alc294_pipewire::alc294_patch();
                    },
                    _ => {
                        println!("Invalid selection");
                    },
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a number.");
            },
        }

    }

}

fn menu() {

}