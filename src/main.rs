use std::env;
use std::fs;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::{thread, time};

fn main() {
    let filename = "data.txt";
    let mut sleep_seconds = 5;

    let args: Vec<String> = env::args().collect();

    match args.len() {
        // one argument passed
        2 => {
            let sleep_seconds_str = &args[1];
            // parse the number
            sleep_seconds = match sleep_seconds_str.parse() {
                Ok(n) => { n },
                Err(_) => {
                    eprintln!("error: argument not an integer");
                    return;
                },
            };
            println!("Setting sleep value to {}s", sleep_seconds);
        },
        _ => { }
    }


    loop {
        print_random_line(filename);

        thread::sleep(time::Duration::from_secs(sleep_seconds));
    }
}

fn print_random_line(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");

    let c_lines: Vec<&str> = lines.collect();

    let mut rng = rand::thread_rng();

    let n1 = rng.gen_range(0..c_lines.len());

    let chosen_line = c_lines[n1];
    println!("{}", chosen_line);

    let mut output = File::create("line.txt").expect("Can't create the line.txt file.");
    write!(output, "{}", chosen_line).expect("Can't write to the line.txt file.");
}
